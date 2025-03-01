use crate::keycloak_types::ScopeRepresentation;
use crate::{
    ImmutableString, KeycloakApiObjectOptions, KeycloakApiPatchList,
    KeycloakApiStatus, KeycloakClient, impl_object, macros::namespace_scope,
    schema_patch, traits::impl_instance_ref,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakScope", "kcs" {
        #[kube(
            doc = "resource to define a Scope within a [KeyclaokClient](./keycloakclient.md)",
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
        /// the KeycloakScope resource
        pub struct KeycloakScopeSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            /// the name of the kubernetes object that created the client.
            pub client_ref: ImmutableString,
            #[schemars(schema_with = "schema")]
            pub definition: ScopeRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_object!("scope" <client_ref: String => KeycloakClient> / |_d| {"authz/resource-server/scope"} / id for KeycloakScopeSpec => ScopeRepresentation);

impl_instance_ref!(KeycloakScope);

schema_patch!(KeycloakScopeSpec: |s| {
    s.prop("resources")
        .array_item()
        .remove("scopesUma")
        .remove("scopes")
        .additional_properties();
    s.prop("policies")
        .array_item()
        .remove("scopesData")
        .additional_properties();
    s.prop("policies")
        .array_item()
        .prop("resourcesData")
        .array_item()
        .remove("scopesUma")
        .remove("scopes")
        .additional_properties();
});
