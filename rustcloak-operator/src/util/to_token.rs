use k8s_openapi::api::core::v1::Secret;
use keycloak::KeycloakAdminToken;

use crate::error::{Error, Result};

pub trait ToToken {
    fn to_token(&self) -> Result<KeycloakAdminToken>;
}

impl ToToken for Secret {
    fn to_token(&self) -> Result<KeycloakAdminToken> {
        Ok(self
            .data
            .as_ref()
            .and_then(|data| data.get("token"))
            .map(|data| serde_json::from_slice::<KeycloakAdminToken>(&data.0))
            .ok_or(Error::NoToken)??)
    }
}
