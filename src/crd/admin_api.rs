use super::KeycloakApiStatus;
use super::KeycloakInstanceSelector;
use k8s_openapi::api::core::v1::EnvVar;
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema, Default,
)]
#[kube(
    kind = "KeycloakAdminApi",
    group = "k8s.eboland.de",
    version = "v1",
    namespaced,
    status = "KeycloakApiStatus"
)]
pub struct KeycloakAdminApiSpec {
    pub api: KeycloakAdminApiOptions,
    pub path: String,
    pub payload: String,
    pub vars: Option<Vec<EnvVar>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
pub struct KeycloakAdminApiOptions {
    pub keycloak_selector: KeycloakInstanceSelector,
    pub must_create: Option<bool>,
}
