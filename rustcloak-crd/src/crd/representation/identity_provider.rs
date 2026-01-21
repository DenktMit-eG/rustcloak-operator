use crate::keycloak_types::IdentityProviderRepresentation;
use crate::refs::ref_type;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiStatus, crd::namespace_scope,
    impl_object, schema_patch, traits::impl_endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::realm::RealmRef;

namespace_scope! {
    "KeycloakIdentityProvider", "kcip" {
        #[kube(
            doc = "resource to define a identity provider in a [KeyclaokRealm](./keycloakrealm.md)",
        )]
        /// the KeycloakIdentityProvider resource
        pub struct KeycloakIdentityProviderSpec {
            #[serde(default, flatten)]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: RealmRef,
            #[schemars(schema_with = "schema")]
            pub definition: Option<IdentityProviderRepresentation>,
        }
    }
}

impl_object!("idp" <RealmRef> / |_d| {"identity-provider/instances"} / "alias" / |d| {
    d.definition.as_ref().and_then(|def| def.alias.as_deref())
} for KeycloakIdentityProviderSpec => IdentityProviderRepresentation);

impl_endpoint!(KeycloakIdentityProvider);

schema_patch!(KeycloakIdentityProviderSpec);

ref_type!(
    IdentityProviderRef,
    identity_provider_ref,
    KeycloakIdentityProvider,
    "the name of the kubernetes object that created the identity provider."
);
