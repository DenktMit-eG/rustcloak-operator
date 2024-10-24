use super::{
    keycloak_endpoint_impl, HasKeycloakEndpoint, KeycloakApiObjectOptions,
    KeycloakApiStatus,
};
use keycloak::types::ClientRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakClient",
    shortname = "kcc",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakClientSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[schemars(schema_with = "KeycloakClientSpec::schema")]
    pub definition: ClientRepresentation,
}

keycloak_endpoint_impl!(KeycloakClientSpec, ClientRepresentation, id, |s| {
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
