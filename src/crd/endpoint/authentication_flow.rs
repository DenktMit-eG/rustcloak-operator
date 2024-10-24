use crate::crd::{
    child_of, endpoint_impl, HasEndpoint, KeycloakApiObjectOptions,
    KeycloakApiStatus,
};
use keycloak::types::AuthenticationFlowRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakAuthenticationFlow",
    shortname = "kcaf",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakAuthenticationFlowSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[schemars(schema_with = "KeycloakAuthenticationFlowSpec::schema")]
    pub definition: AuthenticationFlowRepresentation,
}

endpoint_impl!(
    KeycloakAuthenticationFlowSpec,
    AuthenticationFlowRepresentation,
    id,
    realm,
    |_| {}
);

child_of!(
    KeycloakAuthenticationFlow,
    KeycloakRealm,
    realm_ref,
    "authentication/flows"
);
