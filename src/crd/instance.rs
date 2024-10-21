use super::KeycloakApiStatus;
use super::WithStatus;
use kube::ResourceExt;
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct KeycloakInstanceCredentialReference {
    pub secret_name: String,
    pub user_key: Option<String>,
    pub password_key: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct KeycloakInstanceTokenReference {
    pub secret_name: Option<String>,
    pub token_key: Option<String>,
    pub expires_key: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct KeycloakInstanceClient {
    pub id: String,
    pub secret: Option<String>,
}

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakInstance",
    shortname = "kci",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
pub struct KeycloakInstanceSpec {
    pub base_url: String,
    pub realm: Option<String>,
    pub credentials: KeycloakInstanceCredentialReference,
    pub token: Option<KeycloakInstanceTokenReference>,
    pub client: Option<KeycloakInstanceClient>,
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Serialize,
    JsonSchema,
    Hash,
    Eq,
    PartialEq,
    Default,
)]
pub struct KeycloakInstanceSelector {
    pub name: String,
}

impl WithStatus<KeycloakApiStatus> for KeycloakInstance {
    fn status(&self) -> Option<&KeycloakApiStatus> {
        self.status.as_ref()
    }
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
