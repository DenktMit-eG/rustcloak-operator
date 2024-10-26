use crate::crd::{
    api_object_impl, child_of, schema_patch, HasApiObject,
    KeycloakApiObjectOptions, KeycloakApiStatus,
};
use keycloak::types::UserRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakUser",
    shortname = "kcu",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakUserSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: UserRepresentation,
}

api_object_impl!(KeycloakUser, UserRepresentation, id, user);

crate::crd::route_impl!(KeycloakRealm / "users" / id: KeycloakUser .. realm_ref: String);

child_of!(
    KeycloakUser,
    KeycloakRealm,
    realm_ref,
    "authz/resource-server/scope"
);

schema_patch!(KeycloakUser);
