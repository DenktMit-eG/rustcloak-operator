use crate::crd::{
    api_object_impl, schema_patch, KeycloakApiObjectOptions, KeycloakApiStatus,
};
use keycloak::types::IdentityProviderMapperRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakIdentityProvider;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakIdentityProviderMapper",
    shortname = "kcipm",
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
/// the KeycloakIdentityProviderMapper resource
pub struct KeycloakIdentityProviderMapperSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// the name of the kubernetes object that created the identity provider.
    pub identity_provider_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: IdentityProviderMapperRepresentation,
}

api_object_impl!(
    KeycloakIdentityProviderMapper,
    IdentityProviderMapperRepresentation,
    "ipm"
);

crate::crd::route_impl!(KeycloakIdentityProvider / "identity-provider/instances" / id: KeycloakIdentityProviderMapper .. identity_provider_ref: String);

schema_patch!(KeycloakIdentityProviderMapper);
