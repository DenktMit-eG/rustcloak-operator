use super::traits::ToApiObject;
use crate::{
    crd::{KeycloakApiEndpoint, KeycloakRealm},
    error::Result,
};
use async_trait::async_trait;

#[async_trait]
impl ToApiObject for KeycloakRealm {
    const PREFIX: &'static str = "realm-";

    async fn create_endpoint(
        &self,
        _client: kube::Client,
    ) -> Result<KeycloakApiEndpoint> {
        let path = format!("admin/realms/{}", self.primary_key_value_r()?);
        let instance_ref = &self.spec.instance_ref;
        Ok(KeycloakApiEndpoint::new(instance_ref, &path))
    }
}
