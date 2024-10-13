mod admin_api;
mod instance;
mod realm;

use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;

pub use admin_api::*;
pub use instance::*;
pub use realm::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

trait KeyClaokSelector {
    fn keycloak_selector(&self) -> &LabelSelector;
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct KeycloakApiStatus {
    status: String,
    #[serde(default)]
    code: u32,
    #[serde(default)]
    message: String,
}
