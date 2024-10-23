use super::traits::ToApiObject;
use crate::{
    crd::{
        KeycloakApiEndpoint, KeycloakApiObjectOptions, KeycloakClient,
        KeycloakRealm,
    },
    error::{Error, Result},
};
use async_trait::async_trait;
use kube::{Api, ResourceExt};
use serde_json::Value;

#[async_trait]
impl ToApiObject for KeycloakClient {
    const PREFIX: &'static str = "client-";

    const PRIMARY_KEYS: &'static [&'static str] = &["id"];

    async fn create_endpoint(
        &self,
        client: kube::Client,
    ) -> Result<KeycloakApiEndpoint> {
        let ns = self.namespace().ok_or(Error::NoNamespace)?;
        let realm_name = &self.spec.realm_name;
        let realm_api = Api::<KeycloakRealm>::namespaced(client.clone(), &ns);
        let id = self
            .spec
            .definition
            .id
            .as_ref()
            .ok_or(Error::NoPrimaryKey)?;
        let realm_endpoint = realm_api
            .get(realm_name)
            .await?
            .create_endpoint(client)
            .await?;
        realm_endpoint + &format!("/clients/{id}")
    }

    fn options(&self) -> Option<&KeycloakApiObjectOptions> {
        self.spec.options.as_ref()
    }

    fn payload(&self) -> Result<Value> {
        Ok(serde_json::to_value(&self.spec.definition)?)
    }
}
