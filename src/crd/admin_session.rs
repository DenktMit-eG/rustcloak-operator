use super::KeycloakApiStatus;
use super::WithStatus;
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakAdminSession",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
pub struct KeycloakAdminSessionSpec {
    pub token_secret_name: String,
    pub valid_until: chrono::DateTime<chrono::Utc>,
    pub next_keepalive: chrono::DateTime<chrono::Utc>,
}

impl WithStatus<KeycloakApiStatus> for KeycloakAdminSession {
    fn status(&self) -> Option<&KeycloakApiStatus> {
        self.status.as_ref()
    }
}
