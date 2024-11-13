use crate::crd::{
    api_object_impl, schema_patch, ClientRef, KeycloakApiObjectOptions,
    KeycloakApiPatchList, KeycloakApiStatus, RealmRef,
};
use either::Either;
use keycloak::types::RoleRepresentation;
use kube::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};
use up_impl::OneOf;

use super::{KeycloakClient, KeycloakRealm};

type ParentRef = Either<RealmRef, ClientRef>;
type Parents = Either<KeycloakRealm, KeycloakClient>;
#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakRole",
    shortname = "kcr",
    doc = "resource to define a Protocol Mapper within either a [KeycloakRealm](./keycloakrealm.md) or a [KeycloakClient](./keycloakclient.md)",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    category = "keycloak",
    category = "all",
    namespaced,
    printcolumn = r#"{
            "name":"Ready",
            "type":"boolean",
            "description":"true if the realm is ready",
            "jsonPath":".status.ready"
        }"#,
    printcolumn = r#"{
            "name":"Status",
            "type":"string",
            "description":"Status String of the resource",
            "jsonPath":".status.status"
        }"#,
    printcolumn = r#"{
            "name":"Age",
            "type":"date",
            "description":"time since the realm was created",
            "jsonPath":".metadata.creationTimestamp"
        }"#
)]
#[serde(rename_all = "camelCase")]
/// the KeycloakRole resource
pub struct KeycloakRoleSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    #[serde(flatten)]
    pub parent_ref: ParentRef,
    #[schemars(schema_with = "schema")]
    pub definition: RoleRepresentation,
    #[serde(default, flatten)]
    pub patches: Option<KeycloakApiPatchList>,
}

crate::crd::route_impl!(Parents / "roles" / id: KeycloakRole .. parent_ref: OneOf<String, String>);

api_object_impl!(KeycloakRole, RoleRepresentation, "role");

schema_patch!(KeycloakRole);
