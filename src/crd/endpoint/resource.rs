use super::KeycloakClient;
use crate::crd::{
    api_object_impl, schema_patch, HasRoute, KeycloakApiObjectOptions,
    KeycloakApiPatchList, KeycloakApiStatus,
};
use keycloak::types::ResourceRepresentation;
use kube::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};
use up_impl::HasUp;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakResource",
    shortname = "kcrs",
    doc = "resource to define a Resource within a [KeyclaokClient](./keycloakclient.md)",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    category = "keycloak",
    category = "all",
    namespaced,
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
/// the KeycloakResource resource
pub struct KeycloakResourceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// the name of the kubernetes object that created the client.
    pub client_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: ResourceRepresentation,
    #[serde(default, flatten)]
    pub patches: Option<KeycloakApiPatchList>,
}

//crate::crd::route_impl!(KeycloakClient / "authz/resource-server/resource" / _id: KeycloakResource .. client_ref: String);
impl HasRoute for KeycloakResource {
    type ParentType = KeycloakClient;
    type ParentRefType = String;

    fn id_ident() -> &'static str {
        "_id"
    }

    fn id_option(&self) -> Option<&str> {
        self.spec.definition.id.as_deref()
    }

    fn mount_point(&self) -> &'static str {
        "authz/resource-server/resource"
    }
}

impl HasUp for KeycloakResource {
    type Up = KeycloakClient;
    type UpKey = String;

    fn key(&self) -> String {
        self.spec.client_ref.clone()
    }
}

api_object_impl!(KeycloakResource, ResourceRepresentation, "resource");

schema_patch!(KeycloakResource: |s| {
    s.prop("scopes")
        .array_item()
        .prop("policies")
        .array_item()
        .remove("scopesData")
        .remove("resourcesData")
        .additional_properties();
    s.prop("scopes").array_item().remove("resources");
    s.prop("scopesUma")
        .array_item()
        .prop("policies")
        .array_item()
        .remove("resourcesData")
        .remove("scopesData")
        .additional_properties();
    s.prop("scopesUma")
        .array_item()
        .remove("resources")
        .additional_properties();
});
