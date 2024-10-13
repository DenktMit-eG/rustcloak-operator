use super::KeycloakApiStatus;
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakInstance",
    group = "rustcloak.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
pub struct KeycloakInstanceSpec {
    pub base_url: String,
    pub credentials_from: String,
    pub certificate_from: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Serialize,
    JsonSchema,
    Hash,
    Eq,
    PartialEq,
    Default,
)]
pub struct KeycloakInstanceSelector {
    pub name: String,
}
