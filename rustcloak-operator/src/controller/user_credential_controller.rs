use crate::{
    app_id,
    controller::controller_runner::LifecycleController,
    error::{Error, Result},
    util::{
        ApiExt, ApiFactory, K8sKeycloakBuilder, RefWatcher, Retrieve,
        Retriever, SecretUtils,
    },
};
use async_trait::async_trait;
use either::for_both;
use k8s_openapi::{ByteString, api::core::v1::Secret};
use kube::{
    Api, Resource, ResourceExt,
    api::{ObjectMeta, Patch, PatchParams},
    runtime::{Controller, controller::Action, watcher},
};
use randstr::randstr;
use rustcloak_crd::{
    InstanceRef, KeycloakUserCredential,
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
        let password =
            if let Some(secret) = secret_api.get_opt(secret_name).await? {
                if let Ok([_, password, _]) =
                    secret.extract(&Some(resource.spec.user_secret.clone()))
                {
                    Some(password)
                } else {
                    None
                }
            } else {
                None
            };

        let password = if let Some(password) = password {
            password
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
            let owner_ref = resource.owner_ref(&()).unwrap();

            let secret = Secret {
                data: Some(data),
                metadata: ObjectMeta {
                    name: Some(secret_name.to_string()),
                    namespace: ns.clone(),
                    owner_references: Some(vec![owner_ref]),
                    ..Default::default()
                },
                type_: Some("Opaque".to_string()),
                ..Default::default()
            };

            secret_api
                .patch(
                    secret_name,
                    &PatchParams::apply(app_id!()),
                    &Patch::Apply(secret),
                )
                .await?;
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
