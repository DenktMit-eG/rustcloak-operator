use super::client::ClientRef;
use crate::keycloak_types::ScopeRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiStatus, crd::namespace_scope,
    impl_object, schema_patch, traits::impl_endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakScope", "kcs" {
        #[kube(
            doc = "resource to define a Scope within a [KeyclaokClient](./keycloakclient.md)",
        )]
        /// the KeycloakScope resource
        pub struct KeycloakScopeSpec {
            #[serde(default, flatten)]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: ClientRef,
            #[schemars(schema_with = "schema")]
            pub definition: Option<ScopeRepresentation>,
        }
    }
}

impl_object!("scope" <ClientRef> / |_d| {"authz/resource-server/scope"} / "id" for KeycloakScopeSpec => ScopeRepresentation);

impl_endpoint!(KeycloakScope);

schema_patch!(KeycloakScopeSpec: |s| {
    s.prop("resources")
        .array_item()
        .remove_prop("scopesUma")
        .remove_prop("scopes")
        .additional_properties();
    s.prop("policies")
        .array_item()
        .remove_prop("scopesData")
        .additional_properties();
    s.prop("policies")
        .array_item()
        .prop("resourcesData")
        .array_item()
        .remove_prop("scopesUma")
        .remove_prop("scopes")
        .additional_properties();
});
