use super::{traits::ToApiObject, WithPrimaryKey};
use crate::{
    crd::{KeycloakApiEndpoint, KeycloakApiObjectOptions, KeycloakRealm},
    error::Result,
};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
impl ToApiObject for KeycloakRealm {
    const PREFIX: &'static str = "realm-";

    const PRIMARY_KEY: &'static str = "realm";

    async fn create_endpoint(
        &self,
        _client: kube::Client,
    ) -> Result<KeycloakApiEndpoint> {
        let path = format!("admin/realms/{}", self.primary_key()?);
        let instance_ref = &self.spec.instance_ref;
        Ok(KeycloakApiEndpoint::new(instance_ref, &path))
    }

    fn options(&self) -> Option<&KeycloakApiObjectOptions> {
        self.spec.options.as_ref()
    }

    fn payload(&self) -> Result<Value> {
        Ok(serde_json::to_value(&self.spec.definition)?)
    }
}
