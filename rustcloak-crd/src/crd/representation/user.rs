use super::{client::ClientRef, realm::RealmRef};
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

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakUserSecretReference {
    /// If set to false, rustcloak will not create the secret if it does not exist.
    /// Rustcloak will wait for the secret to be created by the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    /// Name of the Kubernetes Secret where the user credentials will be stored.
    pub secret_name: String,
    /// Key in the secret for storing the user's email. Defaults to "email".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_key: Option<String>,
    /// Key in the secret for storing the username. Defaults to "username".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username_key: Option<String>,
    /// Key in the secret for storing the password. Defaults to "password".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password_key: Option<String>,
}

namespace_scope! {
    "KeycloakUser", "kcu" {
        #[kube(
            doc = "resource to define a User within a [KeycloakRealm](./keycloakrealm.md)",
        )]
        /// the KeycloakUser resource
        pub struct KeycloakUserSpec {
            /// API options for configuring patches and other operational settings.
            #[serde(default, flatten)]
            pub options: Option<KeycloakApiObjectOptions>,
            /// Reference to the parent realm (KeycloakRealm/ClusterKeycloakRealm) or client (for service account users).
            #[serde(flatten)]
            pub parent_ref: ParentRef,
            /// The Keycloak user configuration. See Keycloak Admin REST API documentation for available fields.
            #[schemars(schema_with = "schema")]
            pub definition: Option<UserRepresentation>,
            /// Optional reference to a Kubernetes Secret for storing generated user credentials.
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
} / "id" / |d| {
    d.definition.as_ref().and_then(|def| def.username.as_deref())
} for KeycloakUserSpec => UserRepresentation);

impl_endpoint!(KeycloakUser);

schema_patch!(KeycloakUserSpec);
