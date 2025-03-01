use k8s_openapi::serde_json::{self, Value, json};
use kube::api::Patch;
use rustcloak_crd::KeycloakApiStatus;

use crate::error::Error;

use super::FromError;

pub trait ToPatch {
    fn to_patch(&self) -> Patch<Value>;
}

impl ToPatch for KeycloakApiStatus {
    fn to_patch(&self) -> Patch<Value> {
        let status = serde_json::to_value(self).unwrap();
        Patch::Merge(json!({
            "status": status,
        }))
    }
}

impl ToPatch for Error {
    fn to_patch(&self) -> Patch<Value> {
        KeycloakApiStatus::from_error(self).to_patch()
    }
}
