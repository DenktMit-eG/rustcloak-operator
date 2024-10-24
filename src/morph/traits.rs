use crate::crd::{HasEndpoint, KeycloakApiEndpoint};
use crate::error::{Error, Result};
use async_trait::async_trait;
use kube::Resource;
use kube::ResourceExt;

#[async_trait]
pub trait ToApiObject
where
    Self: Resource + HasEndpoint + Sized,
{
    const PREFIX: &'static str;
    async fn create_endpoint(
        &self,
        _client: kube::Client,
    ) -> Result<KeycloakApiEndpoint>;

    fn primary_key_value_r(&self) -> Result<String> {
        self.primary_key_value()
            .map(|x| Ok(x.to_string()))
            .unwrap_or_else(|| self.uid().ok_or(Error::NoUid))
    }
}
