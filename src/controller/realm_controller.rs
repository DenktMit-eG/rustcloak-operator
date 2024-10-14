use std::sync::Arc;

use crate::{
    app_id,
    crd::{KeycloakAdminApi, KeycloakAdminApiSpec},
    error::{Error, Result},
};
use async_trait::async_trait;
use keycloak::types::RealmRepresentation;
use kube::{
    api::{ObjectMeta, Patch, PatchParams},
    runtime::{controller::Action, watcher, Controller},
    Api, Resource, ResourceExt,
};

use super::controller_runner::LifetimeController;
use crate::crd::KeycloakRealm;

#[derive(Debug, Clone)]
pub struct KeycloakRealmController {}

impl Default for KeycloakRealmController {
    fn default() -> Self {
        Self::new()
    }
}

impl KeycloakRealmController {
    pub fn new() -> Self {
        Self {}
    }

    fn realm_name(&self, resource: &KeycloakRealm) -> String {
        let name = resource.name_unchecked();
        format!("realm-{name}")
    }
}

#[async_trait]
impl LifetimeController for KeycloakRealmController {
    type Resource = KeycloakRealm;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource> {
        controller.owns(
            Api::<KeycloakAdminApi>::all(client.clone()),
            watcher::Config::default(),
        )
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let admin_api: Api<KeycloakAdminApi> =
            Api::namespaced(client.clone(), &ns);
        let realm_name = &resource.spec.definition.realm;
        let name = self.realm_name(&resource);
        let mut json = serde_json::to_value(&resource.spec.definition)?;
        if let Some(extra) = &resource.spec.extra {
            json_patch::merge(&mut json, extra);
        }
        let realm_representation: RealmRepresentation =
            serde_json::from_value(json.clone())?;
        let owner_ref = resource.controller_owner_ref(&()).unwrap();

        let api_object = KeycloakAdminApi {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: Some(ns.clone()),
                owner_references: Some(vec![owner_ref]),
                ..Default::default()
            },
            spec: KeycloakAdminApiSpec {
                api: resource.spec.api.clone(),
                path: format!("realms/{realm_name}"),
                payload: serde_json::to_value(&realm_representation)?,
                vars: None,
            },
            status: Default::default(),
        };

        admin_api
            .patch(
                &name,
                &PatchParams::apply(app_id!()),
                &Patch::Apply(api_object),
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
