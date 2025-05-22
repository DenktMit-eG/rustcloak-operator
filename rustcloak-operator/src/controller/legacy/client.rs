use super::super::controller_runner::LifecycleController;
use super::{find_name, should_handle_prudent};
use crate::app_id;
use crate::error::Result;
use crate::util::{ApiExt, ApiFactory};
use async_trait::async_trait;
use k8s_openapi::serde_json;
use keycloak_crd::KeycloakClient as LegacyClient;
use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::runtime::watcher;
use kube::{
    Api,
    runtime::{Controller, controller::Action},
};
use kube::{Resource, ResourceExt};
use rustcloak_crd::realm::RealmRef;
use rustcloak_crd::{
    client::{
        KeycloakClient, KeycloakClientSecretReference, KeycloakClientSpec,
    },
    realm::KeycloakRealm,
};
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
        let ns = resource.namespace();
        let owner_ref = resource.owner_ref(&()).unwrap();
        let api = ApiExt::<KeycloakClient>::api(client.clone(), &ns);
        let definition = serde_json::to_value(&resource.spec.client)?;
        let parent_ref = RealmRef::from(
            find_name::<KeycloakRealm>(
                client,
                &ns,
                &resource.spec.realm_selector,
                &resource.metadata,
                "realm_ref",
            )
            .await?
            .map_either(|l| l.into(), |r| r.into()),
        );
        let instance = KeycloakClient {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: ns.clone(),
                owner_references: Some(vec![owner_ref]),
                labels: resource.meta().labels.clone(),
                annotations: resource.meta().annotations.clone(),
                ..Default::default()
            },
            spec: KeycloakClientSpec {
                parent_ref,
                options: None,
                definition: serde_json::from_value(definition)?,
                client_secret: Some(KeycloakClientSecretReference {
                    secret_name: format!("keycloak-client-secret-{}", name),
                    client_id_key: Some("CLIENT_ID".to_string()),
                    client_secret_key: Some("CLIENT_SECRET".to_string()),
                }),
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
