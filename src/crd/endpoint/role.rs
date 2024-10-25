use crate::crd::{
    endpoint_impl, schema_patch, HasEndpoint, KeycloakApiObjectOptions,
    KeycloakApiStatus,
};
use keycloak::types::RoleRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakRole",
    shortname = "kcr",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakRoleSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: RoleRepresentation,
}

endpoint_impl!(KeycloakRole, RoleRepresentation, id, role);

schema_patch!(KeycloakRole);
