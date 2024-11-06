use super::{ImmutableString, KeycloakApiStatus};
use k8s_openapi::api::core::v1::EnvVar;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakApiObject",
    shortname = "kcapi",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    namespaced,
    status = "KeycloakApiStatus",
    printcolumn = r#"{
            "name":"Instance",
            "type":"string",
            "description":"",
            "jsonPath":".spec.endpoint.instanceRef"
        }"#,
    printcolumn = r#"{
            "name":"Ready",
            "type":"boolean",
            "description":"",
            "jsonPath":".status.ready"
        }"#,
    printcolumn = r#"{
            "name":"Status",
            "type":"string",
            "description":"",
            "jsonPath":".status.status"
        }"#,
    printcolumn = r#"{
            "name":"Age",
            "type":"date",
            "description":"",
            "jsonPath":".metadata.creationTimestamp"
        }"#
)]
#[serde(rename_all = "camelCase")]
/// defines an API request to the Keycloak Admin API.
pub struct KeycloakApiObjectSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub endpoint: KeycloakApiEndpoint,
    pub immutable_payload: ImmutableString,
    pub payload: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vars: Vec<EnvVar>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
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

impl<T> Add<T> for KeycloakApiEndpoint
where
    T: AsRef<str>,
{
    type Output = KeycloakApiEndpoint;

    fn add(self, rhs: T) -> Self::Output {
        let path = self.path.to_string() + rhs.as_ref();
        KeycloakApiEndpoint::new(&self.instance_ref, &path)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
/// Options for the request to the Keycloak Admin API.
pub struct KeycloakApiObjectOptions {}
