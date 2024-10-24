use crate::{
    crd::{
        ChildOf, HasEndpoint, KeycloakApiEndpoint, KeycloakClient,
        KeycloakClientScope, KeycloakProtocolMapper, KeycloakRealm,
    },
    error::{Error, Result},
};
use async_trait::async_trait;
use either::Either;
use k8s_openapi::NamespaceResourceScope;
use kube::{Api, Resource, ResourceExt};
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait Resolver
where
    Self: Sized,
{
    async fn resolve(
        &self,
        client: kube::Client,
    ) -> Result<KeycloakApiEndpoint>;
}

#[async_trait]
impl Resolver for KeycloakRealm {
    async fn resolve(&self, _: kube::Client) -> Result<KeycloakApiEndpoint> {
        let instance_ref = &self.spec.instance_ref;
        let primary_key = self.primary_key_value().unwrap();
        Ok(KeycloakApiEndpoint::new(instance_ref, "admin/realms/")
            + primary_key)
    }
}

#[async_trait]
impl<T, P> Resolver for T
where
    P: Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + Clone
        + std::fmt::Debug
        + DeserializeOwned
        + Resolver
        + Send
        + Serialize,
    T: Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + HasEndpoint
        + ChildOf<Parent = P, ParentRefType = String>
        + Send,
    Self: Send + Sync,
{
    async fn resolve(
        &self,
        client: kube::Client,
    ) -> Result<KeycloakApiEndpoint> {
        let ns = self.namespace().ok_or(Error::NoNamespace)?;
        let parent_ref = self.parent_ref();
        let parent = Api::<P>::namespaced(client.clone(), &ns)
            .get(parent_ref.as_ref())
            .await?;
        let path = self.sub_path();
        let primary_key = self.primary_key_value().unwrap();
        Ok(parent.resolve(client.clone()).await?
            + "/"
            + path
            + "/"
            + primary_key)
    }
}

#[async_trait]
impl Resolver for KeycloakProtocolMapper {
    async fn resolve(
        &self,
        client: kube::Client,
    ) -> Result<KeycloakApiEndpoint> {
        let ns = self.namespace().ok_or(Error::NoNamespace)?;
        match self.parent_ref() {
            Either::Left(client_ref) => {
                let parent =
                    Api::<KeycloakClient>::namespaced(client.clone(), &ns)
                        .get(client_ref.as_ref())
                        .await?;
                parent.resolve(client).await
            }
            Either::Right(client_scope_ref) => {
                let parent =
                    Api::<KeycloakClientScope>::namespaced(client.clone(), &ns)
                        .get(client_scope_ref.as_ref())
                        .await?;
                parent.resolve(client).await
            }
        }
    }
}
