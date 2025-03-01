use std::{sync::Arc, time::Duration};

use crate::error::*;
use crate::shim::resource::{InstanceShim, ResourceShim};
use crate::util::{RefWatcher, SecretUtils, ToPatch};
use crate::{
    app_id,
    error::Result,
    util::{K8sKeycloakBuilder, wait_for_crd},
};
use futures::StreamExt;
use k8s_openapi::{ByteString, api::core::v1::Secret};
use kube::{
    Api, Resource as KubeResource, ResourceExt,
    api::{ObjectMeta, PatchParams, PostParams},
    runtime::{
        Controller,
        controller::{self, Action},
        watcher,
    },
};
use log::{info, warn};
use randstr::randstr;
use rustcloak_crd::KeycloakUser;
use rustcloak_crd::keycloak_types::CredentialRepresentation;
use rustcloak_crd::traits::SecretKeyNames;

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
        let kind = KeycloakUser::kind(&());

        wait_for_crd::<KeycloakUser, Self>(&self.client).await?;

        info!(kind = kind; "starting secret controller");
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
                            namespace = o.namespace.unwrap(),
                            name = o.name,
                            kind = kind;
                            "reconciled secret",
                        )
                    }
                    Err(controller::Error::ReconcilerFailed(e, o)) => {
                        warn!(
                            namespace = o.namespace.unwrap(),
                            name = o.name,
                            kind = kind;
                            "{e}",
                        )
                    }
                    Err(e) => warn!( kind = kind; "{e}"),
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
        let resource = ResourceShim::new(&resource, &ctx.client);
        let Ok(resource_path) = resource.resource_path() else {
            return Ok(Action::await_change());
        };
        let Some(secret_ref) = resource.spec.user_secret.clone() else {
            return Ok(Action::await_change());
        };
        let client = &ctx.client;
        let ns = resource.namespace()?;

        let secret_api: Api<Secret> = Api::namespaced(client.clone(), ns);
        let [username_key, password_key] =
            resource.spec.user_secret.secret_key_names();

        let secret_name = &secret_ref.secret_name;
        let instance = resource.instance().await?;
        let keycloak = K8sKeycloakBuilder::new(&instance, client)
            .with_token()
            .await?;

        ctx.secret_refs.add(&resource, [&secret_name]);
        let secret = if let Some(secret) =
            secret_api.get_opt(secret_name).await?
        {
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
                (username_key.to_string(), ByteString(username.into_bytes())),
                (password_key.to_string(), ByteString(password.into_bytes())),
            ]
            .into();
            let owner_ref = resource.owner_ref(&()).unwrap();

            let secret = Secret {
                data: Some(data),
                metadata: ObjectMeta {
                    name: Some(secret_name.to_string()),
                    namespace: Some(ns.to_string()),
                    owner_references: Some(vec![owner_ref]),
                    ..Default::default()
                },
                type_: Some("Opaque".to_string()),
                ..Default::default()
            };

            secret_api.create(&PostParams::default(), &secret).await?
        };

        let [_, password] = secret.extract(&resource.spec.user_secret)?;

        let path = format!("{}/reset-password", resource_path);
        let credential = CredentialRepresentation {
            temporary: Some(false),
            value: Some(password),
            type_: Some("password".to_string()),
            ..Default::default()
        };

        keycloak.put(&path, &credential).await?;

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

        ctx.secret_refs.remove(resource);
        api.patch_status(&name, &PatchParams::apply(app_id!()), &e.to_patch())
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
