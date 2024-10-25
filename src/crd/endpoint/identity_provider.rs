use crate::crd::{
    child_of, endpoint_impl, schema_patch, HasEndpoint,
    KeycloakApiObjectOptions, KeycloakApiStatus, KeycloakRealm,
};
use keycloak::types::IdentityProviderRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakIdentityProvider",
    shortname = "kcip",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakIdentityProviderSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: IdentityProviderRepresentation,
}

endpoint_impl!(
    KeycloakIdentityProvider,
    IdentityProviderRepresentation,
    alias,
    idp
);

child_of!(KeycloakIdentityProvider, KeycloakRealm, realm_ref, "group");

schema_patch!(KeycloakIdentityProvider);
