use super::{client::ClientRef, realm::RealmRef};
use crate::either::UntaggedEither;
use crate::keycloak_types::RoleRepresentation;
use crate::refs::ref_type;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiStatus, crd::namespace_scope,
    impl_object, schema_patch, traits::impl_endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakRole", "kcr" {
        #[kube(
            doc = "resource to define a Protocol Mapper within either a [KeycloakRealm](./keycloakrealm.md) or a [KeycloakClient](./keycloakclient.md)",
        )]
        /// the KeycloakRole resource
        pub struct KeycloakRoleSpec {
            #[serde(default, flatten)]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: ParentRef,
            #[schemars(schema_with = "schema")]
            pub definition: Option<RoleRepresentation>,
        }
    }
}

pub type ParentRef = UntaggedEither<RealmRef, ClientRef>;
impl_object!("role" <ParentRef> / |_d| {"roles"} / "name" / |d| {
    d.definition.as_ref().and_then(|def| def.name.as_deref())
} for KeycloakRoleSpec => RoleRepresentation);

ref_type!(
    RoleRef,
    role_ref,
    KeycloakRole,
    "A reference to a KeycloakRole"
);

impl_endpoint!(KeycloakRole);

schema_patch!(KeycloakRoleSpec);
