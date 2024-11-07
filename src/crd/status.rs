use crate::{error::Error, util::FromError};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use kube::api::Patch;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakApiCondition {
    pub last_transition_time: Option<Time>,
    pub last_update_time: Option<Time>,
    pub message: Option<String>,
    pub reason: Option<String>,
    pub status: String,
    pub type_: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakApiStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,
    pub ready: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    pub conditions: Option<Vec<KeycloakApiCondition>>,
}

impl KeycloakApiStatus {
    pub fn ok<T: Into<String>>(status: T) -> Self {
        let status = status.into();
        Self {
            ready: true,
            status,
            message: "ok".to_string(),
            resource_path: None,
            conditions: None,
        }
    }
}

impl FromError for KeycloakApiStatus {
    fn from_error(err: &Error) -> Self {
        Self {
            ready: false,
            status: "Error".to_string(),
            message: err.to_string(),
            resource_path: None,
            conditions: None,
        }
    }
}

impl From<KeycloakApiStatus> for Patch<Value> {
    fn from(val: KeycloakApiStatus) -> Self {
        let status = serde_json::to_value(val).unwrap();
        Patch::Merge(json!({
            "status": status,
        }))
    }
}
