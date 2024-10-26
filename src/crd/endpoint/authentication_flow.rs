use crate::crd::{
    api_object_impl, child_of, route_impl, schema_patch, HasApiObject,
    KeycloakApiObjectOptions, KeycloakApiStatus,
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
    #[schemars(schema_with = "schema")]
    pub definition: AuthenticationFlowRepresentation,
}

api_object_impl!(
    KeycloakAuthenticationFlow,
    AuthenticationFlowRepresentation,
    id,
    realm
);

child_of!(
    KeycloakAuthenticationFlow,
    KeycloakRealm,
    realm_ref,
    "authentication/flows"
);

route_impl!(KeycloakRealm / "authentication/flows" / id: KeycloakAuthenticationFlow .. realm_ref: String);

schema_patch!(KeycloakAuthenticationFlow);
