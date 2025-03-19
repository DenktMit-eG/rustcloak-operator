use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::InstanceRef;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakApiCondition {
    pub last_transition_time: Option<Time>,
    pub message: Option<String>,
    pub reason: Option<String>,
    pub status: String,
    pub type_: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakApiStatusEndpoint {
    pub resource_path: String,
    pub instance: InstanceRef,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakApiStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", flatten)]
    pub endpoint: Option<KeycloakApiStatusEndpoint>,
    pub ready: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<KeycloakApiCondition>,
}

impl KeycloakApiStatus {
    pub fn ok<T: Into<String>>(status: T) -> Self {
        let status = status.into();
        Self {
            ready: true,
            status,
            message: "ok".to_string(),
            ..Default::default()
        }
    }
}
