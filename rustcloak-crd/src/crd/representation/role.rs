use crate::keycloak_types::RoleRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    impl_object,
    macros::namespace_scope,
    refs::{ClientRef, RealmRef},
    schema_patch,
    traits::impl_instance_ref,
};
use either::Either;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{KeycloakClient, KeycloakRealm};

namespace_scope! {
    "KeycloakRole", "kcr" {
        #[kube(
            doc = "resource to define a Protocol Mapper within either a [KeycloakRealm](./keycloakrealm.md) or a [KeycloakClient](./keycloakclient.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
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
    }
}

type ParentRef = Either<RealmRef, ClientRef>;
type Parents = Either<KeycloakRealm, KeycloakClient>;

impl_object!("role" <parent_ref: ParentRef => Parents> / |_d| {"roles"} / name for KeycloakRoleSpec => RoleRepresentation);

impl_instance_ref!(KeycloakRole);

schema_patch!(KeycloakRoleSpec);
