use super::super::controller_runner::LifecycleController;
use super::find_name;
use crate::app_id;
use crate::crd::{KeycloakUserSecretReference, KeycloakUserSpec};
use crate::error::Error;
use crate::{crd::KeycloakUser, error::Result};
use async_trait::async_trait;
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::ByteString;
use keycloak_crd::{KeycloakRealm as LegacyRealm, KeycloakUser as LegacyUser};
use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::runtime::watcher;
use kube::{
    runtime::{controller::Action, Controller},
    Api,
};
use kube::{Resource, ResourceExt};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct LegacyUserController {}

async fn make_secret(
    client: &kube::Client,
    resource: &mut LegacyUser,
) -> Result<Option<KeycloakUserSecretReference>> {
    let Some(credentials) = resource
        .spec
        .user
        .credentials
        .as_ref()
        .and_then(|x| x.get(0).cloned())
    else {
        return Ok(None);
    };
    if credentials.r#type.unwrap_or_default() != "password" {
        return Ok(None);
    }

    let name = resource.name_unchecked();
    let namespace = resource.namespace().ok_or(Error::NoNamespace)?;
    let owner_ref = resource.owner_ref(&()).unwrap();
    let api = Api::<Secret>::namespaced(client.clone(), &namespace);
    let Some(username) = resource.spec.user.username.clone() else {
        return Ok(None);
    };
    let Some(password) = credentials.value.clone() else {
        return Ok(None);
    };
    let name = format!("user-cred-{}", name);
    let username_key = "username".to_string();
    let password_key = "password".to_string();

    let secret = Secret {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            owner_references: Some(vec![owner_ref.clone()]),
            ..Default::default()
        },
        data: Some(
            [
                (username_key.clone(), ByteString(username.into_bytes())),
                (password_key.clone(), ByteString(password.into_bytes())),
            ]
            .into(),
        ),
        ..Default::default()
    };
    api.patch(
        &secret.name_unchecked(),
        &PatchParams::apply(app_id!()),
        &Patch::Apply(secret),
    )
    .await?;
    resource.spec.user.credentials = None;
    Ok(Some(KeycloakUserSecretReference {
        secret_name: name,
        username_key: Some(username_key),
        password_key: Some(password_key),
    }))
}

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
        let owner_ref = resource.owner_ref(&()).unwrap();
        let api = Api::<KeycloakUser>::namespaced(client.clone(), &namespace);
        let mut resource = Arc::unwrap_or_clone(resource);
        let user_secret = make_secret(client, &mut resource).await?;

        let definition = serde_json::to_value(&resource.spec.user)?;
        let instance = KeycloakUser {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: Some(namespace.clone()),
                owner_references: Some(vec![owner_ref]),
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
                user_secret,
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
