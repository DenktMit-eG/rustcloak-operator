use crate::crd::{
    child_of, endpoint_impl, HasEndpoint, KeycloakApiObjectOptions,
    KeycloakApiStatus,
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
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakIdentityProviderMapperSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub identity_provider_ref: String,
    #[schemars(schema_with = "KeycloakIdentityProviderMapperSpec::schema")]
    pub definition: IdentityProviderMapperRepresentation,
}

endpoint_impl!(
    KeycloakIdentityProviderMapperSpec,
    IdentityProviderMapperRepresentation,
    id,
    ipm,
    |_| {}
);

child_of!(
    KeycloakIdentityProviderMapper,
    KeycloakIdentityProvider,
    identity_provider_ref,
    "mappers"
);
