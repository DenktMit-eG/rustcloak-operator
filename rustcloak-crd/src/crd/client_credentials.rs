use super::{InstanceRef, KeycloakClientSecretReference, namespace_scope};
use crate::KeycloakApiStatus;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakClientCredential", "kccc" {
        #[kube(
            doc = "represents credentials for a keycloak client",
            group = "rustcloak.k8s.eboland.de",
            version = "v1beta1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
            printcolumn = r#"{
                    "name":"Path",
                    "type":"string",
                    "description":"Path to the resource",
                    "jsonPath":".spec.resourcePath"
                }"#,
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
                }"#
        )]
        pub struct KeycloakClientCredentialSpec {
            #[serde(flatten)]
            pub instance_ref: InstanceRef,
            pub resource_path: String,
            pub client_secret: KeycloakClientSecretReference,
        }
    }
}
