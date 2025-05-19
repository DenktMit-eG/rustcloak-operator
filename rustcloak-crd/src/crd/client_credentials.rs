use super::{
    client::KeycloakClientSecretReference, instance::InstanceRef,
    namespace_scope,
};
use crate::KeycloakApiStatus;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakClientCredential", "kccc" {
        #[kube(
            doc = "represents credentials for a keycloak client",
            printcolumn = r#"{
                    "name":"Path",
                    "type":"string",
                    "description":"Path to the resource",
                    "jsonPath":".spec.resourcePath"
                }"#,
        )]
        pub struct KeycloakClientCredentialSpec {
            #[serde(flatten)]
            pub instance_ref: InstanceRef,
            pub resource_path: String,
            pub client_secret: KeycloakClientSecretReference,
        }
    }
}
