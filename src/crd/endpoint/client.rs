use crate::crd::{
    api_object_impl, schema_patch, KeycloakApiObjectOptions,
    KeycloakApiPatchList, KeycloakApiStatus,
};
use keycloak::types::ClientRepresentation;
use kube::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakClientSecretReference {
    pub secret_name: String,
    pub client_id_key: Option<String>,
    pub client_secret_key: Option<String>,
}

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakClient",
    shortname = "kcc",
    doc = "resource to define a Client within a [KeycloakRealm](./keycloakrealm.md)",
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
/// the KeycloakClient resource
pub struct KeycloakClientSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// the name of the kubernetes object that created the realm.
    pub realm_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: ClientRepresentation,
    #[serde(default, flatten)]
    pub patches: Option<KeycloakApiPatchList>,
    pub client_secret: Option<KeycloakClientSecretReference>,
}

api_object_impl!(KeycloakClient, ClientRepresentation, "client");

crate::crd::route_impl!(KeycloakRealm / "clients" / id: KeycloakClient .. realm_ref: String);

schema_patch!(KeycloakClient: |s| {
    s.prop("authorizationSettings")
        .prop("policies")
        .array_item()
        .prop("resourcesData")
        .array_item()
        .prop("scopes")
        .array_item()
        .remove("policies")
        .remove("resources")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("policies")
        .array_item()
        .prop("resourcesData")
        .array_item()
        .prop("scopesUma")
        .array_item()
        .remove("policies")
        .remove("resources")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("policies")
        .array_item()
        .prop("scopesData")
        .array_item()
        .prop("resources")
        .array_item()
        .remove("scopes")
        .remove("scopesUma")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("policies")
        .array_item()
        .prop("scopesData")
        .array_item()
        .remove("policies")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("resources")
        .array_item()
        .prop("scopes")
        .array_item()
        .prop("policies")
        .array_item()
        .remove("resourcesData")
        .remove("scopesData")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("resources")
        .array_item()
        .prop("scopes")
        .array_item()
        .remove("resources")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("resources")
        .array_item()
        .prop("scopesUma")
        .array_item()
        .prop("policies")
        .array_item()
        .remove("resourcesData")
        .remove("scopesData")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("resources")
        .array_item()
        .prop("scopesUma")
        .array_item()
        .remove("resources")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("scopes")
        .array_item()
        .prop("policies")
        .array_item()
        .prop("resourcesData")
        .array_item()
        .remove("scopes")
        .remove("scopesUma")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("scopes")
        .array_item()
        .prop("policies")
        .array_item()
        .remove("scopesData")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("scopes")
        .array_item()
        .prop("resources")
        .array_item()
        .remove("scopes")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("scopes")
        .array_item()
        .prop("resources")
        .array_item()
        .remove("scopesUma")
        .additional_properties();
});
