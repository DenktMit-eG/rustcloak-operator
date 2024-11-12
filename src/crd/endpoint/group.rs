use crate::crd::{
    api_object_impl, schema_patch, KeycloakApiObjectOptions,
    KeycloakApiPatchList, KeycloakApiStatus,
};
use keycloak::types::GroupRepresentation;
use kube::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakGroup",
    shortname = "kcg",
    doc = "resource to define a Group within a [KeycloakRealm](./keycloakrealm.md)",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced,
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
/// the KeycloakGroup resource
pub struct KeycloakGroupSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// the name of the kubernetes object that created the realm.
    pub realm_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: GroupRepresentation,
    #[serde(default, flatten)]
    pub patches: Option<KeycloakApiPatchList>,
}

api_object_impl!(KeycloakGroup, GroupRepresentation, "group");

crate::crd::route_impl!(KeycloakRealm / "groups" / id: KeycloakGroup .. realm_ref: String);

schema_patch!(KeycloakGroup: |s| {
    s.remove("subGroups");
});
