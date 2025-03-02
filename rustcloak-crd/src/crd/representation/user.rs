use crate::keycloak_types::UserRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    crd::namespace_scope,
    impl_object, schema_patch,
    traits::{SecretKeyNames, impl_endpoint},
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::RealmRef;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakUserSecretReference {
    pub secret_name: String,
    pub username_key: Option<String>,
    pub password_key: Option<String>,
}

namespace_scope! {
    "KeycloakUser", "kcu" {
        #[kube(
            doc = "resource to define a User within a [KeyclaokRealm](./keycloakrealm.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
        )]
        /// the KeycloakUser resource
        pub struct KeycloakUserSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: RealmRef,
            #[schemars(schema_with = "schema")]
            pub definition: UserRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
            pub user_secret: Option<KeycloakUserSecretReference>,
        }
    }
}

impl SecretKeyNames<2> for Option<KeycloakUserSecretReference> {
    const DEFAULTS: [&'static str; 2] = ["username", "password"];

    fn secret_key_names_opts(&self) -> Option<[&Option<String>; 2]> {
        self.as_ref().map(|s| [&s.username_key, &s.password_key])
    }
}

impl_object!("user" <RealmRef> / |_d| {"users"} / id for KeycloakUserSpec => UserRepresentation);

impl_endpoint!(KeycloakUser);

schema_patch!(KeycloakUserSpec);
