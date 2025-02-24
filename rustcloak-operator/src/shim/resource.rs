use crate::error::{Error, Result};
use either::Either;
use k8s_openapi::NamespaceResourceScope;
use kube::{Api, Resource};
use rustcloak_crd::{traits::Endpoint, KeycloakInstance, KeycloakRestObject};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::{fmt::Debug, ops::Deref, sync::Arc};

pub struct ResourceShim<R> {
    resource: Arc<R>,
    client: kube::Client,
}

impl<R> Deref for ResourceShim<R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        self.resource.deref()
    }
}

impl<R> ResourceShim<R> {
    pub fn new(resource: &Arc<R>, client: &kube::Client) -> Self {
        let resource = Arc::clone(resource);
        let client = client.clone();
        Self { resource, client }
    }
}

impl<R> ResourceShim<R>
where
    R: Resource,
{
    pub fn name(&self) -> Result<&str> {
        self.resource.meta().name.as_deref().ok_or(Error::NoName)
    }

    pub fn namespace(&self) -> Result<&str> {
        self.resource
            .meta()
            .namespace
            .as_deref()
            .ok_or(Error::NoNamespace)
    }
}

pub trait RestShim {
    fn api_name(&self) -> Result<String>;
    fn payload(&self) -> Result<Value>;
}

impl<R> RestShim for ResourceShim<R>
where
    R: KeycloakRestObject + Resource,
    R::Definition: Serialize,
    Self: Send + Sync,
{
    fn api_name(&self) -> Result<String> {
        Ok(format!("{}-{}", R::API_PREFIX, self.name()?))
    }

    fn payload(&self) -> Result<Value> {
        Ok(serde_json::to_value(self.definition())?)
    }
}

#[async_trait::async_trait]
pub trait ParentShim<Marker> {
    type Parent;
    async fn parent(&self) -> Result<Self::Parent>;
}

pub struct ResourceMarker;
#[async_trait::async_trait]
impl<R> ParentShim<ResourceMarker> for ResourceShim<R>
where
    R: KeycloakRestObject + Resource,
    R::ParentObject: Resource<Scope = NamespaceResourceScope, DynamicType = ()>
        + Clone
        + Debug
        + DeserializeOwned,
    R::ParentRef: AsRef<str>,
    Self: Send + Sync,
{
    type Parent = R::ParentObject;

    async fn parent(&self) -> Result<Self::Parent> {
        let api = Api::<Self::Parent>::namespaced(
            self.client.clone(),
            self.namespace()?,
        );
        Ok(api.get(self.parent_ref().as_ref()).await?)
    }
}

pub struct EitherMarker;
#[async_trait::async_trait]
impl<R, RP, LP, RR, LR> ParentShim<EitherMarker> for ResourceShim<R>
where
    R: KeycloakRestObject<
            ParentObject = Either<RP, LP>,
            ParentRef = Either<LR, RR>,
        > + Resource,
    LP: Resource<Scope = NamespaceResourceScope, DynamicType = ()>
        + Clone
        + Debug
        + DeserializeOwned,
    LR: AsRef<str>,
    RP: Resource<Scope = NamespaceResourceScope, DynamicType = ()>
        + Clone
        + Debug
        + DeserializeOwned,
    RR: AsRef<str>,
    Self: Send + Sync,
{
    type Parent = Either<RP, LP>;

    async fn parent(&self) -> Result<Self::Parent> {
        match self.parent_ref() {
            Either::Left(l) => {
                let api = Api::<RP>::namespaced(
                    self.client.clone(),
                    self.namespace()?,
                );
                Ok(Either::Left(api.get(l.as_ref()).await?))
            }
            Either::Right(r) => {
                let api = Api::<LP>::namespaced(
                    self.client.clone(),
                    self.namespace()?,
                );
                Ok(Either::Right(api.get(r.as_ref()).await?))
            }
        }
    }
}

#[async_trait::async_trait]
pub trait InstanceShim {
    fn instance_ref(&self) -> Result<&str>;
    fn resource_path(&self) -> Result<&str>;
    async fn instance(&self) -> Result<KeycloakInstance>;
}

#[async_trait::async_trait]
impl<R> InstanceShim for ResourceShim<R>
where
    R: Endpoint + Resource<DynamicType = (), Scope = NamespaceResourceScope>,
    Self: Send + Sync,
{
    fn instance_ref(&self) -> Result<&str> {
        self.resource.instance_ref().ok_or(Error::NoData)
    }

    fn resource_path(&self) -> Result<&str> {
        self.resource.resource_path().ok_or(Error::NoData)
    }

    async fn instance(&self) -> Result<KeycloakInstance> {
        let api = Api::<KeycloakInstance>::namespaced(
            self.client.clone(),
            self.namespace()?,
        );
        Ok(api.get(self.instance_ref()?).await?)
    }
}
