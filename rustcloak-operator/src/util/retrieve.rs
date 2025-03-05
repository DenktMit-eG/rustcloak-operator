use crate::error::Result;
use crate::morph::Morph;
use either::Either;
use k8s_openapi::{ClusterResourceScope, NamespaceResourceScope};
use kube::{Resource, core::object::HasStatus};
use rustcloak_crd::KeycloakRestObject;
use rustcloak_crd::marker::{EitherMarker, ResourceMarker};
use rustcloak_crd::{
    KeycloakApiStatus,
    either::UntaggedEither,
    inner_spec::HasInnerSpec,
    marker::HasMarker,
    refs::{HasParent, Ref},
};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::ops::Deref;

use super::{ApiExt, ApiFactory};

shorter_bounds::alias!(
    pub trait Parent: Send
        + Sync
        + 'static
        + Sized
        + Clone
        + std::fmt::Debug
        + DeserializeOwned
);
shorter_bounds::alias!(
    pub trait ParentRef: Ref<Target: Parent + HasMarker>
        + Send
        + Sync
        + Debug
        + Clone
        + DeserializeOwned
        + 'static
);
shorter_bounds::alias!(
    pub trait InnerSpec: HasParent<ParentRef: ParentRef>
        + KeycloakRestObject<Definition: Serialize>
        + Send
        + Sync
        + Debug
        + Morph
        + Clone
        + DeserializeOwned
);
shorter_bounds::alias! (
    pub trait Representation: Resource<DynamicType = ()>
        + Send
        + Sync
        + Debug
        + Clone
        + HasInnerSpec<InnerSpec: InnerSpec>
        + DeserializeOwned
        + HasStatus<Status = KeycloakApiStatus>
        + 'static
);

#[async_trait::async_trait]
pub trait Retrieve {
    type Ref: Ref;
    async fn get(
        client: kube::Client,
        reference: &Self::Ref,
        ns: &Option<String>,
    ) -> Result<<Self::Ref as Ref>::Target>;
}

#[async_trait::async_trait]
impl<R> Retrieve for (R, ResourceMarker<NamespaceResourceScope>)
where
    R: ParentRef,
    R::Target:
        Parent + Resource<DynamicType = (), Scope = NamespaceResourceScope>,
{
    type Ref = R;

    async fn get(
        client: kube::Client,
        reference: &Self::Ref,
        ns: &Option<String>,
    ) -> Result<<Self::Ref as Ref>::Target> {
        let api = ApiExt::<R::Target>::api(client, ns);
        Ok(api.get(reference.as_ref()).await?)
    }
}

#[async_trait::async_trait]
impl<R> Retrieve for (R, ResourceMarker<ClusterResourceScope>)
where
    R: ParentRef,
    R::Target:
        Parent + Resource<DynamicType = (), Scope = ClusterResourceScope>,
{
    type Ref = R;

    async fn get(
        client: kube::Client,
        reference: &Self::Ref,
        ns: &Option<String>,
    ) -> Result<<Self::Ref as Ref>::Target> {
        let api = ApiExt::<R::Target>::api(client, ns);
        Ok(api.get(reference.as_ref()).await?)
    }
}

#[async_trait::async_trait]
impl<L, R> Retrieve for (UntaggedEither<L, R>, EitherMarker)
where
    L: ParentRef,
    R: ParentRef,
    Retriever<L>: Retrieve<Ref = L>,
    Retriever<R>: Retrieve<Ref = R>,
{
    type Ref = UntaggedEither<L, R>;

    async fn get(
        client: kube::Client,
        reference: &Self::Ref,
        ns: &Option<String>,
    ) -> Result<<Self::Ref as Ref>::Target> {
        match reference.deref() {
            Either::Left(l) => {
                Retriever::<L>::get(client, l, ns).await.map(Either::Left)
            }
            Either::Right(r) => {
                Retriever::<R>::get(client, r, ns).await.map(Either::Right)
            }
        }
    }
}

pub type Retriever<R> = (R, <<R as Ref>::Target as HasMarker>::Marker);
pub type ParentGetter<R> =
    Retriever<<<R as HasInnerSpec>::InnerSpec as HasParent>::ParentRef>;

shorter_bounds::alias!(
    pub trait ParentRetrieve<R: (HasInnerSpec<InnerSpec: HasParent>)>:
        Retrieve<Ref = <<R as HasInnerSpec>::InnerSpec as HasParent>::ParentRef>
);
