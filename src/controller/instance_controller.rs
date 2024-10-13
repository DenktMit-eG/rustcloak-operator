use std::sync::Arc;

use crate::{
    error::{Error, Result},
    util::ToToken,
};
use async_trait::async_trait;
use k8s_openapi::{api::core::v1::Secret, ByteString};
use keycloak::KeycloakAdminToken;
use kube::{
    api::{ObjectMeta, Patch, PatchParams},
    core::ErrorResponse,
    runtime::{controller::Action, watcher, Controller},
    Api, Resource, ResourceExt,
};
use log::{debug, info, warn};

use super::controller_runner::LifetimeController;
use crate::crd::KeycloakInstance;

#[derive(Debug, Clone)]
pub struct KeycloakInstanceController {}

impl KeycloakInstanceController {
    pub fn new() -> Self {
        Self {}
    }

    fn api_token_name(&self, resource: &KeycloakInstance) -> String {
        let name = resource.name_unchecked();
        format!("{name}-api-token")
    }

    async fn save_token(
        &self,
        api: &Api<Secret>,
        resource: &KeycloakInstance,
        token: &KeycloakAdminToken,
    ) -> Result<()> {
        let owner_ref = resource.controller_owner_ref(&()).unwrap();
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let name = self.api_token_name(resource);
        let secret =
            Secret {
                metadata: ObjectMeta {
                    name: Some(name.clone()),
                    namespace: Some(ns.clone()),
                    owner_references: Some(vec![owner_ref]),
                    ..Default::default()
                },
                data: Some(
                    [(
                        "token".to_string(),
                        ByteString(serde_json::to_vec(token)?),
                    )]
                    .into(),
                ),
                ..Default::default()
            };
        api.patch(
            &name,
            &PatchParams::apply(env!("CARGO_PKG_NAME")),
            &Patch::Apply(secret),
        )
        .await?;
        Ok(())
    }

    pub async fn keycloak_login(
        &self,
        api: &Api<Secret>,
        resource: &KeycloakInstance,
    ) -> Result<KeycloakAdminToken> {
        let secret_name = &resource.spec.credentials_from;
        let secret = api.get(secret_name).await?;

        let data = secret.data.as_ref().ok_or(Error::NoData)?;
        let user = data
            .get("user")
            .and_then(|x| String::from_utf8(x.0.clone()).ok())
            .ok_or(Error::NoUsername)?;
        let password = data
            .get("password")
            .and_then(|x| String::from_utf8(x.0.clone()).ok())
            .ok_or(Error::NoPassword)?;

        let http_client = reqwest::Client::new();
        let token = KeycloakAdminToken::acquire(
            &resource.spec.base_url,
            &user,
            &password,
            &http_client,
        )
        .await?;
        self.save_token(api, resource, &token).await?;
        Ok(token)
    }
}

#[async_trait]
impl LifetimeController for KeycloakInstanceController {
    type Resource = KeycloakInstance;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        _client: &kube::Client,
    ) -> Controller<Self::Resource> {
        /*controller.owns(
            Api::<Secret>::all(client.clone()),
            watcher::Config::default(),
        )*/
        controller
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let secret_name = self.api_token_name(&resource);
        let secret_api = Api::<Secret>::namespaced(client.clone(), &ns);

        debug!("Checking for secret {}", secret_name);
        match secret_api.get(&secret_name).await {
            Ok(secret) => {
                if secret.to_token().is_ok() {
                    info!("Secret {} found", secret_name);
                } else {
                    info!(
                        "Secret {} found, but invalid, trying to login",
                        secret_name
                    );
                    self.keycloak_login(&secret_api, &resource).await?;
                }
            }
            Err(kube::Error::Api(ErrorResponse { code: 404, .. })) => {
                info!("Secret {} not found, trying to login", secret_name);
                self.keycloak_login(&secret_api, &resource).await?;
            }
            Err(x) => {
                return Err(x.into());
            }
        }

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
