use std::{sync::Arc, time::Duration};

use crate::error::*;
use crate::shim::resource::{InstanceShim, ResourceShim};
use crate::util::ToPatch;
use crate::{
    app_id,
    error::Result,
    util::{wait_for_crd, K8sKeycloakBuilder},
};
use futures::StreamExt;
use k8s_openapi::{api::core::v1::Secret, ByteString};
use kube::{
    api::{ObjectMeta, Patch, PatchParams},
    runtime::{
        controller::{self, Action},
        watcher, Controller,
    },
    Api, Resource as KubeResource, ResourceExt,
};
use log::{debug, info, warn};
use rustcloak_crd::keycloak::CredentialRepresentation;
use rustcloak_crd::traits::SecretKeyNames;
use rustcloak_crd::KeycloakClient;

pub struct KeycloakClientSecretController {
    client: kube::Client,
}

impl KeycloakClientSecretController {
    pub fn new(client: &kube::Client) -> Self {
        let client = client.clone();
        KeycloakClientSecretController { client }
    }

    pub async fn run(self) -> Result<()> {
        let api = Api::<KeycloakClient>::all(self.client.clone());
        let config = controller::Config::default().concurrency(2);
        let kind = KeycloakClient::kind(&());

        wait_for_crd::<KeycloakClient, Self>(&self.client).await?;

        info!(kind = kind; "starting secret controller");

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
                            kind = kind,
                            namespace = o.namespace.unwrap(),
                            name = o.name;
                            "reconciled secret",
                        )
                    }
                    Err(controller::Error::ReconcilerFailed(e, o)) => {
                        warn!(
                            kind = kind,
                            namespace = o.namespace.unwrap(),
                            name = o.name;
                            "{e}",
                        )
                    }
                    Err(e) => warn!(kind = kind; "{e}"),
                }
            })
            .await;
        Ok(())
    }

    async fn reconcile(
        resource: Arc<KeycloakClient>,
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
        resource: Arc<KeycloakClient>,
        ctx: Arc<Self>,
    ) -> Result<Action> {
        let resource = ResourceShim::new(&resource, &ctx.client);
        let Ok(resource_path) = resource.resource_path() else {
            return Ok(Action::await_change());
        };
        let Some(secret_ref) = resource.spec.client_secret.clone() else {
            return Ok(Action::await_change());
        };
        if resource.status.as_ref().is_none_or(|x| !x.ready) {
            return Ok(Action::await_change());
        }
        let client = &ctx.client;
        let ns = resource.namespace()?;

        let secret_api: Api<Secret> = Api::namespaced(client.clone(), ns);
        let [client_id_key, client_secret_key] =
            resource.spec.client_secret.secret_key_names();

        let path = format!("{}/client-secret", resource_path);

        let instance = &resource.instance().await?;
        let keycloak = K8sKeycloakBuilder::new(instance, client)
            .with_token()
            .await?;

        debug!("fetching client secret from {}", path);
        let credential =
            keycloak.get::<CredentialRepresentation>(&path).await?;

        match credential.type_.as_deref() {
            Some("secret") => (),
            None => Err(Error::UnknownSecretType("<missing>".to_string()))?,
            Some(x) => Err(Error::UnknownSecretType(x.to_string()))?,
        }

        let Some(client_secret) = credential.value else {
            // if there is no secret present in the response, let's assume that the client has a
            // secret-less flow.
            debug!("client has no secret, skipping secret creation");
            return Ok(Action::await_change());
        };
        let client_id = resource
            .spec
            .definition
            .client_id
            .clone()
            .ok_or(Error::NoClientId)?;

        let data = [
            (
                client_id_key.to_string(),
                ByteString(client_id.into_bytes()),
            ),
            (
                client_secret_key.to_string(),
                ByteString(client_secret.into_bytes()),
            ),
        ]
        .into();
        let owner_ref = resource.owner_ref(&()).unwrap();
        let secret = Secret {
            data: Some(data),
            metadata: ObjectMeta {
                name: Some(secret_ref.secret_name),
                namespace: Some(ns.to_string()),
                owner_references: Some(vec![owner_ref]),
                ..Default::default()
            },
            type_: Some("Opaque".to_string()),
            ..Default::default()
        };

        secret_api
            .patch(
                &secret.name_unchecked(),
                &PatchParams::apply(app_id!()),
                &Patch::Apply(secret),
            )
            .await?;

        Ok(Action::await_change())
    }

    async fn handle_error(
        ctx: Arc<Self>,
        resource: &Arc<KeycloakClient>,
        e: &Error,
    ) -> Result<()> {
        let ns = resource
            .meta()
            .namespace
            .as_deref()
            .ok_or(Error::NoNamespace)?;
        let name = resource.name_unchecked();
        let api: Api<KeycloakClient> = Api::namespaced(ctx.client.clone(), ns);

        api.patch_status(&name, &PatchParams::apply(app_id!()), &e.to_patch())
            .await?;

        Ok(())
    }

    fn error_policy(
        _resource: Arc<KeycloakClient>,
        _error: &Error,
        _ctx: Arc<Self>,
    ) -> Action {
        Action::requeue(Duration::from_secs(5))
    }
}
