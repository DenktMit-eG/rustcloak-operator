use super::super::controller_runner::LifecycleController;
use super::{find_name, should_handle_prudent};
use crate::app_id;
use crate::crd::{
    KeycloakClientSecretReference, KeycloakClientSpec, KeycloakRealm,
};
use crate::error::Error;
use crate::{crd::KeycloakClient, error::Result};
use async_trait::async_trait;
use keycloak_crd::KeycloakClient as LegacyClient;
use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::runtime::watcher;
use kube::{
    runtime::{controller::Action, Controller},
    Api,
};
use kube::{Resource, ResourceExt};
use std::sync::Arc;

#[derive(Debug)]
pub struct LegacyClientController {
    prudent: bool,
}

impl LegacyClientController {
    pub fn new(prudent: bool) -> Self {
        Self { prudent }
    }
}

#[async_trait]
impl LifecycleController for LegacyClientController {
    type Resource = LegacyClient;
    const MODULE_PATH: &'static str = module_path!();

    async fn before_finalizer(
        &self,
        _client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<bool> {
        Ok(should_handle_prudent(resource.meta(), self.prudent))
    }

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource> {
        controller.owns(
            Api::<KeycloakClient>::all(client.clone()),
            watcher::Config::default(),
        )
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let name = resource.name_unchecked();
        let namespace = resource.namespace().ok_or(Error::NoNamespace)?;
        let owner_ref = resource.owner_ref(&()).unwrap();
        let api = Api::<KeycloakClient>::namespaced(client.clone(), &namespace);
        let definition = serde_json::to_value(&resource.spec.client)?;
        let instance = KeycloakClient {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: Some(namespace.clone()),
                owner_references: Some(vec![owner_ref]),
                labels: resource.meta().labels.clone(),
                annotations: resource.meta().annotations.clone(),
                ..Default::default()
            },
            spec: KeycloakClientSpec {
                options: None,
                realm_ref: find_name::<KeycloakRealm>(
                    client,
                    &namespace,
                    &resource.spec.realm_selector,
                    &resource.metadata,
                    "realm_ref",
                )
                .await?,
                definition: serde_json::from_value(definition)?,
                client_secret: Some(KeycloakClientSecretReference {
                    secret_name: format!("keycloak-client-secret-{}", name),
                    client_id_key: Some("CLIENT_ID".to_string()),
                    client_secret_key: Some("CLIENT_SECRET".to_string()),
                }),
                patches: None,
            },
            status: None,
        };
        api.patch(
            &name,
            &PatchParams::apply(app_id!()),
            &Patch::Apply(instance),
        )
        .await?;
        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        _client: &kube::Client,
        _resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        Ok(Action::await_change())
    }
}
