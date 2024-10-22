use crate::crd::KeycloakApiObjectOptions;
use crate::error::Result;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ToApiObject {
    const PREFIX: &'static str;
    const IMMUTABLE_FIELDS: &'static [&'static str];

    async fn create_path(&self, _client: kube::Client) -> Result<String>;

    fn api(&self) -> &KeycloakApiObjectOptions;

    fn payload(&self) -> Result<Value>;
}
