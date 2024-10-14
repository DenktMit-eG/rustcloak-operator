use std::sync::Arc;

use crate::{
    crd::KeycloakInstance,
    error::{Error, Result},
    util::SecretUtils,
};
use async_trait::async_trait;
use k8s_openapi::api::core::v1::Secret;
use keycloak::{KeycloakAdminToken, KeycloakTokenSupplier};
use kube::{
    runtime::{controller::Action, Controller},
    Api, ResourceExt,
};
use log::info;
use reqwest::{Method, Response, StatusCode};
use serde_json::Value;

use super::controller_runner::LifetimeController;
use crate::crd::KeycloakAdminApi;

#[derive(Debug, Default)]
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
        let instance = instance_api.get(instance_name).await?;
        let secret_name = format!("{}-api-token", instance_name);
        let token = secret_api.get(&secret_name).await?.token()?;

        Ok((instance.spec.base_url, token))
    }

    async fn request(
        &self,
        method: Method,
        url: &str,
        token: &KeycloakAdminToken,
        payload: &Value,
    ) -> Result<Response> {
        info!("Payload: {}", serde_json::to_string_pretty(payload)?);
        let request = self
            .http
            .request(method, url)
            .json(payload)
            .bearer_auth(token.get(url).await?);
        println!("{:?}", request);
        Ok(request.send().await?.error_for_status()?)
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
        let url = format!("{url}/admin/{path}");
        // First try to PUT, if we get a 404, try to POST
        let response =
            match self.request(Method::PUT, &url, &token, &payload).await {
                Err(Error::ReqwestError(e)) => {
                    if e.status() == Some(StatusCode::NOT_FOUND) {
                        let url = url.rsplit_once('/').unwrap().0;
                        self.request(Method::POST, url, &token, &payload).await
                    } else {
                        Err(e)?
                    }
                }
                r => r,
            }?;
        info!("Response: {:?}", response.text().await?);
        // TODO: handle errors
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
