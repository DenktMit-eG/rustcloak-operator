use super::super::controller_runner::LifecycleController;
use super::find_name;
use crate::app_id;
use crate::crd::KeycloakUserSpec;
use crate::error::Error;
use crate::{crd::KeycloakUser, error::Result};
use async_trait::async_trait;
use keycloak_crd::{KeycloakRealm as LegacyRealm, KeycloakUser as LegacyUser};
use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::runtime::watcher;
use kube::ResourceExt;
use kube::{
    runtime::{controller::Action, Controller},
    Api,
};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct LegacyUserController {}

#[async_trait]
impl LifecycleController for LegacyUserController {
    type Resource = LegacyUser;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource> {
        controller.owns(
            Api::<KeycloakUser>::all(client.clone()),
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
        let api = Api::<KeycloakUser>::namespaced(client.clone(), &namespace);
        let definition = serde_json::to_value(&resource.spec.user)?;
        let instance = KeycloakUser {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: Some(namespace.clone()),
                ..Default::default()
            },
            spec: KeycloakUserSpec {
                options: None,
                realm_ref: find_name::<LegacyRealm>(
                    client,
                    &namespace,
                    &resource.spec.realm_selector,
                )
                .await?,
                definition: serde_json::from_value(definition)?,
                user_secret: None,
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
