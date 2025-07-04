use super::{find_name, should_handle_prudent};
use crate::{
    app_id,
    controller::controller_runner::LifecycleController,
    error::Result,
    util::{ApiExt, ApiFactory},
};
use async_trait::async_trait;
use either::Either;
use k8s_openapi::{ByteString, api::core::v1::Secret, serde_json};
use keycloak_crd::KeycloakUser as LegacyUser;
use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::runtime::watcher;
use kube::{
    Api,
    runtime::{Controller, controller::Action},
};
use kube::{Resource, ResourceExt};
use rustcloak_crd::{
    realm::{KeycloakRealm, RealmRef},
    user::{KeycloakUser, KeycloakUserSecretReference, KeycloakUserSpec},
};
use std::sync::Arc;

#[derive(Debug)]
pub struct LegacyUserController {
    prudent: bool,
}

impl LegacyUserController {
    pub fn new(prudent: bool) -> Self {
        Self { prudent }
    }
}

async fn make_secret(
    client: &kube::Client,
    resource: &mut LegacyUser,
) -> Result<Option<KeycloakUserSecretReference>> {
    let Some(credentials) = resource
        .spec
        .user
        .credentials
        .as_ref()
        .and_then(|x| x.first().cloned())
    else {
        return Ok(None);
    };
    if credentials.r#type.unwrap_or_default() != "password" {
        return Ok(None);
    }

    let name = resource.name_unchecked();
    let namespace = resource.namespace();
    let owner_ref = resource.owner_ref(&()).unwrap();
    let api = ApiExt::<Secret>::api(client.clone(), &namespace);
    let Some(username) = resource.spec.user.username.clone() else {
        return Ok(None);
    };
    let Some(password) = credentials.value.clone() else {
        return Ok(None);
    };
    let name = format!("user-cred-{name}");
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
        email_key: None,
        create: None,
    }))
}

#[async_trait]
impl LifecycleController for LegacyUserController {
    type Resource = LegacyUser;
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
        let ns = resource.namespace();
        let owner_ref = resource.owner_ref(&()).unwrap();
        let api = ApiExt::<KeycloakUser>::api(client.clone(), &ns);
        let mut resource = Arc::unwrap_or_clone(resource);
        let user_secret = make_secret(client, &mut resource).await?;

        let definition = serde_json::to_value(&resource.spec.user)?;
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

        let instance = KeycloakUser {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: ns.clone(),
                owner_references: Some(vec![owner_ref]),
                labels: resource.meta().labels.clone(),
                annotations: resource.meta().annotations.clone(),
                ..Default::default()
            },
            spec: KeycloakUserSpec {
                options: None,
                parent_ref: Either::Left(parent_ref).into(),
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
