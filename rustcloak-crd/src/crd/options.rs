use k8s_openapi::api::core::v1::{ConfigMapKeySelector, SecretKeySelector};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ValueAs {
    String,
    Number,
    Bool,
    Yaml,
    Json,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ValueFrom {
    SecretKeyRef(SecretKeySelector),
    ConfigMapKeyRef(ConfigMapKeySelector),
    Value(String),
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct ObjectPatchRef {
    pub path: String,
    #[serde(flatten)]
    pub value_from: ValueFrom,
    pub value_as: ValueAs,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakApiObjectOptions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patch_from: Vec<ObjectPatchRef>,
}
