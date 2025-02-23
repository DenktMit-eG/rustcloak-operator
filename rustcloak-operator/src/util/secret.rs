use std::collections::BTreeMap;

use k8s_openapi::{api::core::v1::Secret, ByteString};
use kube::api::ObjectMeta;
use kube::{Resource, ResourceExt};

use crate::{
    api::OAuth2Token,
    error::{Error, Result},
};
use rustcloak_crd::{traits::SecretKeyNames, KeycloakInstance};

pub trait SecretUtils {
    fn from_token(token: &OAuth2Token, instance: &KeycloakInstance) -> Self;
    fn token(&self, secret_ref: &KeycloakInstance) -> Result<OAuth2Token>;

    fn extract<const N: usize>(
        &self,
        key_names: &impl SecretKeyNames<N>,
    ) -> Result<[String; N]>;
}

impl SecretUtils for Secret {
    fn from_token(
        oauth_token: &OAuth2Token,
        instance: &KeycloakInstance,
    ) -> Self {
        let token = ByteString(
            serde_yaml::to_string(&oauth_token.token)
                .unwrap()
                .into_bytes(),
        );
        let expires = oauth_token
            .expires
            .map(|x| ByteString(x.to_rfc3339().into_bytes()));
        let name = instance.token_secret_name();
        let (token_key, expires_key) = instance.token_secret_keys();
        let namespace = instance.namespace().unwrap();
        let owner_ref = instance.owner_ref(&()).unwrap();
        let mut data: BTreeMap<String, ByteString> =
            [(token_key.to_string(), token)].into();
        if let Some(expires) = expires {
            data.insert(expires_key.to_string(), expires);
        }
        Secret {
            metadata: ObjectMeta {
                name: Some(name),
                namespace: Some(namespace),
                owner_references: Some(vec![owner_ref]),
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
        let token = serde_yaml::from_slice(&token.ok_or(Error::NoToken)?.0)?;
        let expires = expires
            .and_then(|e| String::from_utf8(e.0.clone()).ok())
            .and_then(|e| chrono::DateTime::parse_from_rfc3339(&e).ok())
            .map(|e| e.with_timezone(&chrono::Utc));
        Ok(OAuth2Token { token, expires })
    }

    fn extract<const N: usize>(
        &self,
        key_names: &impl SecretKeyNames<N>,
    ) -> Result<[String; N]> {
        let key_names = key_names.secret_key_names();
        let data = self.data.as_ref().ok_or(Error::NoData)?;
        let vec = key_names
            .iter()
            .map(|k| {
                if let Some(v) = data.get(*k) {
                    String::from_utf8(v.0.clone()).map_err(Error::from)
                } else {
                    Err(Error::MissingFields(k.to_string()))
                }
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(vec.try_into().unwrap())
    }
}
