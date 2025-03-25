use k8s_openapi::api::core::v1::{ConfigMapKeySelector, SecretKeySelector};
use rustcloak_crd::InstanceRef;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Keycloak Error: {0}")]
    KeycloakClient(#[from] keycloak_client::Error),
    #[error("{0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Missing Field in Resource: {0}")]
    MissingField(String),
    #[error("Missing Resource Path")]
    MissingResourcePath,
    #[error("Missing Instance Reference")]
    MissingInstanceReference,
    #[error("No username")]
    NoUsername,
    #[error("No var {0}")]
    NoVar(String),
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
    #[error("Serde Yaml error: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),
    #[error("Serde Json error: {0}")]
    SerdeJsonError(#[from] k8s_openapi::serde_json::Error),
    #[error("Borrow error: {0}")]
    BorrowError(#[from] std::cell::BorrowError),
    #[error("Send error: {0}")]
    SendError(#[from] tokio::sync::mpsc::error::SendError<()>),
    #[error("No Instance in namespace {0:?} with name {1:?} found")]
    NoInstance(Option<String>, InstanceRef),
    #[error("No Credential Secret in namespace {0} with name {1} found")]
    NoCredentialSecret(String, String),
    #[error("No Token Secret in namespace {0} with name {1} found")]
    NoTokenSecret(String, String),
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Unknown Secret Type returned by Keycloak: {0}")]
    UnknownSecretType(String),
    #[error("No Client Id")]
    NoClientId,
    #[error("No Legacy Instance found")]
    LegacyInstanceNotFound,
    #[error("Ambiguous Legacy Instances found")]
    AmbiguousLegacyInstancesFound,
    #[error("Parse Expression error: {0}")]
    ParseExpressionError(#[from] kube::core::ParseExpressionError),
    #[error("Resource in use for deletion: {0:?}")]
    ResourceInUseForDeletion(Vec<String>),
    #[error("Path not found: {0}")]
    PathNotFound(String),
    #[error("Wait Error: {0}")]
    WaitError(#[from] kube::runtime::wait::Error),
    #[error("No Client Id was provided by Keycloak")]
    MissingClientId,
    #[error("No ConfigMap in namespace {0} with name {1} found")]
    NoConfigMap(String, String),
    #[error("No Resource Path")]
    NoResourcePath,
    #[error("Unsupported Workflow Method")]
    UnsupportedWorkflowMethod,
    #[error("Prometheus Error: {0}")]
    Prometheus(#[from] prometheus::Error),
    #[error("JsonPath Error: {0}")]
    JsonPathError(#[from] jsonpath_rust::parser::errors::JsonPathError),
    #[error("Cannot request client secret from Keycloak")]
    CannotRequestClientSecret,
}

pub type Result<T> = std::result::Result<T, Error>;
