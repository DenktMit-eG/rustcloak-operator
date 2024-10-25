use crate::crd::{
    child_of, endpoint_impl, schema_patch, HasEndpoint,
    KeycloakApiObjectOptions, KeycloakApiStatus,
};
use keycloak::types::GroupRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakGroup",
    shortname = "kcg",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakGroupSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: GroupRepresentation,
}

endpoint_impl!(KeycloakGroup, GroupRepresentation, id, group);

child_of!(KeycloakGroup, KeycloakRealm, realm_ref, "group");

schema_patch!(KeycloakGroup: |s| {
    s.remove("subGroups");
});
