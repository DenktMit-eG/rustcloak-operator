use std::sync::Arc;

use crate::{
    error::Result,
    util::{K8sKeycloakRefreshJob, K8sKeycloakRefreshManager},
};
use async_trait::async_trait;
use k8s_openapi::api::core::v1::Secret;
use kube::{
    runtime::{controller::Action, watcher, Controller},
    Api,
};

use super::controller_runner::LifecycleController;
use crate::crd::KeycloakInstance;

#[derive(Debug, Default)]
pub struct KeycloakInstanceController {
    manager: K8sKeycloakRefreshManager,
}

#[async_trait]
impl LifecycleController for KeycloakInstanceController {
    type Resource = KeycloakInstance;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource> {
        controller.owns(
            Api::<Secret>::all(client.clone()),
            watcher::Config::default(),
        )
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let session_handler =
            K8sKeycloakRefreshJob::new(resource.clone(), client.clone());

        self.manager.schedule_refresh(session_handler).await?;

        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let session_handler =
            K8sKeycloakRefreshJob::new(resource.clone(), client.clone());
        self.manager.cancel_refresh(session_handler).await?;
        Ok(Action::await_change())
    }
}
