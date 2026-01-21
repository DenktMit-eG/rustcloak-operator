mod auth;
mod import;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustcloak-cli")]
#[command(about = "CLI tool for rustcloak-operator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Import existing Keycloak resources as rustcloak CRDs
    Import(ImportArgs),
}

#[derive(Parser)]
pub struct ImportArgs {
    /// Keycloak base URL (e.g., http://localhost:8080)
    #[arg(long, env = "KEYCLOAK_URL")]
    keycloak_url: String,

    /// Kubernetes namespace for created resources
    #[arg(short, long, default_value = "default")]
    namespace: String,

    /// Name of the KeycloakInstance/ClusterKeycloakInstance resource
    #[arg(long)]
    instance_name: String,

    /// Use ClusterKeycloakInstance instead of KeycloakInstance
    #[arg(long)]
    cluster_instance: bool,

    /// Only import these realms (comma-separated)
    #[arg(long, value_delimiter = ',')]
    realms: Vec<String>,

    /// Include default Keycloak objects (default clients, scopes, flows, etc.)
    #[arg(long)]
    include_defaults: bool,

    /// Output YAML to stdout instead of applying to Kubernetes
    #[arg(long)]
    dry_run: bool,

    #[command(flatten)]
    auth: AuthArgs,
}

#[derive(Parser)]
pub struct AuthArgs {
    /// Name of Kubernetes secret containing credentials (keys: 'user', 'password')
    #[arg(long, group = "auth_source")]
    credential_secret: Option<String>,

    /// Keycloak admin username
    #[arg(long, env = "KEYCLOAK_USER", requires = "password")]
    username: Option<String>,

    /// Keycloak admin password
    #[arg(long, env = "KEYCLOAK_PASSWORD", requires = "username")]
    password: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Import(args) => import::run_import(args).await?,
    }

    Ok(())
}
