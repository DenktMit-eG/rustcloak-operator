use crate::crd::{
    api_object_impl, schema_patch, KeycloakApiObjectOptions, KeycloakApiStatus,
};
use keycloak::types::ScopeRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakClient;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakScope",
    shortname = "kcs",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakScopeSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub client_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: ScopeRepresentation,
}

api_object_impl!(KeycloakScope, ScopeRepresentation, "scope");

crate::crd::route_impl!(KeycloakClient / "authz/resource-server/scope" / id: KeycloakScope .. client_ref: String);

schema_patch!(KeycloakScope: |s| {
    s.prop("resources")
        .array_item()
        .remove("scopesUma")
        .remove("scopes")
        .additional_properties();
    s.prop("policies")
        .array_item()
        .remove("scopesData")
        .additional_properties();
    s.prop("policies")
        .array_item()
        .prop("resourcesData")
        .array_item()
        .remove("scopesUma")
        .remove("scopes")
        .additional_properties();
});
