use crate::keycloak_types::IdentityProviderRepresentation;
use crate::refs::ref_type;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    crd::namespace_scope, impl_object, schema_patch, traits::impl_endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::RealmRef;

namespace_scope! {
    "KeycloakIdentityProvider", "kcip" {
        #[kube(
            doc = "resource to define a identity provider in a [KeyclaokRealm](./keycloakrealm.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1beta1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
        )]
        /// the KeycloakIdentityProvider resource
        pub struct KeycloakIdentityProviderSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: RealmRef,
            #[schemars(schema_with = "schema")]
            pub definition: IdentityProviderRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_object!("idp" <RealmRef> / |_d| {"identity-provider/instances"} / alias for KeycloakIdentityProviderSpec => IdentityProviderRepresentation);

impl_endpoint!(KeycloakIdentityProvider);

schema_patch!(KeycloakIdentityProviderSpec);

ref_type!(
    IdentityProviderRef,
    identity_provider_ref,
    KeycloakIdentityProvider,
    "the name of the kubernetes object that created the identity provider."
);
