use super::{
    client::ClientRef, group::GroupRef, namespace_scope, role::RoleRef,
    user::UserRef,
};
use crate::{KeycloakApiStatus, either::UntaggedEither, traits::impl_endpoint};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ClientNameOrRef {
    ClientRef(ClientRef),
    KeycloakClient {
        #[serde(rename = "clientId")]
        /// The client id of the the client.
        client_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakRoleRef {
    /// The name of the role in keycloak
    pub name: String,

    /// If null the role is treated as a realm role, otherwise it is treated as a client role
    /// of the referenced kuberntes KeycloakClient resource.
    #[serde(flatten)]
    pub client_ref: Option<ClientNameOrRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", untagged)]
pub enum RoleNameOrRef {
    /// The kubernetes resource name of a KeycloakRole object. Mutual exclusive with role
    RoleRef(RoleRef),
    /// The name of the role in keycloak. Mutual exclusive with roleRef
    KeycloakRole {
        #[serde(rename = "role")]
        role: KeycloakRoleRef,
    },
}

namespace_scope! {
    "KeycloakRoleMapping", "kcrmp" {
        #[kube(
            doc = "represents a mapping between a user or group and a client",
            printcolumn = r#"{
                    "name": "Role Name",
                    "type": "string",
                    "description": "Role Name",
                    "jsonPath": ".spec.roleRef"
                }"#,
            printcolumn = r#"{
                    "name": "Role Ref",
                    "type": "string",
                    "description": "Role Ref",
                    "jsonPath": ".spec.roleRef"
                }"#,

        )]
        pub struct KeycloakRoleMappingSpec {
            /// The object that :the role mapping is for
            pub subject: RoleMappingParentRef,
            #[serde(flatten)]
            pub role: RoleNameOrRef,
        }
    }
}

pub type RoleMappingParentRef = UntaggedEither<UserRef, GroupRef>;

impl_endpoint!(KeycloakRoleMapping);
