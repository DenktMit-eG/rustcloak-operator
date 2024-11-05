use crate::crd::{HasApiObject, HasRoute, KeycloakApiStatus, KeycloakInstance};
use either::Either;
use k8s_openapi::NamespaceResourceScope;
use kube::{core::object::HasStatus, Resource, ResourceExt};
use serde::de::DeserializeOwned;
use std::ops::Deref;
use up_impl::{Container, HasContainer, HasUp, Root, Up};

pub trait Path {
    fn resource_path(&self) -> Option<String>;
    fn endpoint_path(&self) -> Option<String>;
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
        + HasStatus<Status = KeycloakApiStatus>
        + HasUp
        + Send
        + Sync,
    O: HasUp<Up: HasContainer>,
    <O::Up as HasContainer>::Container: Container,
    <<O::Up as HasContainer>::Container as Container>::Output: Path,
{
    fn resource_path(&self) -> Option<String> {
        self.status().and_then(|x| x.resource_path.clone())
    }

    fn endpoint_path(&self) -> Option<String> {
        let Some(up_resource_path) = self.up.resource_path() else {
            return None;
        };
        Some(format!("{}/{}", up_resource_path, self.value.mount_point()))
    }

    fn instance_ref(&self) -> String {
        self.up.instance_ref()
    }
}

impl<L: Path, R: Path> Path for Either<L, R> {
    fn resource_path(&self) -> Option<String> {
        match self {
            Either::Left(left) => left.resource_path(),
            Either::Right(right) => right.resource_path(),
        }
    }

    fn endpoint_path(&self) -> Option<String> {
        match self {
            Either::Left(left) => left.endpoint_path(),
            Either::Right(right) => right.endpoint_path(),
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
    fn resource_path(&self) -> Option<String> {
        Some("".to_string())
    }

    fn endpoint_path(&self) -> Option<String> {
        Some("".to_string())
    }

    fn instance_ref(&self) -> String {
        self.deref().name_unchecked()
    }
}
