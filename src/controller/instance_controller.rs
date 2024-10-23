use std::sync::Arc;

use crate::{
    app_id,
    crd::KeycloakApiStatus,
    error::{Error, Result},
    util::{K8sKeycloakRefreshJob, K8sKeycloakRefreshManager},
};
use async_trait::async_trait;
use k8s_openapi::api::core::v1::Secret;
use kube::{
    api::PatchParams,
    runtime::{controller::Action, watcher, Controller},
    Api, ResourceExt,
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
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let api = Api::<Self::Resource>::namespaced(client.clone(), &ns);
        let session_handler =
            K8sKeycloakRefreshJob::new(resource.clone(), client.clone());

        self.manager.schedule_refresh(session_handler).await?;

        api.patch_status(
            &resource.name_unchecked(),
            &PatchParams::apply(app_id!()),
            &KeycloakApiStatus::ok("Authenticated").into(),
        )
        .await?;

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
