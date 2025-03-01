use crate::keycloak_types::IdentityProviderMapperRepresentation;
use crate::{
    ImmutableString, KeycloakApiObjectOptions, KeycloakApiPatchList,
    KeycloakApiStatus, KeycloakIdentityProvider, impl_object,
    macros::namespace_scope, schema_patch, traits::impl_instance_ref,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakIdentityProviderMapper",
    "kcipm" {
        #[kube(
            doc = "resource to define a identity provider mapper within a [KeyclaokIdentityProvider](./keycloakidentityprovider.md)",
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
        /// the KeycloakIdentityProviderMapper resource
        pub struct KeycloakIdentityProviderMapperSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            /// the name of the kubernetes object that created the identity provider.
            pub identity_provider_ref: ImmutableString,
            #[schemars(schema_with = "schema")]
            pub definition: IdentityProviderMapperRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_object!("ipm" <identity_provider_ref: String => KeycloakIdentityProvider> / |_d| {"mappers"} / id for KeycloakIdentityProviderMapperSpec => IdentityProviderMapperRepresentation);

impl_instance_ref!(KeycloakIdentityProviderMapper);

schema_patch!(KeycloakIdentityProviderMapperSpec);
