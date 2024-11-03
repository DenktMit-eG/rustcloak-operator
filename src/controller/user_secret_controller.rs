use std::{sync::Arc, time::Duration};

use crate::error::*;
use crate::{
    app_id,
    crd::{KeycloakApiStatus, KeycloakUser},
    endpoint::path::Path,
    error::Result,
    util::K8sKeycloakBuilder,
};
use futures::StreamExt;
use http::Method;
use k8s_openapi::{api::core::v1::Secret, ByteString};
use keycloak::types::CredentialRepresentation;
use kube::{
    api::{ObjectMeta, PatchParams, PostParams},
    runtime::{
        controller::{self, Action},
        watcher, Controller,
    },
    Api, Resource as KubeResource, ResourceExt,
};
use log::{error, info};
use randstr::randstr;
use up_impl::Container;
use up_impl::Up;

pub struct KeycloakUserSecretController {
    client: kube::Client,
}

impl KeycloakUserSecretController {
    pub fn new(client: &kube::Client) -> Self {
        let client = client.clone();
        KeycloakUserSecretController { client }
    }

    pub async fn run(self) -> Result<()> {
        let api = Api::<KeycloakUser>::all(self.client.clone());
        let config = controller::Config::default().concurrency(2);

        info!("starting secret controller for KeycloakUser");

        Controller::new(api, watcher::Config::default())
            .with_config(config)
            .shutdown_on_signal()
            .owns(
                Api::<Secret>::all(self.client.clone()),
                watcher::Config::default(),
            )
            .run(Self::reconcile, Self::error_policy, Arc::new(self))
            .for_each(|res| async {
                match res {
                    Ok((o, _)) => {
                        info!(
                            "reconciled secret for KeycloakUser {ns}/{name}",
                            ns = o.namespace.unwrap(),
                            name = o.name
                        )
                    }
                    Err(controller::Error::ReconcilerFailed(e, o)) => {
                        error!(
                            "KeycloakUser {ns}/{name}: {e}",
                            ns = o.namespace.unwrap(),
                            name = o.name
                        )
                    }
                    Err(e) => error!("{e}"),
                }
            })
            .await;
        Ok(())
    }

    async fn reconcile(
        resource: Arc<KeycloakUser>,
        ctx: Arc<Self>,
    ) -> Result<Action> {
        match Self::reconcile_inner(resource.clone(), ctx.clone()).await {
            Ok(action) => Ok(action),
            Err(e) => {
                Self::handle_error(ctx, &resource, &e).await?;
                Err(e)
            }
        }
    }

    async fn reconcile_inner(
        resource: Arc<KeycloakUser>,
        ctx: Arc<Self>,
    ) -> Result<Action> {
        let client = &ctx.client;
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let secret_ref = resource.spec.user_secret.clone().unwrap_or_default();

        let resource = Arc::unwrap_or_clone(resource);
        let resource = Up::with((client.clone(), ns.clone()), resource).await?;

        let secret_api: Api<Secret> = Api::namespaced(client.clone(), &ns);
        let username_key = secret_ref
            .username_key
            .clone()
            .unwrap_or("username".to_string());
        let password_key = secret_ref
            .password_key
            .clone()
            .unwrap_or("password".to_string());

        let instance = &resource.up.up;
        let keycloak = K8sKeycloakBuilder::new(instance, client)
            .with_token()
            .await?;

        let secret_name =
            secret_ref.secret_name.unwrap_or(resource.name_unchecked());
        let secret =
            if let Some(secret) = secret_api.get_opt(&secret_name).await? {
                secret
            } else {
                let username = resource
                    .spec
                    .definition
                    .username
                    .clone()
                    .ok_or(Error::NoUsername)?;
                let password = randstr()
                    .must_upper()
                    .must_lower()
                    .must_digit()
                    .must_symbol()
                    .len(32)
                    .build()
                    .generate();
                let data = [
                    (username_key, ByteString(username.into_bytes())),
                    (password_key.clone(), ByteString(password.into_bytes())),
                ]
                .into();
                let owner_ref = resource.owner_ref(&()).unwrap();

                let secret = Secret {
                    data: Some(data),
                    metadata: ObjectMeta {
                        name: Some(secret_name),
                        namespace: Some(ns),
                        owner_references: Some(vec![owner_ref]),
                        ..Default::default()
                    },
                    type_: Some("Opaque".to_string()),
                    ..Default::default()
                };

                secret_api.create(&PostParams::default(), &secret).await?
            };

        let password = secret
            .data
            .and_then(|data| data.get(&password_key).cloned())
            .ok_or(Error::NoPassword)?;
        let password = String::from_utf8(password.0)?;

        let path = format!("{}/reset-password", resource.path());
        let credential = CredentialRepresentation {
            temporary: Some(false),
            value: Some(password),
            type_: Some("password".to_string()),
            ..Default::default()
        };

        keycloak
            .request(Method::POST, &path)
            .json(&credential)
            .send()
            .await?;

        Ok(Action::await_change())
    }

    async fn handle_error(
        ctx: Arc<Self>,
        resource: &Arc<KeycloakUser>,
        e: &Error,
    ) -> Result<()> {
        let ns = resource
            .meta()
            .namespace
            .as_deref()
            .ok_or(Error::NoNamespace)?;
        let name = resource.name_unchecked();
        let api: Api<KeycloakUser> = Api::namespaced(ctx.client.clone(), ns);
        let patch = KeycloakApiStatus::from(e).into();

        api.patch_status(&name, &PatchParams::apply(app_id!()), &patch)
            .await?;

        Ok(())
    }

    fn error_policy(
        _resource: Arc<KeycloakUser>,
        _error: &Error,
        _ctx: Arc<Self>,
    ) -> Action {
        Action::requeue(Duration::from_secs(5))
    }
}
