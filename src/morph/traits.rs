use crate::crd::{KeycloakApiEndpoint, KeycloakApiObjectOptions};
use crate::error::Result;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ToApiObject {
    const PREFIX: &'static str;
    const PRIMARY_KEYS: &'static [&'static str];

    async fn create_endpoint(
        &self,
        _client: kube::Client,
    ) -> Result<KeycloakApiEndpoint>;

    fn options(&self) -> Option<&KeycloakApiObjectOptions>;

    fn payload(&self) -> Result<Value>;
}
