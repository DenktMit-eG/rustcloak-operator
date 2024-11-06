use std::{sync::Arc, time::Duration};

use crate::error::*;
use crate::util::RefWatcher;
use crate::{
    app_id,
    crd::{KeycloakApiStatus, KeycloakUser},
    error::Result,
    util::{FromError, K8sKeycloakBuilder},
};
use futures::StreamExt;
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
use reqwest::Method;
use up_impl::Container;
use up_impl::Up;

pub struct KeycloakUserSecretController {
    client: kube::Client,
    secret_refs: Arc<RefWatcher<KeycloakUser, Secret>>,
}

impl KeycloakUserSecretController {
    pub fn new(client: &kube::Client) -> Self {
        let client = client.clone();
        KeycloakUserSecretController {
            client,
            secret_refs: Arc::new(RefWatcher::default()),
        }
    }

    pub async fn run(self) -> Result<()> {
        let api = Api::<KeycloakUser>::all(self.client.clone());
        let config = controller::Config::default().concurrency(2);

        info!("starting secret controller for KeycloakUser");
        let secret_api = Api::<Secret>::all(self.client.clone());
        let secret_refs = self.secret_refs.clone();
        Controller::new(api, watcher::Config::default())
            .with_config(config)
            .shutdown_on_signal()
            .owns(secret_api.clone(), watcher::Config::default())
            .watches(secret_api, watcher::Config::default(), move |secret| {
                secret_refs.watch(&secret)
            })
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
        let Some(resource_path) = resource
            .status
            .as_ref()
            .and_then(|x| x.resource_path.clone())
        else {
            return Ok(Action::await_change());
        };
        let Some(secret_ref) = resource.spec.user_secret.clone() else {
            return Ok(Action::await_change());
        };
        let client = &ctx.client;
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;

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

        let secret_name = &secret_ref.secret_name;
        let instance = &resource.up.up;
        let keycloak = K8sKeycloakBuilder::new(instance, client)
            .with_token()
            .await?;

        ctx.secret_refs.add(&resource, [&secret_name]);
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
                        name: Some(secret_name.to_string()),
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

        let path = format!("{}/reset-password", resource_path);
        let credential = CredentialRepresentation {
            temporary: Some(false),
            value: Some(password),
            type_: Some("password".to_string()),
            ..Default::default()
        };

        keycloak
            .request(Method::PUT, &path)
            .json(&credential)
            .send()
            .await?
            .error_for_status()?;

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
        let patch = KeycloakApiStatus::from_error(e).into();

        ctx.secret_refs.remove(resource);
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
