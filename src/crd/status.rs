use crate::error::Error;
use kube::api::Patch;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakApiStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,
    pub ready: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String,
    pub code: u32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

impl KeycloakApiStatus {
    pub fn ok<T: Into<String>>(status: T) -> Self {
        let status = status.into();
        Self {
            ready: true,
            status,
            code: 200,
            message: "ok".to_string(),
            resource_path: None,
        }
    }
}

impl From<Error> for KeycloakApiStatus {
    fn from(err: Error) -> Self {
        Self::from(&err)
    }
}

impl From<&Error> for KeycloakApiStatus {
    fn from(err: &Error) -> Self {
        Self {
            ready: false,
            status: "Error".to_string(),
            code: 0,
            message: err.to_string(),
            resource_path: None,
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
