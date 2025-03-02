use crate::keycloak_types::IdentityProviderMapperRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    crd::namespace_scope, impl_object, schema_patch, traits::impl_instance_ref,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::IdentityProviderRef;

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
        )]
        /// the KeycloakIdentityProviderMapper resource
        pub struct KeycloakIdentityProviderMapperSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            /// the name of the kubernetes object that created the identity provider.
            pub parent_ref: IdentityProviderRef,
            #[schemars(schema_with = "schema")]
            pub definition: IdentityProviderMapperRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_object!("ipm" <IdentityProviderRef> / |_d| {"mappers"} / id for KeycloakIdentityProviderMapperSpec => IdentityProviderMapperRepresentation);

impl_instance_ref!(KeycloakIdentityProviderMapper);

schema_patch!(KeycloakIdentityProviderMapperSpec);
