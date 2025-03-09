use std::{sync::Arc, time::Duration};

use crate::{
    app_id,
    error::Result,
    util::{ApiExt, ApiFactory, FromError, wait_for_crd},
};
use async_trait::async_trait;
use k8s_openapi::serde_json::json;
use kube::{
    Api, Resource as KubeResource, ResourceExt,
    api::{Patch, PatchParams},
    core::object::HasStatus,
    runtime::{
        Controller,
        controller::{self, Action},
        watcher,
    },
};
use log::{debug, info, warn};
use serde::{Serialize, de::DeserializeOwned};

use crate::error::*;
use futures::StreamExt;
use kube::runtime::finalizer::{Error as FinalizerError, Event, finalizer};
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
    ) -> Result<bool> {
        Ok(true)
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
    C::Resource: KubeResource<DynamicType = ()>
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
    ApiExt<C::Resource>: ApiFactory<Resource = C::Resource>,
{
    pub fn new(controller: C, client: &kube::Client) -> Self {
        let client = client.clone();
        ControllerRunner { controller, client }
    }

    pub async fn run(self) -> Result<()> {
        let api = Api::<C::Resource>::all(self.client.clone());
        let config = controller::Config::default().concurrency(2);
        let kind = C::Resource::kind(&());

        wait_for_crd::<C::Resource, C>(&self.client).await?;

        info!(target: C::MODULE_PATH, kind = kind; "starting controller for {kind}");

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
                            namespace = o.namespace,
                            name = o.name;
                            "reconciled"
                        )
                    }
                    Err(controller::Error::ReconcilerFailed(e, o)) => {
                        warn!(target: C::MODULE_PATH,
                            namespace = o.namespace,
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
        let ns = resource.namespace();
        let name = resource.name_unchecked();
        let api = ApiExt::<C::Resource>::api(ctx.client.clone(), &ns);
        let client = ctx.client.clone();
        let kind = C::Resource::kind(&());

        debug!(
            kind = kind,
            namespace = ns,
            name = name;
            "start reconciling"
        );

        match ctx
            .controller
            .before_finalizer(&client, resource.clone())
            .await
        {
            Ok(true) => {
                debug!(
                    kind = kind,
                    namespace = ns,
                    name = name;
                    "handling resource, before_finalizer returned true"
                )
            }
            Ok(false) => {
                debug!(
                    kind = kind,
                    namespace = ns,
                    name = name;
                    "skipping resource, before_finalizer returned false"
                );
                return Ok(Action::await_change());
            }
            Err(e) => {
                Self::handle_error(ctx.clone(), &resource, &e).await?;
                return Err(e);
            }
        }

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
        let ns = resource.namespace();
        let name = resource.name_unchecked();
        let api: Api<C::Resource> = ApiExt::api(ctx.client.clone(), &ns);
        let status = <C::Resource as HasStatus>::Status::from_error(e);
        log::error!(
            kind = C::Resource::kind(&()),
            namespace = resource.namespace().unwrap_or_default(),
            name = name;
            "error: {e}"
        );
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
