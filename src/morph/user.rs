use super::{traits::ToApiObject, WithPrimaryKey};
use crate::{
    crd::{
        KeycloakApiEndpoint, KeycloakApiObjectOptions, KeycloakRealm,
        KeycloakUser,
    },
    error::{Error, Result},
};
use async_trait::async_trait;
use kube::{Api, ResourceExt};
use serde_json::Value;

#[async_trait]
impl ToApiObject for KeycloakUser {
    const PREFIX: &'static str = "user-";

    const PRIMARY_KEY: &'static str = "id";

    async fn create_endpoint(
        &self,
        client: kube::Client,
    ) -> Result<KeycloakApiEndpoint> {
        let ns = self.namespace().ok_or(Error::NoNamespace)?;
        let realm_name = &self.spec.realm_ref;
        let realm_api = Api::<KeycloakRealm>::namespaced(client.clone(), &ns);
        let realm_endpoint = realm_api
            .get_opt(realm_name)
            .await?
            .ok_or(Error::NoRealm(ns, realm_name.clone()))?
            .create_endpoint(client)
            .await?;
        Ok(realm_endpoint + "/users/" + self.primary_key()?)
    }

    fn options(&self) -> Option<&KeycloakApiObjectOptions> {
        self.spec.options.as_ref()
    }

    fn payload(&self) -> Result<Value> {
        Ok(serde_json::to_value(&self.spec.definition)?)
    }
}
