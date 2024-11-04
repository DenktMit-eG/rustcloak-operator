use crate::crd::{HasApiObject, HasRoute, KeycloakInstance};
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

impl Path for Root<KeycloakInstance> {
    fn path(&self) -> String {
        "".to_string()
    }

    fn instance_ref(&self) -> String {
        self.deref().name_unchecked()
    }
}
