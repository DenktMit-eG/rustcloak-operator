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
                    "name":"Role Name",
                    "type":"string",
                    "description":"Role Name",
                    "jsonPath":".spec.roleRef"
                }"#,
            printcolumn = r#"{
                    "name":"Role Ref",
                    "type":"string",
                    "description":"Role Ref",
                    "jsonPath":".spec.roleRef"
                }"#,

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
