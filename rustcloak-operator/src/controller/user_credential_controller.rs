use crate::{
    app_id,
    controller::controller_runner::LifecycleController,
    error::{Error, Result},
    util::{
        ApiExt, ApiFactory, K8sKeycloakBuilder, RefWatcher, Retrieve,
        Retriever, SecretUtils, ToPatch,
    },
};
use async_trait::async_trait;
use either::for_both;
use k8s_openapi::{ByteString, api::core::v1::Secret};
use kube::{
    Api, ResourceExt,
    api::{ObjectMeta, PatchParams, PostParams},
    runtime::{Controller, controller::Action, watcher},
};
use randstr::randstr;
use rustcloak_crd::{
    InstanceRef, KeycloakApiStatus, KeycloakUserCredential,
    keycloak_types::{CredentialRepresentation, UserRepresentation},
    traits::SecretKeyNames,
};
use std::sync::Arc;

#[derive(Default)]
pub struct UserCredentialController {
    secret_refs: Arc<RefWatcher<KeycloakUserCredential, Secret>>,
}

#[async_trait]
impl LifecycleController for UserCredentialController {
    type Resource = KeycloakUserCredential;
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
        let ns = resource.namespace();
        let name = resource.name_unchecked();

        let api = ApiExt::<Self::Resource>::api(client.clone(), &ns);
        let secret_api = ApiExt::<Secret>::api(client.clone(), &ns);
        let [username_key, password_key, email_key] =
            resource.spec.user_secret.secret_key_names();

        let resource_path = &resource.spec.resource_path;
        let secret_name = &resource.spec.user_secret.secret_name;
        let instance = Retriever::<InstanceRef>::get(
            client.clone(),
            &resource.spec.instance_ref,
            &ns,
        )
        .await?;
        let keycloak = for_both!(&instance, instance => K8sKeycloakBuilder::new(instance, client))
            .with_token()
            .await?;

        self.secret_refs.add(&resource, [&secret_name]);

        // If we don't allow the creation of new secrets, we fail if the secret
        // doesn't exist. Doing it this way returns the actual kube.rs error.
        let secret = if resource.spec.user_secret.create.unwrap_or(true) {
            secret_api.get_opt(secret_name).await?
        } else {
            Some(secret_api.get(secret_name).await?)
        };

        let password = if let Some(secret) = secret {
            if let [_, Some(password), _] =
                secret.extract_opt(&Some(resource.spec.user_secret.clone()))
            {
                password
            } else {
                let [_, key, _] = resource.spec.user_secret.secret_key_names();
                return Err(Error::MissingField(key.to_string()));
            }
        } else {
            let user =
                keycloak.get::<UserRepresentation>(resource_path).await?;

            let username = user.username.ok_or(Error::NoUsername)?;
            let email = user.email;
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
                (
                    password_key.to_string(),
                    ByteString(password.clone().into_bytes()),
                ),
                (
                    email_key.to_string(),
                    ByteString(email.unwrap_or_default().into_bytes()),
                ),
            ]
            .into();

            let secret = Secret {
                data: Some(data),
                metadata: ObjectMeta {
                    name: Some(secret_name.to_string()),
                    namespace: ns.clone(),
                    labels: Some(
                        [(
                            app_id!("autoCreated").to_string(),
                            "true".to_string(),
                        )]
                        .into(),
                    ),
                    ..Default::default()
                },
                type_: Some("Opaque".to_string()),
                ..Default::default()
            };

            secret_api.create(&PostParams::default(), &secret).await?;
            password
        };

        let path = format!("{}/reset-password", resource_path);
        let credential = CredentialRepresentation {
            temporary: Some(false),
            value: Some(password),
            type_: Some("password".to_string()),
            ..Default::default()
        };

        keycloak.put(&path, &credential).await?;

        let status = KeycloakApiStatus::ok("Applied");

        api.patch_status(
            &name,
            &PatchParams::apply(app_id!()),
            &status.to_patch(),
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
