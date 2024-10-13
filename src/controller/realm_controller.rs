use std::{collections::HashMap, sync::Arc};

use crate::{
    crd::{KeycloakAdminApi, KeycloakAdminApiSpec},
    error::{Error, Result},
};
use async_trait::async_trait;
use kube::{
    api::{ObjectMeta, Patch, PatchParams},
    runtime::{controller::Action, watcher, Controller},
    Api, Resource, ResourceExt,
};
use log::info;
use serde_json::Value;

use super::controller_runner::LifetimeController;
use crate::crd::KeycloakRealm;

#[derive(Debug, Clone)]
pub struct KeycloakRealmController {}

impl KeycloakRealmController {
    pub fn new() -> Self {
        Self {}
    }

    fn api_token_name(&self, resource: &KeycloakRealm) -> String {
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
        let name = self.api_token_name(&resource);
        let mut json = serde_json::to_value(&resource.spec.definition)?;
        if let Some(extra_yaml) = &resource.spec.extra_yaml {
            let extra_yaml: Value = serde_yaml::from_str(extra_yaml)?;
            json_patch::merge(&mut json, &extra_yaml);
        }
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
                path: "realms".to_string(),
                payload: serde_json::to_string(&json)?,
                vars: None,
            },
            status: Default::default(),
        };

        admin_api
            .patch(
                &name,
                &PatchParams::apply(env!("CARGO_PKG_NAME")),
                &Patch::Apply(api_object),
            )
            .await?;

        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        Ok(Action::await_change())
    }
}
