use clap::Parser;

/// Keyclaok Operator
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Opts {
    /// Enables CRD handling for the keycloak-realms operator
    #[clap(short, long, env)]
    deprecated: bool,
}
