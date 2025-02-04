use k8s_openapi::api::core::v1::{ConfigMapKeySelector, SecretKeySelector};
use thiserror::Error;

use crate::api::KeycloakApiAuthBuilderError;

type OAuth2TokenError = oauth2::RequestTokenError<
    oauth2::HttpClientError<reqwest::Error>,
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An error occurred")]
    GenericError,
    #[error("Keycloak instance not found: {0}")]
    KeycloakInstanceNotFound(String),
    #[error("No Parent: {0}/{1}")]
    NoParent(String, String),
    #[error("Recursive Parent: {0}/{1}")]
    RecursiveParent(String, String),
    #[error("No Namespace")]
    NoNamespace,
    #[error("{0}")]
    ToStrError(#[from] reqwest::header::ToStrError),
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
    #[error("No Secret")]
    NoSecret(String, String),
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
    #[error("Serde Yaml error: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),
    #[error("Serde Json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("No Uid")]
    NoUid,
    #[error("Oauth Parse error: {0}")]
    OauthParseError(#[from] oauth2::url::ParseError),
    #[error("Oauth Token Request error: {0}")]
    OAuth2TokenError(#[from] OAuth2TokenError),
    #[error("KeycloakApiAuthBuilderError: {0}")]
    KeycloakAuthBuilderError(#[from] KeycloakApiAuthBuilderError),
    #[error("Borrow error: {0}")]
    BorrowError(#[from] std::cell::BorrowError),
    #[error("Send error: {0}")]
    SendError(#[from] tokio::sync::mpsc::error::SendError<()>),
    #[error("Missing Fields: {0}")]
    MissingFields(String),
    #[error("Missing Primary Key")]
    NoPrimaryKey,
    #[error("No Instance in namespace {0} with name {1} found")]
    NoInstance(String, String),
    #[error("No Credential Secret in namespace {0} with name {1} found")]
    NoCredentialSecret(String, String),
    #[error("No Token Secret in namespace {0} with name {1} found")]
    NoTokenSecret(String, String),
    #[error("No Realm in namespace {0} with name {1} found")]
    NoRealm(String, String),
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Unknown Secret Type: {0}")]
    UnknownSecretType(String),
    #[error("No Client Id")]
    NoClientId,
    #[error("No Client Secret")]
    NoClientSecret,
    #[error("Resource not found")]
    NoResource,
    #[error("No Location Header")]
    NoLocationHeader,
    #[error("No Legacy Instance found")]
    LegacyInstanceNotFound,
    #[error("Ambiguous Legacy Instances found")]
    AmbiguousLegacyInstancesFound,
    #[error("Parse Expression error: {0}")]
    ParseExpressionError(#[from] kube::core::ParseExpressionError),
    #[error("Resource in use for deletion: {0:?}")]
    ResourceInUseForDeletion(Vec<String>),
    #[error("Keycloak Error Code {0}: {1}")]
    KeycloakError(reqwest::StatusCode, String),
    #[error("garbage URL found in Keycloak: {0}")]
    GarbageUrlFromKeycloak(String),
    #[error("Path not found: {0}")]
    PathNotFound(String),
    #[error("Invalid patch value as auto: {0}")]
    InvalidPatchValueAsAuto(String),
    #[error("Wait Error: {0}")]
    WaitError(#[from] kube::runtime::wait::Error),
    #[error("No ConfigMap in namespace {0} with name {1} found")]
    NoConfigMap(String, String),
}

pub type Result<T> = std::result::Result<T, Error>;
