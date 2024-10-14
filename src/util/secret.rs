use k8s_openapi::{
    api::core::v1::Secret, apimachinery::pkg::apis::meta::v1::OwnerReference,
    ByteString,
};
use keycloak::KeycloakAdminToken;
use kube::api::ObjectMeta;

use crate::error::{Error, Result};

pub trait SecretUtils {
    fn credentials(&self) -> Result<(String, String)>;
    fn from_token(
        name: &str,
        namespace: &str,
        owner_ref: Option<OwnerReference>,
        token: &KeycloakAdminToken,
    ) -> Self;
    fn token(&self) -> Result<KeycloakAdminToken>;
}

impl SecretUtils for Secret {
    fn credentials(&self) -> Result<(String, String)> {
        let data = self.data.as_ref().ok_or(Error::NoData)?;
        let user = data
            .get("user")
            .and_then(|x| String::from_utf8(x.0.clone()).ok())
            .ok_or(Error::NoUsername)?;
        let password = data
            .get("password")
            .and_then(|x| String::from_utf8(x.0.clone()).ok())
            .ok_or(Error::NoPassword)?;
        Ok((user, password))
    }

    fn from_token(
        name: &str,
        namespace: &str,
        owner_ref: Option<OwnerReference>,
        token: &KeycloakAdminToken,
    ) -> Self {
        let token = ByteString(serde_json::to_vec(token).unwrap());
        Secret {
            metadata: ObjectMeta {
                name: Some(name.to_string()),
                namespace: Some(namespace.to_string()),
                owner_references: Some(vec![owner_ref.unwrap()]),
                ..Default::default()
            },
            data: Some([("token".to_string(), token)].into()),
            ..Default::default()
        }
    }

    fn token(&self) -> Result<KeycloakAdminToken> {
        Ok(self
            .data
            .as_ref()
            .and_then(|data| data.get("token"))
            .map(|data| serde_json::from_slice::<KeycloakAdminToken>(&data.0))
            .ok_or(Error::NoToken)??)
    }
}
