use std::{sync::Arc, time::Duration};

use crate::{
    app_id,
    error::Result,
    util::{wait_for_crd, FromError},
};
use async_trait::async_trait;
use k8s_openapi::NamespaceResourceScope;
use kube::{
    api::{Patch, PatchParams},
    core::object::HasStatus,
    runtime::{
        controller::{self, Action},
        watcher, Controller,
    },
    Api, Resource as KubeResource, ResourceExt,
};
use log::{debug, info, warn};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

use crate::error::*;
use futures::StreamExt;
use kube::runtime::finalizer::{finalizer, Error as FinalizerError, Event};
use std::fmt::Debug;
use std::hash::Hash;

#[async_trait]
pub trait LifecycleController {
    type Resource: Clone + KubeResource + Debug + 'static + Send + Sync;
    const MODULE_PATH: &'static str;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource>
    where
        <Self::Resource as kube::Resource>::DynamicType: Eq + Hash + From<()>;

    async fn before_finalizer(
        &self,
        _client: &kube::Client,
        _resource: Arc<Self::Resource>,
    ) -> Result<()> {
        Ok(())
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action>;
    async fn cleanup(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action>;
}

pub struct ControllerRunner<C> {
    controller: C,
    client: kube::Client,
}

impl<C> ControllerRunner<C>
where
    C: LifecycleController + Sync + Send + 'static,
    C::Resource: KubeResource<Scope = NamespaceResourceScope>
        + Clone
        + HasStatus<Status: FromError + Serialize + Send + Sync>
        + Debug
        + 'static
        + Send
        + Sync
        + Serialize
        + DeserializeOwned
        + Debug,
    <C::Resource as KubeResource>::DynamicType:
        Default + Eq + Hash + Clone + Debug + Unpin + From<()>,
{
    pub fn new(controller: C, client: &kube::Client) -> Self {
        let client = client.clone();
        ControllerRunner { controller, client }
    }

    pub async fn run(self) -> Result<()> {
        let api = Api::<C::Resource>::all(self.client.clone());
        let config = controller::Config::default().concurrency(2);
        let dt = ().into();
        let kind = C::Resource::kind(&dt);

        wait_for_crd::<C::Resource, C>(&self.client).await?;

        info!(kind = kind; "starting controller");

        let controller = Controller::new(api, watcher::Config::default())
            .with_config(config)
            .shutdown_on_signal();
        let controller = self.controller.prepare(controller, &self.client);
        controller
            .run(Self::reconcile, Self::error_policy, Arc::new(self))
            .for_each(|res| async {
                match res {
                    Ok((o, _)) => {
                        info!(target: C::MODULE_PATH,
                            kind = kind,
                            namespace = o.namespace.unwrap(),
                            name = o.name;
                            "reconciled"
                        )
                    }
                    Err(controller::Error::ReconcilerFailed(e, o)) => {
                        warn!(target: C::MODULE_PATH,
                            namespace = o.namespace.unwrap(),
                            name = o.name,
                            kind = kind;
                            "reconciling failed: {e}",
                        )
                    }
                    Err(controller::Error::QueueError(e)) => {
                        warn!(target: C::MODULE_PATH, kind = kind; "Queue Error: {}", e)
                    },
                    Err(controller::Error::RunnerError(e)) => {
                        warn!(target: C::MODULE_PATH, kind = kind; "Runner Error: {}", e)
                    },
                    Err(e) => {
                        warn!(target: C::MODULE_PATH, kind = kind; "{}", e)
                    }
                }
            })
            .await;
        Ok(())
    }

    async fn reconcile(
        resource: Arc<C::Resource>,
        ctx: Arc<Self>,
    ) -> Result<Action> {
        let ns = resource
            .meta()
            .namespace
            .as_deref()
            .ok_or(Error::NoNamespace)?;
        let name = resource.name_unchecked();
        let api: Api<C::Resource> = Api::namespaced(ctx.client.clone(), ns);
        let client = ctx.client.clone();
        let dt = ().into();
        let kind = C::Resource::kind(&dt);

        debug!(
            kind = kind,
            namespace = ns,
            name = name;
            "start reconciling"
        );

        ctx.controller
            .before_finalizer(&client, resource.clone())
            .await?;

        match finalizer(
            &api,
            app_id!("finalizer"),
            resource.clone(),
            |event| async {
                match event {
                    Event::Apply(resource) => {
                        ctx.controller.apply(&client, resource).await
                    }
                    Event::Cleanup(resource) => {
                        ctx.controller.cleanup(&client, resource).await
                    }
                }
            },
        )
        .await
        {
            Ok(action) => Ok(action),
            Err(FinalizerError::ApplyFailed(e))
            | Err(FinalizerError::CleanupFailed(e)) => {
                Self::handle_error(ctx.clone(), &resource, &e).await?;
                Err(e)
            }
            Err(e) => Err(Error::Other(Box::new(e))),
        }
    }

    async fn handle_error(
        ctx: Arc<Self>,
        resource: &C::Resource,
        e: &Error,
    ) -> Result<()> {
        let ns = resource
            .meta()
            .namespace
            .as_deref()
            .ok_or(Error::NoNamespace)?;
        let name = resource.name_unchecked();
        let api: Api<C::Resource> = Api::namespaced(ctx.client.clone(), ns);
        let status = <C::Resource as HasStatus>::Status::from_error(e);
        let patch = Patch::Merge(json!({
            "status": status,
        }));

        api.patch_status(&name, &PatchParams::apply(app_id!()), &patch)
            .await?;

        Ok(())
    }
    fn error_policy(
        _resource: Arc<C::Resource>,
        _error: &Error,
        _ctx: Arc<Self>,
    ) -> Action {
        Action::requeue(Duration::from_secs(5))
    }
}
