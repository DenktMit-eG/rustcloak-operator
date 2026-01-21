use super::identity_provider::IdentityProviderRef;
use crate::keycloak_types::IdentityProviderMapperRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiStatus, crd::namespace_scope,
    impl_object, schema_patch, traits::impl_endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakIdentityProviderMapper",
    "kcipm" {
        #[kube(
            doc = "resource to define a identity provider mapper within a [KeyclaokIdentityProvider](./keycloakidentityprovider.md)",
        )]
        /// the KeycloakIdentityProviderMapper resource
        pub struct KeycloakIdentityProviderMapperSpec {
            #[serde(default, flatten)]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: IdentityProviderRef,
            #[schemars(schema_with = "schema")]
            pub definition: Option<IdentityProviderMapperRepresentation>,
        }
    }
}

impl_object!("ipm" <IdentityProviderRef> / |_d| {"mappers"} / "id" / |d| {
    d.definition.as_ref().and_then(|def| def.name.as_deref())
} for KeycloakIdentityProviderMapperSpec => IdentityProviderMapperRepresentation);

impl_endpoint!(KeycloakIdentityProviderMapper);

schema_patch!(KeycloakIdentityProviderMapperSpec);
