use super::traits::ToApiObject;
use crate::{
    crd::{KeycloakApiEndpoint, KeycloakRealm, KeycloakUser},
    error::{Error, Result},
};
use async_trait::async_trait;
use kube::{Api, ResourceExt};

#[async_trait]
impl ToApiObject for KeycloakUser {
    const PREFIX: &'static str = "user-";

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
        Ok(realm_endpoint + "/users/" + self.primary_key_value_r()?)
    }
}
