use crate::crd::{
    api_object_impl, schema_patch, KeycloakApiObjectOptions, KeycloakApiStatus,
    KeycloakRealm,
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

api_object_impl!(
    KeycloakIdentityProvider,
    IdentityProviderRepresentation,
    "idp"
);

crate::crd::route_impl!(KeycloakRealm / "identity-provider/instances" / alias: KeycloakIdentityProvider .. realm_ref: String);

schema_patch!(KeycloakIdentityProvider);
