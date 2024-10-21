use std::collections::BTreeMap;

use k8s_openapi::{api::core::v1::Secret, ByteString};
use kube::api::ObjectMeta;
use kube::ResourceExt;

use crate::crd::KeycloakInstanceCredentialReference;
use crate::{
    api::OAuth2Token,
    crd::KeycloakInstance,
    error::{Error, Result},
};

pub trait SecretUtils {
    fn credentials(
        &self,
        instance: &KeycloakInstanceCredentialReference,
    ) -> Result<(String, String)>;
    fn from_token(token: &OAuth2Token, instance: &KeycloakInstance) -> Self;
    fn token(&self, secret_ref: &KeycloakInstance) -> Result<OAuth2Token>;
}

impl SecretUtils for Secret {
    fn credentials(
        &self,
        secret_ref: &KeycloakInstanceCredentialReference,
    ) -> Result<(String, String)> {
        let (user_key, password_key) = secret_ref.key_names();
        let data = self.data.as_ref().ok_or(Error::NoData)?;
        let user = data
            .get(user_key)
            .and_then(|u| String::from_utf8(u.0.clone()).ok())
            .ok_or(Error::NoUsername)?;
        let password = data
            .get(password_key)
            .and_then(|p| String::from_utf8(p.0.clone()).ok())
            .ok_or(Error::NoPassword)?;
        Ok((user, password))
    }

    fn from_token(
        oauth_token: &OAuth2Token,
        instance: &KeycloakInstance,
    ) -> Self {
        let token =
            ByteString(serde_json::to_vec_pretty(&oauth_token.token).unwrap());
        let expires = oauth_token
            .expires
            .map(|x| ByteString(x.to_rfc3339().into_bytes()));
        let name = instance.token_secret_name();
        let (token_key, expires_key) = instance.token_secret_keys();
        let namespace = instance.namespace().unwrap();
        //let owner_ref = instance.owner_ref(&()).unwrap();
        let mut data: BTreeMap<String, ByteString> =
            [(token_key.to_string(), token)].into();
        if let Some(expires) = expires {
            data.insert(expires_key.to_string(), expires);
        }
        Secret {
            metadata: ObjectMeta {
                name: Some(name),
                namespace: Some(namespace),
                //owner_references: Some(vec![owner_ref]),
                ..Default::default()
            },
            data: Some(data),
            ..Default::default()
        }
    }

    fn token(&self, instance: &KeycloakInstance) -> Result<OAuth2Token> {
        let (token_key, expires_key) = instance.token_secret_keys();
        let (token, expires) =
            self.data.as_ref().map_or((None, None), |data| {
                (data.get(token_key), data.get(expires_key))
            });
        let token = serde_json::from_slice(&token.ok_or(Error::NoToken)?.0)?;
        let expires = expires
            .and_then(|e| String::from_utf8(e.0.clone()).ok())
            .and_then(|e| chrono::DateTime::parse_from_rfc3339(&e).ok())
            .map(|e| e.with_timezone(&chrono::Utc));
        Ok(OAuth2Token { token, expires })
    }
}
