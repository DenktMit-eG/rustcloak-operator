use crate::crd::HasRoute;
use crate::error::Result;
use async_trait::async_trait;
use either::Either;
use k8s_openapi::NamespaceResourceScope;
use kube::{Api, Resource};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

use super::hierarchy::Root;

pub struct Retriever<O, R> {
    phantom: PhantomData<(O, R)>,
}

#[async_trait]
pub trait Retrieve {
    type Child;
    type Object;
    async fn retrieve(
        client: kube::Client,
        ns: &str,
        obj_ref: &Self::Child,
    ) -> Result<Self::Object>;
}

#[async_trait]
impl<C> Retrieve for Retriever<Root, C>
where
    C: Send + Sync + HasRoute<ParentRefType = String>,
{
    type Object = Root;
    type Child = C;

    async fn retrieve(
        _client: kube::Client,
        _ns: &str,
        child: &Self::Child,
    ) -> Result<Self::Object> {
        Ok(child.route_parent_ref().clone())
    }
}

#[async_trait]
impl<O, C> Retrieve for Retriever<O, C>
where
    O: Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + HasRoute
        + Clone
        + std::fmt::Debug
        + DeserializeOwned
        + Send
        + Sync,
    C: HasRoute + Send + Sync,
    C::ParentRefType: AsRef<str> + Send + Sync,
{
    type Object = O;
    type Child = C;
    async fn retrieve(
        client: kube::Client,
        ns: &str,
        child: &Self::Child,
    ) -> Result<Self::Object> {
        let api = Api::<O>::namespaced(client, ns);
        let obj_ref = child.route_parent_ref();
        Ok(api.get(obj_ref.as_ref()).await?)
    }
}

#[async_trait]
impl<LO, RO, RK, LK, C> Retrieve for Retriever<Either<LO, RO>, (RK, LK, C)>
where
    LO: Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + HasRoute
        + Clone
        + std::fmt::Debug
        + DeserializeOwned
        + Send
        + Sync,
    RO: Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + HasRoute
        + Clone
        + std::fmt::Debug
        + DeserializeOwned
        + Send
        + Sync,
    RK: AsRef<str> + Send + Sync,
    LK: AsRef<str> + Send + Sync,
    C: HasRoute<ParentRefType = Either<LK, RK>> + Send + Sync,
{
    type Object = Either<LO, RO>;
    type Child = C;
    async fn retrieve(
        client: kube::Client,
        ns: &str,
        child: &Self::Child,
    ) -> Result<Self::Object> {
        let eith = child.route_parent_ref();
        let key = AsRef::<str>::as_ref(eith);
        match eith {
            Either::Left(_) => {
                let api = Api::<LO>::namespaced(client, ns);
                Ok(Either::Left(api.get(key).await?))
            }
            Either::Right(_) => {
                let api = Api::<RO>::namespaced(client, ns);
                Ok(Either::Right(api.get(key).await?))
            }
        }
    }
}
