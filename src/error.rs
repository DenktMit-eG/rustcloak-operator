use k8s_openapi::api::core::v1::{ConfigMapKeySelector, SecretKeySelector};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An error occurred")]
    GenericError,
    #[error("Keycloak instance not found: {0}")]
    KeycloakInstanceNotFound(String),
    #[error("No Namespace")]
    NoNamespace,
    #[error("{0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("No data")]
    NoData,
    #[error("No username")]
    NoUsername,
    #[error("No var {0}")]
    NoVar(String),
    #[error("No password")]
    NoPassword,
    #[error("No token")]
    NoToken,
    #[error("No key {} in Secret {}", r.key, r.name)]
    NoKeyInSecret { r: SecretKeySelector },
    #[error("No key {} in ConfigMap {}", r.key, r.name)]
    NoKeyInConfigMap { r: ConfigMapKeySelector },
    #[error("Kube error: {0}")]
    NoValue(String),
    #[error("Kube error: {0}")]
    KubeError(#[from] kube::Error),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Keycloak error: {0}")]
    KeycloakError(#[from] keycloak::KeycloakError),
    #[error("Serde Yaml error: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),
    #[error("Serde Json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
