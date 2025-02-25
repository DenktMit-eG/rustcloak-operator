use crate::{
    impl_object, macros::namespace_scope, schema_patch,
    traits::impl_instance_ref, ImmutableString, KeycloakApiObjectOptions,
    KeycloakApiPatchList, KeycloakApiStatus, KeycloakClient,
};
use keycloak::types::ResourceRepresentation;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakResource", "kcrs" {
        #[kube(
            doc = "resource to define a Resource within a [KeyclaokClient](./keycloakclient.md)",
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
        /// the KeycloakResource resource
        pub struct KeycloakResourceSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            /// the name of the kubernetes object that created the client.
            pub client_ref: ImmutableString,
            #[schemars(schema_with = "schema")]
            pub definition: ResourceRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_object!("resource" <client_ref: String => KeycloakClient> / |_d| {"authz/resource-server/resource"} / (id => "_id") for KeycloakResourceSpec => ResourceRepresentation);

impl_instance_ref!(KeycloakResource);

schema_patch!(KeycloakResourceSpec: |s| {
    s.prop("scopes")
        .array_item()
        .prop("policies")
        .array_item()
        .remove("scopesData")
        .remove("resourcesData")
        .additional_properties();
    s.prop("scopes").array_item().remove("resources");
    s.prop("scopesUma")
        .array_item()
        .prop("policies")
        .array_item()
        .remove("resourcesData")
        .remove("scopesData")
        .additional_properties();
    s.prop("scopesUma")
        .array_item()
        .remove("resources")
        .additional_properties();
});
