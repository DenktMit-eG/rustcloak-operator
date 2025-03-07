use super::{ClientRef, GroupRef, RoleRef, UserRef, namespace_scope};
use crate::{KeycloakApiStatus, either::UntaggedEither, traits::impl_endpoint};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(untagged)]
pub enum RoleNameOrRef {
    Ref(RoleRef),
    Name { role_name: String },
}

namespace_scope! {
    "KeycloakRoleMapping", "krmp" {
        #[kube(
            doc = "represents a mapping between a user or group and a client",
            group = "rustcloak.k8s.eboland.de",
            version = "v1beta1",
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
                    "name":"Path",
                    "type":"string",
                    "description":"Path to the resource",
                    "jsonPath":".spec.resourcePath"
                }"#,
            printcolumn = r#"{
                    "name":"Status",
                    "type":"string",
                    "description":"Status String of the resource",
                    "jsonPath":".status.status"
                }"#
        )]
        pub struct KeycloakRoleMappingSpec {
            #[serde(flatten)]
            pub client_ref: Option<ClientRef>,
            #[serde(flatten)]
            pub role_ref: RoleNameOrRef,
            #[serde(flatten)]
            pub parent_ref: RoleMappingParentRef,
        }
    }
}

pub type RoleMappingParentRef = UntaggedEither<UserRef, GroupRef>;

impl_endpoint!(KeycloakRoleMapping);
