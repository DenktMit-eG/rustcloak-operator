use crate::endpoint::query::Query;

use super::KeycloakApiStatus;
use kube::CustomResource;
use kube::ResourceExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use up_impl::HasQuery;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakInstanceCredentialReference {
    pub secret_name: String,
    pub user_key: Option<String>,
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
    namespaced,
    printcolumn = r#"{
            "name":"Base URL",
            "type":"string",
            "description":"",
            "jsonPath":".spec.baseUrl"
        }"#,
    printcolumn = r#"{
            "name":"Ready",
            "type":"boolean",
            "description":"",
            "jsonPath":".status.ready"
        }"#,
    printcolumn = r#"{
            "name":"Status",
            "type":"string",
            "description":"",
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

impl KeycloakInstanceCredentialReference {
    pub fn key_names(&self) -> (&str, &str) {
        let user_key = self.user_key.as_ref().map_or("user", |x| x);
        let password_key = self.password_key.as_ref().map_or("password", |x| x);
        (user_key, password_key)
    }
}

impl HasQuery for KeycloakInstance {
    type Query = Query<Self, String>;
}
