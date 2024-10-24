use crate::crd::{
    endpoint_impl, HasEndpoint, KeycloakApiObjectOptions, KeycloakApiStatus,
};
use keycloak::types::ResourceRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakResource",
    shortname = "kcrs",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakResourceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[schemars(schema_with = "KeycloakResourceSpec::schema")]
    pub definition: ResourceRepresentation,
}

endpoint_impl!(
    KeycloakResourceSpec,
    ResourceRepresentation,
    id,
    resource,
    |_| {}
);
