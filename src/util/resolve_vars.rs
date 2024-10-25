use std::collections::HashMap;

use crate::{
    crd::KeycloakApiObject,
    error::{Error, Result},
};
use k8s_openapi::api::core::v1::{
    ConfigMap, ConfigMapKeySelector, EnvVarSource, Secret,
};
use kube::ResourceExt;
use kube::{Api, Client};
use serde_json::Value;

impl KeycloakApiObject {
    pub async fn resolve(&self, client: &Client) -> Result<Value> {
        let mut resolved_vars = HashMap::new();

        for r in self.spec.vars.iter() {
            let value = if let Some(value) = &r.value {
                value.clone()
            } else if let Some(ref value_from) = r.value_from.clone() {
                self.resolve_from(client, &r.name, value_from).await?
            } else {
                return Err(Error::NoValue(r.name.clone()));
            };
            resolved_vars.insert(r.name.clone(), value);
        }

        Self::visit(&self.spec.payload, &resolved_vars)
    }

    fn visit(value: &Value, vars: &HashMap<String, String>) -> Result<Value> {
        use Value::*;
        Ok(match value {
            Object(obj) => {
                if obj.len() != 1 {
                    Self::visit_object(obj, vars)
                } else if let Some(String(str)) = obj.get("$str") {
                    String(
                        vars.get(str).ok_or(Error::NoVar(str.clone()))?.clone(),
                    )
                } else if let Some(String(nbr)) = obj.get("$nbr") {
                    Number(
                        vars.get(nbr)
                            .ok_or(Error::NoVar(nbr.clone()))?
                            .parse()?,
                    )
                } else if let Some(String(obj)) = obj.get("$json") {
                    Object(serde_json::from_str(
                        vars.get(obj).ok_or(Error::NoVar(obj.clone()))?,
                    )?)
                } else if let Some(String(obj)) = obj.get("$yaml") {
                    Object(serde_yaml::from_str(
                        vars.get(obj).ok_or(Error::NoVar(obj.clone()))?,
                    )?)
                } else {
                    Self::visit_object(obj, vars)
                }
            }
            Array(arr) => Array(
                arr.iter()
                    .map(|v| Self::visit(v, vars))
                    .collect::<Result<Vec<Value>>>()?,
            ),
            x => x.clone(),
        })
    }

    fn visit_object(
        obj: &serde_json::Map<String, Value>,
        vars: &HashMap<String, String>,
    ) -> Value {
        let mut new_obj = serde_json::Map::with_capacity(obj.len());
        for (k, v) in obj.iter() {
            new_obj.insert(k.clone(), Self::visit(v, vars).unwrap());
        }
        Value::Object(new_obj)
    }

    async fn resolve_from(
        &self,
        client: &Client,
        name: &str,
        value_from: &EnvVarSource,
    ) -> Result<String> {
        match value_from {
            EnvVarSource {
                config_map_key_ref: Some(r),
                ..
            } => self.resolve_from_config_map(client, r).await,
            EnvVarSource {
                secret_key_ref: Some(r),
                ..
            } => self.resolve_from_secret(client, r).await,
            _ => Err(Error::NoValue(name.to_string())),
        }
    }

    async fn resolve_from_config_map(
        &self,
        client: &Client,
        r: &ConfigMapKeySelector,
    ) -> Result<String> {
        let ns = self.namespace().ok_or(Error::NoNamespace)?;
        let api = Api::<ConfigMap>::namespaced(client.clone(), &ns);
        let config_map = api.get(&r.name).await?;
        Ok(config_map
            .data
            .as_ref()
            .and_then(|x| x.get(&r.key))
            .ok_or(Error::NoKeyInConfigMap { r: r.clone() })?
            .clone())
    }

    pub async fn resolve_from_secret(
        &self,
        client: &Client,
        r: &k8s_openapi::api::core::v1::SecretKeySelector,
    ) -> Result<String> {
        let ns = self.namespace().ok_or(Error::NoNamespace)?;
        let api = Api::<Secret>::namespaced(client.clone(), &ns);
        let secret = api.get(&r.name).await?;
        Ok(secret
            .data
            .as_ref()
            .and_then(|x| x.get(&r.key))
            .map(|x| String::from_utf8(x.0.clone()))
            .ok_or(Error::NoKeyInSecret { r: r.clone() })??
            .clone())
    }
}
