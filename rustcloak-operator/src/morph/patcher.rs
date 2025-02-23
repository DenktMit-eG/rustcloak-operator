use k8s_openapi::api::core::v1::EnvVar;
use serde::Serialize;
use serde_json::{json, Value};

use crate::{error::Error, util::ObjectPathMut};
use rustcloak_crd::{KeycloakApiPatch, KeycloakApiPatchValueAs};

pub struct Patcher {
    pub value: Value,
    pub vars: Vec<EnvVar>,
}

impl Patcher {
    pub fn new<V: Serialize>(value: V) -> Self {
        let value = serde_json::to_value(value).unwrap();
        Self {
            value,
            vars: Vec::new(),
        }
    }

    pub fn patch(
        &mut self,
        path: &str,
        patch: &KeycloakApiPatch,
    ) -> Result<(), Error> {
        let target = self
            .value
            .path_mut(path)
            .ok_or(Error::PathNotFound(path.to_string()))?;

        let name = format!("var{}", self.vars.len());
        let value_from = patch.source.clone();
        let ty = match (patch.value_as.clone().unwrap_or_default(), &target) {
            (KeycloakApiPatchValueAs::Auto, Value::String(_)) => {
                KeycloakApiPatchValueAs::String
            }
            (KeycloakApiPatchValueAs::Auto, Value::Number(_)) => {
                KeycloakApiPatchValueAs::Number
            }
            (KeycloakApiPatchValueAs::Auto, Value::Object(_)) => {
                KeycloakApiPatchValueAs::Yaml
            }
            (KeycloakApiPatchValueAs::Auto, Value::Array(_)) => {
                KeycloakApiPatchValueAs::Yaml
            }
            (KeycloakApiPatchValueAs::Auto, Value::Bool(_)) => {
                KeycloakApiPatchValueAs::Bool
            }
            x => x.0,
        };
        *target = match ty {
            KeycloakApiPatchValueAs::String => json!({"$str": name}),
            KeycloakApiPatchValueAs::Number => json!({"$nbr": name}),
            KeycloakApiPatchValueAs::Yaml => json!({"$yaml": name}),
            KeycloakApiPatchValueAs::Bool => json!({"$bool": name}),
            KeycloakApiPatchValueAs::Json => json!({"$json": name}),
            KeycloakApiPatchValueAs::Auto => unreachable!(),
        };
        self.vars.push(EnvVar {
            name,
            value_from: Some(value_from),
            value: None,
        });

        Ok(())
    }
}
