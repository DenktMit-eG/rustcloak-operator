use crate::error::{Error, Result};
use std::{collections::HashMap, fmt::Debug};

use super::{ApiExt, ApiFactory};
use jsonpath_rust::{JsonPath, query::queryable::Queryable};
use k8s_openapi::{
    NamespaceResourceScope,
    api::core::v1::{ConfigMap, Secret, SecretKeySelector},
    serde_json::{self, Value},
};
use kube::Resource;
use rustcloak_crd::{ObjectPatchRef, ValueAs, ValueFrom};
use serde::de::DeserializeOwned;

#[tokio::test]
async fn test_json_patcher() {
    let client = kube::Client::try_default().await.unwrap();
    let mut patcher = JsonPatcher::with_client(client, None);
    let mut obj = serde_json::json!({
      "realm": "blarg",
      "enabled": true,
      "defaultRole": {
        "name": "default-roles-test-1",
        "description": "${role_default-roles}",
        "composite": true,
        "clientRole": false,
        "containerId": "test"
      },
      "identityProviderMappers": [
        {
          "name": "blarg Mapper",
          "identityProviderAlias": "saml",
          "identityProviderMapper": "saml-user-attribute-idp-mapper",
        }
      ]
    });
    let patch = ObjectPatchRef {
        path: "$.enabled".to_string(),
        value_from: ValueFrom::Value("false".to_string()),
        value_as: ValueAs::Bool,
    };

    patcher.patch(&mut obj, &patch).await.unwrap();
    assert_eq!(obj["enabled"], false);

    let patch = ObjectPatchRef {
        path: "$.defaultRole".to_string(),
        value_from: ValueFrom::Value(
            r#"
            name: "default-roles-test-1"
        "#
            .to_string(),
        ),
        value_as: ValueAs::Yaml,
    };

    patcher.patch(&mut obj, &patch).await.unwrap();
    assert_eq!(obj["defaultRole"]["name"], "default-roles-test-1");
}

pub struct JsonPatcher {
    secret_cache: HashMap<String, Secret>,
    config_map_cache: HashMap<String, ConfigMap>,
    client: kube::Client,
    namespace: Option<String>,
}

impl JsonPatcher {
    pub fn with_client(
        client: kube::Client,
        namespace: Option<String>,
    ) -> JsonPatcher {
        JsonPatcher {
            secret_cache: HashMap::new(),
            config_map_cache: HashMap::new(),
            client,
            namespace,
        }
    }

    pub async fn patch(
        &mut self,
        obj: &mut Value,
        patch: &ObjectPatchRef,
    ) -> Result<()> {
        let Some(value) = self.get_value(&patch.value_from).await? else {
            return Ok(());
        };
        let results = obj.query_only_path(&patch.path)?;
        for path in results {
            let Some(mut_ref) = obj.reference_mut(path) else {
                continue;
            };
            match patch.value_as {
                ValueAs::String => {
                    *mut_ref = Value::String(value.clone());
                }
                ValueAs::Number => {
                    *mut_ref = Value::Number(value.parse()?);
                }
                ValueAs::Bool => {
                    let result: bool = serde_yaml::from_str(&value)?;
                    *mut_ref = Value::Bool(result);
                }
                ValueAs::Yaml => *mut_ref = serde_yaml::from_str(&value)?,
                ValueAs::Json => *mut_ref = serde_json::from_str(&value)?,
            }
        }

        Ok(())
    }

    async fn get_value(
        &mut self,
        reference: &ValueFrom,
    ) -> Result<Option<String>> {
        let res = match reference {
            ValueFrom::SecretKeyRef(secret_ref) => {
                self.get_secret_value(secret_ref).await?
            }
            ValueFrom::ConfigMapKeyRef(config_map_ref) => self
                .get_config_map_value(&config_map_ref.name, &config_map_ref.key)
                .await?
                .cloned(),
            ValueFrom::Value(value) => Some(value.clone()),
        };
        Ok(res)
    }

    async fn get_secret_value(
        &mut self,
        selector: &SecretKeySelector,
    ) -> Result<Option<String>> {
        let secret = Self::get_respource(
            &self.client,
            &self.namespace,
            &mut self.secret_cache,
            &selector.name,
        )
        .await?;

        let Some(value) = secret
            .as_ref()
            .and_then(|s| s.data.as_ref())
            .and_then(|x| x.get(&selector.key))
        else {
            if selector.optional.unwrap_or_default() {
                return Ok(None);
            } else {
                return Err(Error::NoKeyInSecret {
                    r: selector.clone(),
                });
            }
        };

        let value = String::from_utf8(value.0.clone())?;

        Ok(Some(value))
    }

    async fn get_config_map_value(
        &mut self,
        name: &str,
        key: &str,
    ) -> Result<Option<&String>> {
        let Some(config_map) = Self::get_respource(
            &self.client,
            &self.namespace,
            &mut self.config_map_cache,
            name,
        )
        .await?
        else {
            return Ok(None);
        };

        let Some(value) = config_map.data.as_ref().and_then(|x| x.get(key))
        else {
            return Ok(None);
        };

        Ok(Some(value))
    }

    async fn get_respource<'a, R>(
        client: &kube::Client,
        namespace: &Option<String>,
        cache: &'a mut HashMap<String, R>,
        name: &str,
    ) -> Result<Option<&'a R>>
    where
        R: Resource<DynamicType = (), Scope = NamespaceResourceScope>
            + DeserializeOwned
            + Clone
            + Debug,
    {
        if !cache.contains_key(name) {
            let api = ApiExt::<R>::api(client.clone(), namespace);
            let secret = api.get(name).await?;
            cache.insert(name.to_string(), secret);
        }
        Ok(cache.get(name))
    }
}
