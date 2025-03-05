use std::net::SocketAddr;

use clap::builder::PossibleValuesParser;
use clap::{Parser, ValueEnum};
use kube::Resource;
use rustcloak_crd::map_all_crds;

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum LegacyMode {
    /// Do not run legacy controllers
    Disabled,
    /// Run legacy controllers on all resources
    All,
    /// Run legacy controllers on resources that have the rustcloak.k8s.eboland.de/handle: "true"
    /// annotation
    Prudent,
}

pub fn legacy_kinds() -> [String; 4] {
    [
        "LegacyInstance",
        "LegacyRealm",
        "LegacyUser",
        "LegacyClient",
    ]
    .map(String::from)
}

fn controller_values() -> PossibleValuesParser {
    let rustcloak_kinds =
        map_all_crds!(C => C::kind(&()).as_ref().to_string()).into_iter();
    PossibleValuesParser::new(rustcloak_kinds.chain(legacy_kinds().into_iter()))
}

/// Keycloak Operator
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[clap(short, long, value_delimiter = ',', value_parser = controller_values())]
    /// Enables the specified controllers. defined as comma seperated list.
    pub controllers: Vec<String>,
    #[clap(short, long)]
    /// if specified, the operator will report metrics and health checks on the specified address.
    /// e.g. --metrics-addr 0.0.0.0:8080
    pub metrics_addr: Option<SocketAddr>,
    /// Enables the legacy controllers: LegacyInstance, LegacyRealm, LegacyUser, LegacyClient
    #[clap(long, default_value = "disabled")]
    pub legacy: LegacyMode,
}
