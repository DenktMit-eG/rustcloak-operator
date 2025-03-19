use crate::InitWorkflow;
use crate::either::UntaggedEither;
use crate::keycloak_types::UserRepresentation;
use crate::refs::ref_type;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiStatus,
    crd::namespace_scope,
    impl_object, schema_patch,
    traits::{SecretKeyNames, impl_endpoint},
};
use either::Either;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{ClientRef, RealmRef};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakUserSecretReference {
    pub secret_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password_key: Option<String>,
}

namespace_scope! {
    "KeycloakUser", "kcu" {
        #[kube(
            doc = "resource to define a User within a [KeyclaokRealm](./keycloakrealm.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1beta1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
        )]
        /// the KeycloakUser resource
        pub struct KeycloakUserSpec {
            #[serde(default, flatten)]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: ParentRef,
            #[schemars(schema_with = "schema")]
            pub definition: Option<UserRepresentation>,
            pub user_secret: Option<KeycloakUserSecretReference>,
        }
    }
}

impl SecretKeyNames<3> for KeycloakUserSecretReference {
    const DEFAULTS: [&'static str; 3] = ["username", "password", "email"];

    fn secret_key_names_opts(&self) -> [&Option<String>; 3] {
        [&self.username_key, &self.password_key, &self.email_key]
    }
}

ref_type!(
    UserRef,
    user_ref,
    KeycloakUser,
    "The name of a KeycloakUser resource"
);

pub type ParentRef = UntaggedEither<RealmRef, ClientRef>;
impl ParentRef {
    pub fn with_realm(realm: RealmRef) -> Self {
        Self {
            inner: Either::Left(realm),
        }
    }
    pub fn with_client(client: &str) -> Self {
        Self {
            inner: Either::Right(ClientRef::from(client.to_string())),
        }
    }
}
impl_object!("user" <ParentRef> / |d| {
    if d.parent_ref.is_left() {
        "users".into()
    } else {
        InitWorkflow { workflow: http::Method::GET, mount_path: "service-account-user"}
    }
} / "id" for KeycloakUserSpec => UserRepresentation);

impl_endpoint!(KeycloakUser);

schema_patch!(KeycloakUserSpec);
