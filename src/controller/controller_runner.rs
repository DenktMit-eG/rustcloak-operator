use std::{sync::Arc, time::Duration};

use crate::{app_id, crd::KeycloakApiStatus, error::Result};
use async_trait::async_trait;
use k8s_openapi::NamespaceResourceScope;
use kube::{
    api::PatchParams,
    core::object::HasStatus,
    runtime::{
        controller::{self, Action},
        watcher, Controller,
    },
    Api, Resource as KubeResource, ResourceExt,
};
use log::{error, info};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::*;
use futures::StreamExt;
use kube::runtime::finalizer::{finalizer, Error as FinalizerError, Event};
use std::fmt::Debug;
use std::hash::Hash;

#[async_trait]
pub trait LifecycleController {
    type Resource: Clone + KubeResource + Debug + 'static;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource>
    where
        <Self::Resource as kube::Resource>::DynamicType: Eq + Hash + From<()>;

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
        + HasStatus<Status = KeycloakApiStatus>
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

        let controller = Controller::new(api, watcher::Config::default())
            .with_config(config)
            .shutdown_on_signal();
        let controller = self.controller.prepare(controller, &self.client);
        controller
            .run(Self::reconcile, Self::error_policy, Arc::new(self))
            .for_each(|res| async {
                match res {
                    Ok((o, _)) => {
                        let ns = o.namespace.unwrap();
                        info!("reconciled {kind} {ns}/{}", o.name)
                    }
                    Err(e) => error!("reconcile error: {:?}", e),
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

        info!("start reconciling {kind} {}/{}", ns, name);

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
        let patch = KeycloakApiStatus::from(e).into();

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
