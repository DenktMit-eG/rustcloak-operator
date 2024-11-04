use std::net::SocketAddr;

use clap::{Parser, ValueEnum};
use serde::Serialize;

#[derive(ValueEnum, Clone, Debug, Serialize, PartialEq)]
pub enum ControllerOpt {
    /// handles API requests between the operator and Keycloak
    Api,
    /// handles the oauth session between the operator and Keycloak instances
    Instance,
    /// handles realm creation updates and deletion
    Realm,
    /// handles client creation updates and deletion
    Client,
    /// handles user creation updates and deletion
    User,
    AuthenticationFlow,
    AuthenticatorConfig,
    ClientScope,
    Component,
    Group,
    IdentityProvider,
    IdentityProviderMapper,
    Organization,
    ProtocolMapper,
    RequiredActionProvider,
    Resource,
    Role,
    Scope,
    ClientSecret,
    UserSecret,
}

/// Keyclaok Operator
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[clap(short, long, value_delimiter = ',', num_args = 1.., default_value = "api,instance,realm,client,user,authentication-flow,authenticator-config,client-scope,component,group,identity-provider,identity-provider-mapper,organization,protocol-mapper,required-action-provider,resource,role,scope,client-secret,user-secret")]
    /// Enables the specified controllers. defined as comma seperated list.
    pub controllers: Vec<ControllerOpt>,
    #[clap(short, long)]
    /// if specified, the operator will report metrics and health checks on the specified address.
    /// e.g. --metrics-addr 0.0.0.0:8080
    pub metrics_addr: Option<SocketAddr>,
}
