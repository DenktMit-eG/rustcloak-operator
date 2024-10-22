use crate::error::Error;
use kube::api::Patch;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct KeycloakApiStatus {
    ready: bool,
    status: String,
    #[serde(default)]
    code: u32,
    #[serde(default)]
    message: String,
}

impl From<Error> for KeycloakApiStatus {
    fn from(err: Error) -> Self {
        Self::from(&err)
    }
}

impl From<&Error> for KeycloakApiStatus {
    fn from(err: &Error) -> Self {
        KeycloakApiStatus {
            ready: false,
            status: "error".to_string(),
            code: 0,
            message: err.to_string(),
        }
    }
}

impl KeycloakApiStatus {
    pub fn to_patch(self) -> Patch<Value> {
        let status = serde_json::to_value(self).unwrap();
        Patch::Merge(json!({
            "status": status,
        }))
    }
}
