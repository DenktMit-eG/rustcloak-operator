use crate::traits::Endpoint;
use crate::traits::SecretKeyNames;

use super::KeycloakApiStatus;
use super::KeycloakApiStatusEndpoint;
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

impl SecretKeyNames<2> for KeycloakInstanceCredentialReference {
    const DEFAULTS: [&'static str; 2] = ["user", "password"];

    fn secret_key_names_opts(&self) -> Option<[&Option<String>; 2]> {
        Some([&self.username_key, &self.password_key])
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakInstanceTokenReference {
    pub secret_name: Option<String>,
    pub token_key: Option<String>,
    pub expires_key: Option<String>,
}

impl SecretKeyNames<2> for Option<KeycloakInstanceTokenReference> {
    const DEFAULTS: [&'static str; 2] = ["token", "expires"];

    fn secret_key_names_opts(&self) -> Option<[&Option<String>; 2]> {
        self.as_ref().map(|x| [&x.token_key, &x.expires_key])
    }
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
    pub fn token_secret_ref(&self) -> Option<&KeycloakInstanceTokenReference> {
        self.spec.token.as_ref()
    }

    pub fn token_secret_name(&self) -> String {
        if let Some(name) = self
            .spec
            .token
            .as_ref()
            .and_then(|x| x.secret_name.as_ref())
        {
            name.to_string()
        } else {
            self.name_unchecked() + "-api-token"
        }
    }

    pub fn credential_secret_name(&self) -> &str {
        self.spec.credentials.secret_name.as_str()
    }
}

impl Endpoint for KeycloakInstance {
    fn endpoint(&self) -> Option<&KeycloakApiStatusEndpoint> {
        self.status.as_ref().and_then(|s| s.endpoint.as_ref())
    }
    fn instance_ref(&self) -> Option<&str> {
        self.metadata.name.as_deref()
    }
    fn resource_path(&self) -> Option<&str> {
        Some("")
    }
}
