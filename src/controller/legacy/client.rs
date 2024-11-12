use super::super::controller_runner::LifecycleController;
use super::find_name;
use crate::app_id;
use crate::crd::{KeycloakClientSecretReference, KeycloakClientSpec};
use crate::error::Error;
use crate::{crd::KeycloakClient, error::Result};
use async_trait::async_trait;
use keycloak_crd::{
    KeycloakClient as LegacyClient, KeycloakRealm as LegacyRealm,
};
use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::runtime::watcher;
use kube::{
    runtime::{controller::Action, Controller},
    Api,
};
use kube::{Resource, ResourceExt};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct LegacyClientController {}

#[async_trait]
impl LifecycleController for LegacyClientController {
    type Resource = LegacyClient;

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
                ..Default::default()
            },
            spec: KeycloakClientSpec {
                options: None,
                realm_ref: find_name::<LegacyRealm>(
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
