use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Enables CRD handling for the keycloak-realms operator
    #[clap(short, long)]
    deprecated: bool,
}
