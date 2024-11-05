use crate::crd::{HasApiObject, HasRoute, KeycloakInstance};
use either::Either;
use k8s_openapi::NamespaceResourceScope;
use kube::{Resource, ResourceExt};
use serde::de::DeserializeOwned;
use std::ops::Deref;
use up_impl::{Container, HasContainer, HasUp, Root, Up};

pub trait Path {
    fn path(&self) -> String;
    fn instance_ref(&self) -> String;
}

impl<O> Path for Up<O>
where
    O: Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + HasApiObject
        + Clone
        + std::fmt::Debug
        + DeserializeOwned
        + HasRoute
        + HasUp
        + Send
        + Sync,
    O: HasUp<Up: HasContainer>,
    <O::Up as HasContainer>::Container: Container,
    <<O::Up as HasContainer>::Container as Container>::Output: Path,
{
    fn path(&self) -> String {
        format!(
            "{}/{}/{}",
            self.up.path(),
            self.value.mount_point(),
            self.value.id()
        )
    }

    fn instance_ref(&self) -> String {
        self.up.instance_ref()
    }
}

impl<L: Path, R: Path> Path for Either<L, R> {
    fn path(&self) -> String {
        match self {
            Either::Left(left) => left.path(),
            Either::Right(right) => right.path(),
        }
    }

    fn instance_ref(&self) -> String {
        match self {
            Either::Left(left) => left.instance_ref(),
            Either::Right(right) => right.instance_ref(),
        }
    }
}

impl Path for Root<KeycloakInstance> {
    fn path(&self) -> String {
        "".to_string()
    }

    fn instance_ref(&self) -> String {
        self.deref().name_unchecked()
    }
}
