use super::traits::ToApiObject;
use crate::{
    crd::{KeycloakApiObjectOptions, KeycloakRealm},
    error::Result,
};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
impl ToApiObject for KeycloakRealm {
    const PREFIX: &'static str = "realm-";

    const IMMUTABLE_FIELDS: &'static [&'static str] = &["realm"];

    async fn create_path(&self, _client: kube::Client) -> Result<String> {
        Ok(format!("admin/realms/{}", self.spec.definition.realm))
    }

    fn api(&self) -> &KeycloakApiObjectOptions {
        &self.spec.api
    }

    fn payload(&self) -> Result<Value> {
        Ok(serde_json::to_value(&self.spec.definition)?)
    }
}
