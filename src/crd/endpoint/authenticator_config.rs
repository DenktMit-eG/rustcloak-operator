use crate::{
    child_of,
    crd::{
        endpoint_impl, HasEndpoint, KeycloakApiObjectOptions, KeycloakApiStatus,
    },
};
use keycloak::types::AuthenticatorConfigRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakAuthenticatorConfig",
    shortname = "kcac",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakAuthenticatorConfigSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[schemars(schema_with = "KeycloakAuthenticatorConfigSpec::schema")]
    pub definition: AuthenticatorConfigRepresentation,
}

endpoint_impl!(
    KeycloakAuthenticatorConfigSpec,
    AuthenticatorConfigRepresentation,
    id,
    authconfig,
    |_| {}
);

child_of!(
    KeycloakAuthenticatorConfig,
    KeycloakRealm,
    realm_ref,
    "authentication/config"
);
