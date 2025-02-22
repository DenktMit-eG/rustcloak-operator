use crate::traits::InstanceRef;
use crate::traits::SecretKeyNames;

use super::KeycloakApiStatus;
use kube::CustomResource;
use kube::ResourceExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakInstanceCredentialReference {
    pub create: Option<bool>,
    pub secret_name: String,
    pub username_key: Option<String>,
    pub password_key: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakInstanceTokenReference {
    pub secret_name: Option<String>,
    pub token_key: Option<String>,
    pub expires_key: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakInstanceClient {
    pub id: String,
    pub secret: Option<String>,
}

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakInstance",
    shortname = "kci",
    doc = "This resource makes a Keycloak instance known to the operator",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    category = "keycloak",
    category = "all",
    namespaced,
    printcolumn = r#"{
            "name":"Base URL",
            "type":"string",
            "description":"The base URL of the Keycloak instance",
            "jsonPath":".spec.baseUrl"
        }"#,
    printcolumn = r#"{
            "name":"Ready",
            "type":"boolean",
            "description":"true if the realm is ready",
            "jsonPath":".status.ready"
        }"#,
    printcolumn = r#"{
            "name":"Status",
            "type":"string",
            "description":"Status String of the resource",
            "jsonPath":".status.status"
        }"#
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakInstanceSpec {
    pub base_url: String,
    pub realm: Option<String>,
    pub credentials: KeycloakInstanceCredentialReference,
    pub token: Option<KeycloakInstanceTokenReference>,
    pub client: Option<KeycloakInstanceClient>,
}

impl KeycloakInstance {
    fn token_secret_ref(&self) -> Option<&KeycloakInstanceTokenReference> {
        self.spec.token.as_ref()
    }

    pub fn token_secret_name(&self) -> String {
        if let Some(name) = self
            .token_secret_ref()
            .and_then(|x| x.secret_name.as_deref())
        {
            name.to_string()
        } else {
            self.name_unchecked() + "-api-token"
        }
    }

    pub fn token_secret_keys(&self) -> (&str, &str) {
        let (token_key, expire_key) =
            self.token_secret_ref().map_or((None, None), |x| {
                (x.token_key.as_deref(), x.expires_key.as_deref())
            });
        let token_key = token_key.unwrap_or("token");
        let expires_key = expire_key.unwrap_or("expires");
        (token_key, expires_key)
    }

    pub fn credential_secret_name(&self) -> &str {
        self.spec.credentials.secret_name.as_str()
    }
}

impl InstanceRef for KeycloakInstance {
    fn instance_ref(&self) -> Option<&str> {
        self.metadata.name.as_deref()
    }
    fn resource_path(&self) -> Option<&str> {
        Some("")
    }
}

impl SecretKeyNames<2> for KeycloakInstance {
    const DEFAULTS: [&'static str; 2] = ["user", "password"];

    fn secret_key_names_opts(&self) -> Option<[&Option<String>; 2]> {
        let cred = &self.spec.credentials;
        Some([&cred.username_key, &cred.password_key])
    }
}
