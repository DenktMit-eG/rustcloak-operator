use crate::crd::{
    api_object_impl, schema_patch, ClientRef, HasApiObject,
    KeycloakApiObjectOptions, KeycloakApiStatus, RealmRef,
};
use either::Either;
use keycloak::types::RoleRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::{KeycloakClient, KeycloakRealm};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakRole",
    shortname = "kcr",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakRoleSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    #[serde(flatten)]
    pub parent_ref: Either<RealmRef, ClientRef>,
    #[schemars(schema_with = "schema")]
    pub definition: RoleRepresentation,
}

crate::crd::route_impl!(<Either<KeycloakRealm, KeycloakClient>> / "roles" / id: KeycloakRole .. parent_ref: Either<RealmRef, ClientRef>);

api_object_impl!(KeycloakRole, RoleRepresentation, id, role);

schema_patch!(KeycloakRole);
