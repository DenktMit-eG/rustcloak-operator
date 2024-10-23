use clap::{Parser, ValueEnum};
use serde::Serialize;

#[derive(ValueEnum, Clone, Debug, Serialize, PartialEq)]
pub enum ControllerOpt {
    AdminApi,
    Instance,
    Realm,
    Client,
    User,
}

/// Keyclaok Operator
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[clap(short, long, value_delimiter = ',', num_args = 1.., default_value = "admin-api,instance,realm,client,user")]
    pub controllers: Vec<ControllerOpt>,
}
