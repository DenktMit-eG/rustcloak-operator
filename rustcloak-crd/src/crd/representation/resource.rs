use super::client::ClientRef;
use crate::keycloak_types::ResourceRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiStatus, crd::namespace_scope,
    impl_object, schema_patch, traits::impl_endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakResource", "kcrs" {
        #[kube(
            doc = "resource to define a Resource within a [KeyclaokClient](./keycloakclient.md)",
        )]
        /// the KeycloakResource resource
        pub struct KeycloakResourceSpec {
            #[serde(default, flatten)]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: ClientRef,
            #[schemars(schema_with = "schema")]
            pub definition: Option<ResourceRepresentation>,
        }
    }
}

impl_object!("resource" <ClientRef> / |_d| {"authz/resource-server/resource"} / "_id" for KeycloakResourceSpec => ResourceRepresentation);

impl_endpoint!(KeycloakResource);

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
