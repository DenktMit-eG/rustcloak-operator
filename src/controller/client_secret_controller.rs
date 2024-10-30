use std::{sync::Arc, time::Duration};

use crate::error::*;
use crate::{
    app_id,
    crd::{KeycloakApiStatus, KeycloakClient},
    endpoint::path::Path,
    error::Result,
    util::K8sKeycloakBuilder,
};
use futures::StreamExt;
use http::Method;
use k8s_openapi::{api::core::v1::Secret, ByteString};
use keycloak::types::CredentialRepresentation;
use kube::{
    api::{ObjectMeta, Patch, PatchParams},
    runtime::{
        controller::{self, Action},
        watcher, Controller,
    },
    Api, Resource as KubeResource, ResourceExt,
};
use log::{error, info};
use up_impl::Container;
use up_impl::Up;

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

        info!("starting secret controller for KeycloakClient");

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
                            "reconciled secret for KeycloakClient {ns}/{name}",
                            ns = o.namespace.unwrap(),
                            name = o.name
                        )
                    }
                    Err(controller::Error::ReconcilerFailed(e, o)) => {
                        error!(
                            "KeycloakClient {ns}/{name}: {e}",
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
        let client = &ctx.client;
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let secret_ref =
            resource.spec.client_secret.clone().unwrap_or_default();

        let resource = Arc::unwrap_or_clone(resource);
        let resource = Up::with((client.clone(), ns.clone()), resource).await?;

        let secret_api: Api<Secret> = Api::namespaced(client.clone(), &ns);
        let client_id_key = secret_ref
            .client_id_key
            .clone()
            .unwrap_or("client_id".to_string());
        let client_secret_key = secret_ref
            .client_secret_key
            .clone()
            .unwrap_or("client_secret".to_string());

        let path = format!("{}/client-secret", resource.path());

        let instance = &resource.up.up;
        let keycloak = K8sKeycloakBuilder::new(instance, client)
            .with_token()
            .await?;

        let credential = keycloak
            .request(Method::GET, &path)
            .send()
            .await?
            .error_for_status()?
            .json::<CredentialRepresentation>()
            .await?;

        match credential.type_.as_deref() {
            Some("secret") => (),
            None => Err(Error::UnknownSecretType("<missing>".to_string()))?,
            Some(x) => Err(Error::UnknownSecretType(x.to_string()))?,
        }

        let client_secret = credential.value.ok_or(Error::NoClientSecret)?;
        let client_id = resource
            .spec
            .definition
            .client_id
            .clone()
            .ok_or(Error::NoClientId)?;

        let data = [
            (client_id_key, ByteString(client_id.into_bytes())),
            (client_secret_key, ByteString(client_secret.into_bytes())),
        ]
        .into();
        let owner_ref = resource.owner_ref(&()).unwrap();
        let secret = Secret {
            data: Some(data),
            metadata: ObjectMeta {
                name: Some(
                    secret_ref.secret_name.unwrap_or(resource.name_unchecked()),
                ),
                namespace: Some(ns),
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
        let patch = KeycloakApiStatus::from(e).into();

        api.patch_status(&name, &PatchParams::apply(app_id!()), &patch)
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
