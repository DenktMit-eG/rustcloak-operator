use crate::keycloak_types::ComponentRepresentation;
use crate::{
    ImmutableString, KeycloakApiObjectOptions, KeycloakApiPatchList,
    KeycloakApiStatus, KeycloakRealm, impl_object, macros::namespace_scope,
    schema_patch, traits::impl_instance_ref,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakComponent", "kcco" {
        #[kube(
            doc = "resource to define a Component within a [KeycloakRealm](./keycloakrealm.md)",
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
        /// the KeycloakComponent resource
        pub struct KeycloakComponentSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            /// the name of the kubernetes object that created the realm.
            pub realm_ref: ImmutableString,
            #[schemars(schema_with = "schema")]
            pub definition: ComponentRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_object!("component" <realm_ref: String => KeycloakRealm> / |_d| {"components"} / id for KeycloakComponentSpec => ComponentRepresentation);

impl_instance_ref!(KeycloakComponent);

schema_patch!(KeycloakComponentSpec);
