use std::sync::Arc;

use crate::{
    app_id,
    crd::KeycloakAdminSession,
    error::{Error, Result},
};
use async_trait::async_trait;
use k8s_openapi::api::core::v1::Secret;
use keycloak::KeycloakAdminToken;
use kube::{
    api::{Patch, PatchParams},
    core::ErrorResponse,
    runtime::{controller::Action, watcher, Controller},
    Api, Resource, ResourceExt,
};
use log::{debug, info};

use super::controller_runner::LifetimeController;
use crate::{crd::KeycloakInstance, util::SecretUtils};

#[derive(Debug, Default)]
pub struct KeycloakInstanceController {}

impl KeycloakInstanceController {
    fn api_token_name(&self, resource: &KeycloakInstance) -> String {
        let name = resource.name_unchecked();
        format!("{name}-api-token")
    }

    pub async fn keycloak_login(
        &self,
        api: &Api<Secret>,
        resource: &KeycloakInstance,
    ) -> Result<KeycloakAdminToken> {
        let cred_secret_name = &resource.spec.credentials_from;
        let (user, password) =
            api.get(cred_secret_name).await?.credentials()?;

        let http_client = reqwest::Client::new();
        let token = KeycloakAdminToken::acquire(
            &resource.spec.base_url,
            &user,
            &password,
            &http_client,
        )
        .await?;
        Ok(token)
    }

    pub async fn setup_token(
        &self,
        api: &Api<Secret>,
        token_secret_name: &str,
        resource: &KeycloakInstance,
    ) -> Result<KeycloakAdminToken> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let name = self.api_token_name(resource);
        let owner_ref = resource.owner_ref(&());
        let token = match api.get(token_secret_name).await {
            Ok(secret) => {
                if let Ok(token) = secret.token() {
                    info!("Secret {} found", token_secret_name);
                    token
                } else {
                    info!(
                        "Secret {token_secret_name} found, but invalid, trying to login",
                    );
                    self.keycloak_login(api, resource).await?
                }
            }
            Err(kube::Error::Api(ErrorResponse { code: 404, .. })) => {
                info!(
                    "Secret {} not found, trying to login",
                    token_secret_name
                );
                self.keycloak_login(api, resource).await?
            }
            Err(x) => {
                return Err(x.into());
            }
        };

        let secret = Secret::from_token(&name, &ns, owner_ref, &token);
        api.patch(&name, &PatchParams::apply(app_id!()), &Patch::Apply(secret))
            .await?;
        Ok(token)
    }
}

#[async_trait]
impl LifetimeController for KeycloakInstanceController {
    type Resource = KeycloakInstance;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource> {
        controller.owns(
            Api::<KeycloakAdminSession>::all(client.clone()),
            watcher::Config::default(),
        )
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let secret_name = self.api_token_name(&resource);
        let secret_api = Api::<Secret>::namespaced(client.clone(), &ns);
        let _token = self
            .setup_token(&secret_api, &secret_name, &resource)
            .await?;

        debug!("Checking for secret {}", secret_name);

        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let secret_name = self.api_token_name(&resource);
        let secret_api = Api::<Secret>::namespaced(client.clone(), &ns);

        secret_api.delete(&secret_name, &Default::default()).await?;

        Ok(Action::await_change())
    }
}
