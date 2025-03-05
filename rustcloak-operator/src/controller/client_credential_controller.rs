use crate::{
    app_id,
    controller::controller_runner::LifecycleController,
    error::{Error, Result},
    util::{
        ApiExt, ApiFactory, K8sKeycloakBuilder, RefWatcher, Retrieve, Retriever,
    },
};
use async_trait::async_trait;
use either::for_both;
use k8s_openapi::{
    api::core::v1::Secret,
    serde_json::{self, json},
};
use kube::{
    Api, Resource, ResourceExt,
    api::{ObjectMeta, Patch, PatchParams},
    runtime::{Controller, controller::Action, watcher},
};
use log::debug;
use rustcloak_crd::{
    InstanceRef, KeycloakClientCredential,
    keycloak_types::{ClientRepresentation, CredentialRepresentation},
    traits::SecretKeyNames,
};
use std::sync::Arc;

#[derive(Default)]
pub struct ClientCredentialController {
    secret_refs: Arc<RefWatcher<KeycloakClientCredential, Secret>>,
}

#[async_trait]
impl LifecycleController for ClientCredentialController {
    type Resource = KeycloakClientCredential;
    const MODULE_PATH: &'static str = module_path!();

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource> {
        let secret_refs = self.secret_refs.clone();
        let secret_api = Api::<Secret>::all(client.clone());
        controller
            .owns(secret_api.clone(), watcher::Config::default())
            .watches(secret_api, watcher::Config::default(), move |secret| {
                secret_refs.watch(&secret)
            })
    }

    async fn before_finalizer(
        &self,
        _client: &kube::Client,
        _resource: Arc<Self::Resource>,
    ) -> Result<bool> {
        Ok(true)
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let resource_path = &resource.spec.resource_path;

        if resource.status.as_ref().is_none_or(|x| !x.ready) {
            return Ok(Action::await_change());
        }
        let ns = resource.namespace();

        let secret_api = ApiExt::<Secret>::api(client.clone(), &ns);
        let [client_id_key, client_secret_key] =
            resource.spec.client_secret.secret_key_names();

        let secret_path = format!("{}/client-secret", resource_path);

        let instance = Retriever::<InstanceRef>::get(
            client.clone(),
            &resource.spec.instance_ref,
            &ns,
        )
        .await?;

        let keycloak = for_both!(&instance, instance => K8sKeycloakBuilder::new(instance, client))
            .with_token()
            .await?;

        debug!("fetching client from {}", resource_path);
        let client =
            keycloak.get::<ClientRepresentation>(&resource_path).await?;
        let Some(client_id) = client.client_id else {
            return Err(Error::MissingClientId)?;
        };

        debug!("fetching client secret from {}", secret_path);
        let credential = keycloak
            .get::<CredentialRepresentation>(&secret_path)
            .await?;

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
        let owner_ref = resource.owner_ref(&()).unwrap();
        let secret = Secret {
            data: Some(serde_json::from_value(json!({
                client_id_key: client_id,
                client_secret_key: client_secret,
            }))?),
            metadata: ObjectMeta {
                name: Some(resource.spec.client_secret.secret_name.clone()),
                namespace: ns.clone(),
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

    async fn cleanup(
        &self,
        _client: &kube::Client,
        _resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        Ok(Action::await_change())
    }
}
