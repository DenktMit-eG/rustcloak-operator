use super::{
    instance::InstanceRef, namespace_scope, user::KeycloakUserSecretReference,
};
use crate::KeycloakApiStatus;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakUserCredential", "kcuc" {
        #[kube(
            doc = "represents credentials for a keycloak user",
            printcolumn = r#"{
                    "name":"Path",
                    "type":"string",
                    "description":"Path to the resource",
                    "jsonPath":".spec.resourcePath"
                }"#,
        )]
        pub struct KeycloakUserCredentialSpec {
            #[serde(flatten)]
            pub instance_ref: InstanceRef,
            pub resource_path: String,
            pub user_secret: KeycloakUserSecretReference,
        }
    }
}
