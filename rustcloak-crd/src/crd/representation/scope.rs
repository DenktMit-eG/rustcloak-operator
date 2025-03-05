use crate::keycloak_types::ScopeRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    crd::namespace_scope, impl_object, schema_patch, traits::impl_endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::ClientRef;

namespace_scope! {
    "KeycloakScope", "kcs" {
        #[kube(
            doc = "resource to define a Scope within a [KeyclaokClient](./keycloakclient.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1beta1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
        )]
        /// the KeycloakScope resource
        pub struct KeycloakScopeSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: ClientRef,
            #[schemars(schema_with = "schema")]
            pub definition: ScopeRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_object!("scope" <ClientRef> / |_d| {"authz/resource-server/scope"} / id for KeycloakScopeSpec => ScopeRepresentation);

impl_endpoint!(KeycloakScope);

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
