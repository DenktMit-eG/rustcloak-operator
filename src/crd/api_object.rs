use super::{
    ImmutableJsonObject, ImmutableString, JsonObject, KeycloakApiStatus,
    WithStatus,
};
use crate::error::Result;
use k8s_openapi::api::core::v1::EnvVar;
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::Add;

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub endpoint: KeycloakApiEndpoint,
    pub immutable_payload: ImmutableJsonObject,
    pub payload: JsonObject,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vars: Option<Vec<EnvVar>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct KeycloakApiEndpoint {
    pub instance_ref: ImmutableString,
    pub path: ImmutableString,
}

impl KeycloakApiEndpoint {
    pub fn new(instance_ref: &str, path: &str) -> Self {
        let instance_ref = instance_ref.to_string().into();
        let path = path.to_string().into();
        Self { instance_ref, path }
    }
}

impl Add<&str> for KeycloakApiEndpoint {
    type Output = Result<KeycloakApiEndpoint>;

    fn add(self, rhs: &str) -> Self::Output {
        let path = self.path.to_string() + rhs;
        let path = path.into();
        Ok(KeycloakApiEndpoint {
            path,
            instance_ref: self.instance_ref,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
pub struct KeycloakApiObjectOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_create: Option<bool>,
}

impl WithStatus<KeycloakApiStatus> for KeycloakApiObject {
    fn status(&self) -> Option<&KeycloakApiStatus> {
        self.status.as_ref()
    }
}
