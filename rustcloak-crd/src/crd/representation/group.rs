use super::RealmRef;
use crate::keycloak_types::GroupRepresentation;
use crate::refs::ref_type;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    crd::namespace_scope, impl_object, schema_patch, traits::impl_endpoint,
};
use either::Either;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakGroup", "kcg" {
        #[kube(
            doc = "resource to define a Group within a [KeycloakRealm](./keycloakrealm.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
        )]
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
    }
}

type ParentRef = Either<RealmRef, SubGroupRef>;

impl_object!("group" <ParentRef> / |d| {
    if d.parent_ref.is_left() {
        "groups"
    } else {
        "children"
    }
} / id for KeycloakGroupSpec => GroupRepresentation);

impl_endpoint!(KeycloakGroup);

schema_patch!(KeycloakGroupSpec: |s| {
    s.remove("subGroups");
});
ref_type!(SubGroupRef, parent_group_ref, KeycloakGroup);
