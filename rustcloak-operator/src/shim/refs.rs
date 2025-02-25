use crate::error::{Error, Result};
use k8s_openapi::{ClusterResourceScope, ResourceScope};
use kube::{
    core::{object::HasSpec, NamespaceResourceScope},
    Api, Resource, ResourceExt,
};
use rustcloak_crd::KeycloakRestObject;
use serde::de::DeserializeOwned;
use std::{fmt::Debug, marker::PhantomData};

pub struct RefShim<S, T> {
    namespace: Option<String>,
    source: S,
    target: PhantomData<T>,
}

impl<S, T> RefShim<S, T> {
    pub fn from_resource<R>(resource: &R) -> Self
    where
        R: Resource + HasSpec,
        R::Spec: KeycloakRestObject<ParentRef = S, ParentObject = T>,
        <R::Spec as KeycloakRestObject>::ParentRef: AsRef<str> + Clone,
    {
        RefShim {
            source: resource.spec().parent_ref().clone(),
            namespace: resource.namespace(),
            target: PhantomData,
        }
    }
}

pub trait RefApi<T, S>
where
    T: Resource<Scope = S, DynamicType = ()>,
{
    fn api(&self, client: kube::Client) -> Result<Api<T>>;
}

impl<S, T> RefApi<T, NamespaceResourceScope> for RefShim<S, T>
where
    T: Resource<Scope = NamespaceResourceScope, DynamicType = ()>,
{
    fn api(&self, client: kube::Client) -> Result<Api<T>> {
        Ok(Api::namespaced(
            client,
            self.namespace.as_ref().ok_or(Error::NoNamespace)?,
        ))
    }
}

impl<S, T> RefApi<T, ClusterResourceScope> for RefShim<S, T>
where
    T: Resource<Scope = ClusterResourceScope, DynamicType = ()>,
{
    fn api(&self, client: kube::Client) -> Result<Api<T>> {
        Ok(Api::all(client))
    }
}

impl<T> RefShim<String, T>
where
    T: Resource<Scope: ResourceScope, DynamicType = ()>
        + Clone
        + Debug
        + DeserializeOwned,
    Self: RefApi<T, T::Scope>,
{
    pub async fn get_opt(&self, client: &kube::Client) -> Result<Option<T>> {
        let name = self.source.as_ref();
        let api = self.api(client.clone())?;
        Ok(api.get_opt(name).await?)
    }
}
