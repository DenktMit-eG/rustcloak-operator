use std::collections::HashMap;

use crate::error::{Error, Result};
use k8s_openapi::api::core::v1::{
    ConfigMap, ConfigMapKeySelector, EnvVarSource, Secret,
};
use kube::ResourceExt;
use kube::{Api, Client};
use rustcloak_crd::KeycloakApiObject;
use serde_json::Value;

struct Resolver<'a> {
    client: Client,
    resource: &'a KeycloakApiObject,
    resolved_vars: HashMap<String, String>,
    secret_cache: HashMap<String, Secret>,
    config_map_cache: HashMap<String, ConfigMap>,
}

impl Resolver<'_> {
    fn visit(&self, value: Value) -> Result<Value> {
        use Value::*;
        let vars = &self.resolved_vars;
        Ok(match value {
            Object(obj) => {
                if obj.len() != 1 {
                    self.visit_object(obj)
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
                    self.visit_object(obj)
                }
            }
            Array(arr) => Array(
                arr.into_iter()
                    .map(|v| self.visit(v))
                    .collect::<Result<Vec<Value>>>()?,
            ),
            x => x,
        })
    }

    fn visit_object(&self, obj: serde_json::Map<String, Value>) -> Value {
        let mut new_obj = serde_json::Map::with_capacity(obj.len());
        for (k, v) in obj.into_iter() {
            new_obj.insert(k, self.visit(v).unwrap());
        }
        Value::Object(new_obj)
    }

    async fn resolve_from(
        &mut self,
        name: &str,
        value_from: &EnvVarSource,
    ) -> Result<String> {
        match value_from {
            EnvVarSource {
                config_map_key_ref: Some(r),
                ..
            } => self.resolve_from_config_map(r).await,
            EnvVarSource {
                secret_key_ref: Some(r),
                ..
            } => self.resolve_from_secret(r).await,
            _ => Err(Error::NoValue(name.to_string())),
        }
    }

    async fn resolve_from_config_map(
        &mut self,
        r: &ConfigMapKeySelector,
    ) -> Result<String> {
        let ns = self.resource.namespace().ok_or(Error::NoNamespace)?;
        let api = Api::<ConfigMap>::namespaced(self.client.clone(), &ns);
        let config_map =
            if let Some(config_map) = self.config_map_cache.get(&r.name) {
                config_map
            } else if let Some(config_map) = api.get_opt(&r.name).await? {
                self.config_map_cache.insert(r.name.clone(), config_map);
                self.config_map_cache.get(&r.name).unwrap()
            } else {
                return Err(Error::NoConfigMap(ns, r.name.clone()));
            };
        Ok(config_map
            .data
            .as_ref()
            .and_then(|x| x.get(&r.key))
            .ok_or(Error::NoKeyInConfigMap { r: r.clone() })?
            .clone())
    }

    pub async fn resolve_from_secret(
        &mut self,
        r: &k8s_openapi::api::core::v1::SecretKeySelector,
    ) -> Result<String> {
        let ns = self.resource.namespace().ok_or(Error::NoNamespace)?;
        let api = Api::<Secret>::namespaced(self.client.clone(), &ns);
        let secret = if let Some(secret) = self.secret_cache.get(&r.name) {
            secret
        } else if let Some(secret) = api.get_opt(&r.name).await? {
            self.secret_cache.insert(r.name.clone(), secret);
            self.secret_cache.get(&r.name).unwrap()
        } else {
            return Err(Error::NoConfigMap(ns, r.name.clone()));
        };
        Ok(secret
            .data
            .as_ref()
            .and_then(|x| x.get(&r.key))
            .map(|x| String::from_utf8(x.0.clone()))
            .ok_or(Error::NoKeyInSecret { r: r.clone() })??
            .clone())
    }

    async fn resolve(&mut self) -> Result<Value> {
        for r in self.resource.spec.vars.iter() {
            let value = if let Some(value) = &r.value {
                value.clone()
            } else if let Some(ref value_from) = r.value_from.clone() {
                self.resolve_from(&r.name, value_from).await?
            } else {
                return Err(Error::NoValue(r.name.clone()));
            };
            self.resolved_vars.insert(r.name.clone(), value);
        }

        let payload = serde_yaml::from_str(&self.resource.spec.payload)?;
        self.visit(payload)
    }
}

#[async_trait::async_trait]
pub trait ApiResolver {
    async fn resolve(&self, client: &Client) -> Result<Value>;
}

#[async_trait::async_trait]
impl ApiResolver for KeycloakApiObject {
    async fn resolve(&self, client: &Client) -> Result<Value> {
        let mut resolver = Resolver {
            client: client.clone(),
            resource: self,
            resolved_vars: HashMap::new(),
            secret_cache: HashMap::new(),
            config_map_cache: HashMap::new(),
        };

        resolver.resolve().await
    }
}
