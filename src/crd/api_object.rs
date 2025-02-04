use super::{ImmutableString, KeycloakApiStatus};
use k8s_openapi::api::core::v1::EnvVar;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakApiObject",
    shortname = "kcapi",
    doc = "Custom Resource for Keycloak API requests. The user should not use this resource directly.",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    namespaced,
    status = "KeycloakApiStatus",
    category = "keycloak",
    category = "all",
    printcolumn = r#"{
            "name":"Instance",
            "type":"string",
            "description":"Instance that API request is sent to",
            "jsonPath":".spec.endpoint.instanceRef"
        }"#,
    printcolumn = r#"{
            "name":"Ready",
            "type":"boolean",
            "description":"true if the realm is ready",
            "jsonPath":".status.ready"
        }"#,
    printcolumn = r#"{
            "name":"Status",
            "type":"string",
            "description":"Status String of the resource",
            "jsonPath":".status.status"
        }"#,
    printcolumn = r#"{
            "name":"Age",
            "type":"date",
            "description":"time since the realm was created",
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
pub enum KeycloakApiEndpointPath {
    // BUG: while the values of Path and Parent variants are both ImmutableString, there's
    // there's currently no guard in place prevent the user from replacing the Parent variant with
    // a Path variant. This is a potential source of bugs.
    #[serde(rename = "path")]
    Path(ImmutableString),
    #[serde(rename = "parent")]
    Parent {
        parent_ref: ImmutableString,
        sub_path: ImmutableString,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakApiEndpoint {
    pub instance_ref: ImmutableString,
    #[serde(flatten)]
    pub path_def: KeycloakApiEndpointPath,
}

impl KeycloakApiEndpoint {
    pub fn new(instance_ref: &str, path: &str) -> Self {
        let instance_ref = instance_ref.to_string().into();
        let path = path.to_string().into();
        Self {
            instance_ref,
            path_def: KeycloakApiEndpointPath::Path(path),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
/// Options for the request to the Keycloak Admin API.
pub struct KeycloakApiObjectOptions {}
