use super::{KeycloakApiStatus, both_scopes};
use crate::{
    either::UntaggedEither,
    marker::{HasMarker, ResourceMarker},
    realm::RealmRef,
    refs::ref_type,
    traits::{Endpoint, SecretKeyNames},
};
use kube::{CustomResource, Resource};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakInstanceCredentialReference {
    /// If true, the operator will create the secret with a randomly generated password if it doesn't exist.
    pub create: Option<bool>,
    /// Name of the Kubernetes Secret containing the credentials.
    pub secret_name: String,
    /// Key in the secret containing the username. Defaults to "user".
    pub username_key: Option<String>,
    /// Key in the secret containing the password. Defaults to "password".
    pub password_key: Option<String>,
}

impl SecretKeyNames<2> for KeycloakInstanceCredentialReference {
    const DEFAULTS: [&'static str; 2] = ["user", "password"];

    fn secret_key_names_opts(&self) -> [&Option<String>; 2] {
        [&self.username_key, &self.password_key]
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakInstanceTokenReference {
    /// Name of the Kubernetes Secret for caching the authentication token. If not specified, defaults to "{instance-name}-api-token".
    pub secret_name: Option<String>,
    /// Key in the secret for storing the token. Defaults to "token".
    pub token_key: Option<String>,
    /// Key in the secret for storing the token expiration timestamp. Defaults to "expires".
    pub expires_key: Option<String>,
}

impl SecretKeyNames<2> for KeycloakInstanceTokenReference {
    const DEFAULTS: [&'static str; 2] = ["token", "expires"];

    fn secret_key_names_opts(&self) -> [&Option<String>; 2] {
        [&self.token_key, &self.expires_key]
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakInstanceClient {
    /// Client ID for service account authentication.
    pub id: String,
    /// Client secret for service account authentication. Optional if the client is public.
    pub secret: Option<String>,
}

both_scopes! {
    "KeycloakInstance", "kci", "ClusterKeycloakInstance", "ckci", ClusterKeycloakInstanceSpec {
        #[kube(
            doc = "This resource makes a Keycloak instance known to the operator",
            printcolumn = r#"{
                    "name":"Base URL",
                    "type":"string",
                    "description":"The base URL of the Keycloak instance",
                    "jsonPath":".spec.baseUrl"
                }"#,
        )]
        pub struct KeycloakInstanceSpec {
            /// Base URL of the Keycloak server (e.g., "https://keycloak.example.com").
            pub base_url: String,
            /// Realm used for authentication. Defaults to "master".
            pub realm: Option<String>,
            /// Reference to the Kubernetes Secret containing admin credentials.
            pub credentials: KeycloakInstanceCredentialReference,
            /// Optional configuration for caching the authentication token in a Secret.
            pub token: Option<KeycloakInstanceTokenReference>,
            /// Optional client credentials for service account authentication instead of username/password.
            pub client: Option<KeycloakInstanceClient>,
        }
    }
}

impl KeycloakInstanceSpec {
    pub fn token_secret_ref(&self) -> Option<&KeycloakInstanceTokenReference> {
        self.token.as_ref()
    }

    pub fn token_secret_name(&self, name: &str) -> String {
        if let Some(name) =
            self.token.as_ref().and_then(|x| x.secret_name.as_ref())
        {
            name.to_string()
        } else {
            format!("{name}-api-token")
        }
    }

    pub fn credential_secret_name(&self) -> &str {
        self.credentials.secret_name.as_str()
    }
}

macro_rules! instance_endpoint {
    ($name:ident, $either:ident) => {
        impl Endpoint for $name {
            fn is_ready(&self) -> bool {
                self.status.as_ref().map_or(false, |s| s.ready)
            }
            fn instance_ref(&self) -> Option<&InstanceRef> {
                None
            }
            fn realm_ref(&self) -> Option<RealmRef> {
                None
            }
            fn resource_path(&self) -> Option<&str> {
                Some("")
            }
        }
    };
}

instance_endpoint!(KeycloakInstance, Left);
instance_endpoint!(ClusterKeycloakInstance, Right);

impl HasMarker for KeycloakInstance {
    type Marker = ResourceMarker<<Self as Resource>::Scope>;
}

impl HasMarker for ClusterKeycloakInstance {
    type Marker = ResourceMarker<<Self as Resource>::Scope>;
}

ref_type!(
    NamespacedInstanceRef,
    instance_ref,
    KeycloakInstance,
    "The name of the namespaced instance to which this object belongs to."
);
ref_type!(
    ClusterInstanceRef,
    cluster_instance_ref,
    ClusterKeycloakInstance,
    "The name of the cluster instance to which this object belongs to."
);
pub type InstanceRef =
    UntaggedEither<NamespacedInstanceRef, ClusterInstanceRef>;
