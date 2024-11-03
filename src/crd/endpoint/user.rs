use crate::crd::{
    api_object_impl, schema_patch, KeycloakApiObjectOptions, KeycloakApiStatus,
};
use keycloak::types::UserRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakUserSecretReference {
    pub secret_name: Option<String>,
    pub username_key: Option<String>,
    pub password_key: Option<String>,
}

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
    pub user_secret: Option<KeycloakUserSecretReference>,
}

api_object_impl!(KeycloakUser, UserRepresentation, "user");

crate::crd::route_impl!(KeycloakRealm / "users" / id: KeycloakUser .. realm_ref: String);

schema_patch!(KeycloakUser);
