use std::sync::Arc;

use crate::{
    crd::KeycloakInstance,
    error::{Error, Result},
    util::ToToken,
};
use async_trait::async_trait;
use k8s_openapi::api::core::v1::Secret;
use keycloak::{KeycloakAdminToken, KeycloakTokenSupplier};
use kube::{
    runtime::{controller::Action, Controller},
    Api, ResourceExt,
};

use super::controller_runner::LifetimeController;
use crate::crd::KeycloakAdminApi;

#[derive(Clone, Debug)]
pub struct KeycloakAdminApiController {
    http: reqwest::Client,
}

impl KeycloakAdminApiController {
    pub fn new() -> Self {
        let http = reqwest::Client::new();
        Self { http }
    }

    async fn get_creds(
        client: kube::Client,
        resource: &KeycloakAdminApi,
    ) -> Result<(String, KeycloakAdminToken)> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let secret_api = Api::<Secret>::namespaced(client.clone(), &ns);
        let instance_api =
            Api::<KeycloakInstance>::namespaced(client.clone(), &ns);

        let instance_name = &resource.spec.api.keycloak_selector.name;
        let instance = instance_api.get(&instance_name).await?;
        let secret_name = format!("{}-api-token", instance_name);
        let token = secret_api.get(&secret_name).await?.to_token()?;

        Ok((instance.spec.base_url, token))
    }
}

#[async_trait]
impl LifetimeController for KeycloakAdminApiController {
    type Resource = KeycloakAdminApi;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        _client: &kube::Client,
    ) -> Controller<Self::Resource> {
        controller
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let path = &resource.spec.path;
        let (url, token) = Self::get_creds(client.clone(), &resource).await?;
        let payload = resource.resolve(client).await?;
        // TODO: handle errors
        let _response = self
            .http
            .post(format!("{url}/{path}"))
            .json(&payload)
            .bearer_auth(token.get(&url).await?)
            .send()
            .await?;

        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let path = &resource.spec.path;
        let (url, token) = Self::get_creds(client.clone(), &resource).await?;
        // TODO: handle errors
        let _response = self
            .http
            .delete(format!("{url}/{path}"))
            .bearer_auth(token.get(&url).await?)
            .send()
            .await?;

        Ok(Action::await_change())
    }
}
