use crate::{
    crd::{
        ChildOf, HasEndpoint, HasParentType, KeycloakApiEndpoint,
        KeycloakClient, KeycloakClientScope, KeycloakProtocolMapper,
        KeycloakRealm,
    },
    error::{Error, Result},
};
use async_trait::async_trait;
use either::Either;
use k8s_openapi::NamespaceResourceScope;
use kube::{Api, Resource, ResourceExt};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

pub trait HasHierarchy {
    type HierarchyParent;
}

impl HasHierarchy for KeycloakRealm {
    type HierarchyParent = ();
}

impl<P: HasHierarchy + Resource, T: HasParentType<Parent = P>> HasHierarchy
    for T
{
    type HierarchyParent = Hierarchy<P>;
}

pub struct Hierarchy<O>
where
    O: HasHierarchy + Resource,
{
    pub object: Arc<O>,
    pub parent: O::HierarchyParent,
}

#[async_trait]
pub trait HierarchyQuery
where
    Self: Sized + Send + Sync,
{
    type Object;
    fn path(&self) -> String;

    fn instance_ref(&self) -> &str;

    async fn query(
        object: &Arc<Self::Object>,
        client: kube::Client,
    ) -> Result<Self>;
}

#[async_trait]
impl HierarchyQuery for Hierarchy<KeycloakRealm> {
    type Object = KeycloakRealm;

    fn instance_ref(&self) -> &str {
        &self.object.spec.instance_ref
    }

    fn path(&self) -> String {
        format!("admin/realms/{}", self.object.primary_key_value())
    }

    async fn query(
        object: &Arc<Self::Object>,
        _client: kube::Client,
    ) -> Result<Self> {
        Ok(Self {
            object: object.clone(),
            parent: (),
        })
    }
}

#[async_trait]
impl<O> HierarchyQuery for Hierarchy<O>
where
    O: Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + HasHierarchy
        + HasParentType
        + ChildOf
        + Clone
        + HasEndpoint
        + std::fmt::Debug
        + DeserializeOwned
        + Send
        + Sync,
    O::ParentRefType: AsRef<str> + Send + Sync,
    O::HierarchyParent: HierarchyQuery<Object = O::Parent> + Send + Sync,
    O::Parent: Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + Clone
        + std::fmt::Debug
        + DeserializeOwned
        + Send
        + Sync,
{
    type Object = O;

    fn path(&self) -> String {
        format!(
            "{}/{}/{}",
            self.parent.path(),
            self.object.sub_path(),
            self.object.primary_key_value()
        )
    }

    fn instance_ref(&self) -> &str {
        self.parent.instance_ref()
    }

    async fn query(object: &Arc<O>, client: kube::Client) -> Result<Self> {
        let client = client.clone();
        let object = object.clone();
        let ns = object.namespace().ok_or(Error::NoNamespace)?;
        let parent_ref = object.parent_ref();
        let parent = Api::<O::Parent>::namespaced(client.clone(), &ns)
            .get(parent_ref.as_ref())
            .await?;
        let parent = Arc::new(parent);
        let parent = O::HierarchyParent::query(&parent, client).await?;
        Ok(Self { object, parent })
    }
}

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
        let primary_key = self.primary_key_value();
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
        + ChildOf<ParentType = P, ParentRefType = String>
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
        let primary_key = self.primary_key_value();
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
