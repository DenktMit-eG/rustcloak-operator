use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    crd::namespace_scope, impl_object, schema_patch, traits::impl_endpoint,
};

use crate::keycloak_types::AuthenticatorConfigRepresentation;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::RealmRef;

namespace_scope! {
    "KeycloakAuthenticatorConfig", "kcac" {
        #[kube(
            doc = "resource to define an Authenticator Config within a [KeycloakRealm](./keycloakrealm.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1beta1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
        )]
        /// the KeycloakAuthenticatorConfig resource
        pub struct KeycloakAuthenticatorConfigSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: RealmRef,
            #[schemars(schema_with = "schema")]
            pub definition: AuthenticatorConfigRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_object!("authconfig" <RealmRef> / |_d| {"authentication/config"} / id for KeycloakAuthenticatorConfigSpec => AuthenticatorConfigRepresentation);

impl_endpoint!(KeycloakAuthenticatorConfig);

schema_patch!(KeycloakAuthenticatorConfigSpec);
