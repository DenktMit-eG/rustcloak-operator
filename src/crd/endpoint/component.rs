use crate::crd::{
    api_object_impl, schema_patch, KeycloakApiObjectOptions, KeycloakApiStatus,
};
use keycloak::types::ComponentRepresentation;
use kube::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakComponent",
    shortname = "kcco",
    doc = "resource to define a Component within a [KeycloakRealm](./keycloakrealm.md)",
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
/// the KeycloakComponent resource
pub struct KeycloakComponentSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// the name of the kubernetes object that created the realm.
    pub realm_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: ComponentRepresentation,
}

api_object_impl!(KeycloakComponent, ComponentRepresentation, "component");

crate::crd::route_impl!(KeycloakRealm / "components" / id: KeycloakComponent .. realm_ref: String);

schema_patch!(KeycloakComponent);
