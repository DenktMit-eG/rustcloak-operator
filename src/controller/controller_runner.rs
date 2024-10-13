use std::{sync::Arc, time::Duration};

use crate::{
    app_id,
    crd::{KeycloakApiStatus, WithStatus},
    error::Result,
};
use async_trait::async_trait;
use k8s_openapi::NamespaceResourceScope;
use kube::{
    runtime::{
        controller::{self, Action},
        watcher, Controller,
    },
    Api, Resource as KubeResource,
};
use log::{error, info};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::*;
use futures::StreamExt;
use kube::runtime::finalizer::{finalizer, Error as FinalizerError, Event};
use std::fmt::Debug;
use std::hash::Hash;

#[async_trait]
pub trait LifetimeController {
    type Resource: Clone + KubeResource + Debug + 'static;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource>
    where
        <Self::Resource as kube::Resource>::DynamicType: Eq + Hash;

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
    C: LifetimeController + Clone + Sync + Send + 'static,
    C::Resource: KubeResource<Scope = NamespaceResourceScope>
        + Clone
        + WithStatus<KeycloakApiStatus>
        + Debug
        + 'static
        + Send
        + Sync
        + Serialize
        + DeserializeOwned
        + Debug,
    <C::Resource as KubeResource>::DynamicType:
        Default + Eq + Hash + Clone + Debug + Unpin,
{
    pub fn new(controller: C, client: &kube::Client) -> Self {
        let client = client.clone();
        ControllerRunner { controller, client }
    }

    pub async fn run(self) -> Result<()> {
        let api = Api::<C::Resource>::all(self.client.clone());
        let config = controller::Config::default().concurrency(2);

        let controller = Controller::new(api, watcher::Config::default())
            .with_config(config)
            .shutdown_on_signal();
        let controller = self.controller.prepare(controller, &self.client);
        controller
            .run(Self::reconcile, Self::error_policy, Arc::new(self))
            .for_each(|res| async move {
                match res {
                    Ok(o) => info!("reconciled {:?}", o),
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
        let api: Api<C::Resource> = Api::namespaced(ctx.client.clone(), ns);
        let client = ctx.client.clone();

        finalizer(&api, app_id!("finalizer"), resource, |event| async move {
            match event {
                Event::Apply(resource) => {
                    ctx.controller.apply(&client, resource).await
                }
                Event::Cleanup(resource) => {
                    ctx.controller.cleanup(&client, resource).await
                }
            }
        })
        .await
        .map_err(|e| match e {
            FinalizerError::ApplyFailed(e)
            | FinalizerError::CleanupFailed(e) => e,
            e => Error::Other(Box::new(e)),
        })
    }

    fn error_policy(
        resource: Arc<C::Resource>,
        _error: &Error,
        _ctx: Arc<Self>,
    ) -> Action {
        Action::requeue(Duration::from_secs(5))
    }
}
