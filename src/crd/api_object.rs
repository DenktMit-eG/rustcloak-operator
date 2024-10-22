use super::{
    ImmutableJsonObject, ImmutableString, JsonObject, KeycloakApiStatus,
    KeycloakInstanceSelector, WithStatus,
};
use k8s_openapi::api::core::v1::EnvVar;
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakApiObject",
    shortname = "kcao",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    namespaced,
    status = "KeycloakApiStatus"
)]
/// defines an API request to the Keycloak Admin API.
pub struct KeycloakApiObjectSpec {
    pub api: KeycloakApiObjectOptions,
    pub path: ImmutableString,
    pub immutable_payload: ImmutableJsonObject,
    pub payload: JsonObject,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vars: Option<Vec<EnvVar>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
pub struct KeycloakApiObjectOptions {
    pub keycloak_selector: KeycloakInstanceSelector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_create: Option<bool>,
}

impl WithStatus<KeycloakApiStatus> for KeycloakApiObject {
    fn status(&self) -> Option<&KeycloakApiStatus> {
        self.status.as_ref()
    }
}
