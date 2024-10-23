use crate::crd::{KeycloakApiEndpoint, KeycloakApiObjectOptions};
use crate::error::{Error, Result};
use async_trait::async_trait;
use kube::Resource;
use kube::ResourceExt;
use serde_json::Value;

#[async_trait]
pub trait ToApiObject {
    const PREFIX: &'static str;
    const PRIMARY_KEY: &'static str;

    async fn create_endpoint(
        &self,
        _client: kube::Client,
    ) -> Result<KeycloakApiEndpoint>;

    fn options(&self) -> Option<&KeycloakApiObjectOptions>;

    fn payload(&self) -> Result<Value>;
}

pub trait WithPrimaryKey {
    fn primary_key(&self) -> Result<String>;
}

impl<R> WithPrimaryKey for R
where
    R: ToApiObject + Resource,
{
    fn primary_key(&self) -> Result<String> {
        let ns = self.namespace().ok_or(Error::NoNamespace)?;
        let payload = serde_json::to_value(&self.payload()?)?;

        Ok(payload
            .as_object()
            .unwrap()
            .get(R::PRIMARY_KEY)
            .and_then(|x| x.as_str())
            .map(|x| x.to_string())
            .unwrap_or_else(|| format!("{ns}_{}", self.name_unchecked())))
    }
}
