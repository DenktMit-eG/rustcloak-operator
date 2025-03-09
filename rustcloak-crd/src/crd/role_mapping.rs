use super::{ClientRef, GroupRef, RoleRef, UserRef, namespace_scope};
use crate::{KeycloakApiStatus, either::UntaggedEither, traits::impl_endpoint};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct RoleLink {
    /// The name of the role in keycloak
    pub name: String,
    /// If null the role is treated as a realm role, otherwise it is treated as a client role
    /// of the referenced kuberntes KeycloakClient resource.
    #[serde(flatten)]
    pub client_ref: Option<ClientRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", untagged)]
pub enum RoleNameOrRef {
    /// The kubernetes resource name of a KeycloakRole object. Mutual exclusive with roleName
    RoleRef(RoleRef),
    /// The name of the role in keycloak. Mutual exclusive with roleRef
    Role { role: RoleLink },
}

namespace_scope! {
    "KeycloakRoleMapping", "kcrmp" {
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
            /// The object that the role mapping is for
            pub subject: RoleMappingParentRef,
            #[serde(flatten)]
            pub role_ref: RoleNameOrRef,
        }
    }
}

pub type RoleMappingParentRef = UntaggedEither<UserRef, GroupRef>;

impl_endpoint!(KeycloakRoleMapping);
