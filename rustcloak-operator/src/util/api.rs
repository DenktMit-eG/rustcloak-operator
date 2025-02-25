use crate::error::Result;
use kube::{Api, Resource, ResourceExt};
use rustcloak_crd::{refs::ParentRef, KeycloakRestObject};

enum ApiShim {
    Namespaced(),
    Cluster(),
}

impl<R> From<R> for ApiShim
where
    R: Resource + KeycloakRestObject,
    R::ParentRef: Into<ParentRef> + Clone,
{
    fn from(val: R) -> Self {
        let parent_ref: ParentRef = val.parent_ref().clone().into();

    }
}
