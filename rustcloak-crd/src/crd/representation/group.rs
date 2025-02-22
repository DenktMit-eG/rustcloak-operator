use crate::{
    impl_object,
    refs::{RealmRef, SubGroupRef},
    schema_patch,
    traits::impl_instance_ref,
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
};
use either::Either;
use keycloak::types::GroupRepresentation;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakGroup",
    shortname = "kcg",
    doc = "resource to define a Group within a [KeycloakRealm](./keycloakrealm.md)",
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
/// the KeycloakGroup resource
pub struct KeycloakGroupSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// the name of the kubernetes object that created the realm.
    #[serde(flatten)]
    pub parent_ref: ParentRef,
    #[schemars(schema_with = "schema")]
    pub definition: GroupRepresentation,
    #[serde(default, flatten)]
    pub patches: Option<KeycloakApiPatchList>,
}

type ParentRef = Either<RealmRef, SubGroupRef>;
type Parent = Either<KeycloakRealm, KeycloakGroup>;

impl_object!("group" <parent_ref: ParentRef => Parent> / |d| {
    if d.parent_ref.is_left() {
        "groups"
    } else {
        "children"
    }
} / id for KeycloakGroup => GroupRepresentation);

impl_instance_ref!(KeycloakGroup);

schema_patch!(KeycloakGroup: |s| {
    s.remove("subGroups");
});
