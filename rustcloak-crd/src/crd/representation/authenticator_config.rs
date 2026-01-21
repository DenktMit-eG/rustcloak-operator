use super::realm::RealmRef;
use crate::keycloak_types::AuthenticatorConfigRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiStatus, crd::namespace_scope,
    impl_object, schema_patch, traits::impl_endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakAuthenticatorConfig", "kcac" {
        #[kube(
            doc = "resource to define an Authenticator Config within a [KeycloakRealm](./keycloakrealm.md)",
        )]
        /// the KeycloakAuthenticatorConfig resource
        pub struct KeycloakAuthenticatorConfigSpec {
            #[serde(default, flatten)]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: RealmRef,
            #[schemars(schema_with = "schema")]
            pub definition: Option<AuthenticatorConfigRepresentation>,
        }
    }
}

impl_object!("authconfig" <RealmRef> / |_d| {"authentication/config"} / "id" / |d| {
    d.definition.as_ref().and_then(|def| def.alias.as_deref())
} for KeycloakAuthenticatorConfigSpec => AuthenticatorConfigRepresentation);

impl_endpoint!(KeycloakAuthenticatorConfig);

schema_patch!(KeycloakAuthenticatorConfigSpec);
