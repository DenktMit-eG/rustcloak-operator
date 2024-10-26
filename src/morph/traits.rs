use crate::error::Result;
use k8s_openapi::api::core::v1::EnvVar;
use kube::Resource;
use serde::Serialize;
use serde_json::Value;

use crate::crd::HasApiObject;

pub trait Morph {
    fn payload(&self) -> Result<Value>;
    fn variables(&self) -> Result<Vec<EnvVar>>;
}

impl<T> Morph for T
where
    T: Resource + HasApiObject,
    T::Definition: Serialize,
{
    fn payload(&self) -> Result<Value> {
        Ok(serde_json::to_value(self.definition())?)
    }

    fn variables(&self) -> Result<Vec<EnvVar>> {
        Ok(vec![])
    }
}
