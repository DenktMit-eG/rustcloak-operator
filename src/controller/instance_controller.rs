use std::sync::Arc;

use crate::{
    app_id,
    crd::KeycloakApiStatus,
    error::{Error, Result},
    util::{K8sKeycloakRefreshManager, RefWatcher},
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
    secret_refs: Arc<RefWatcher<KeycloakInstance, Secret>>,
}

#[async_trait]
impl LifecycleController for KeycloakInstanceController {
    type Resource = KeycloakInstance;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource> {
        let secret_refs = self.secret_refs.clone();
        let secret_api = Api::<Secret>::all(client.clone());
        controller
            .owns(secret_api.clone(), watcher::Config::default())
            .watches(secret_api, watcher::Config::default(), move |secret| {
                secret_refs.watch(&secret)
            })
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let api = Api::<Self::Resource>::namespaced(client.clone(), &ns);

        self.manager
            .schedule_refresh(&resource, client.clone())
            .await?;

        api.patch_status(
            &resource.name_unchecked(),
            &PatchParams::apply(app_id!()),
            &KeycloakApiStatus::ok("Authenticated").into(),
        )
        .await?;

        self.secret_refs
            .add(&resource, [resource.credential_secret_name()]);

        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        _client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        self.manager.cancel_refresh(&resource).await?;
        self.secret_refs.remove(&resource);
        Ok(Action::await_change())
    }
}
