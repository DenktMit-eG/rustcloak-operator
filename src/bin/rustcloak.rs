use anyhow::Result;
use clap::Parser;
use rustcloak_operator::controller::admin_api_controller::KeycloakAdminApiController;
use rustcloak_operator::controller::controller_runner::ControllerRunner;
use rustcloak_operator::controller::instance_controller::KeycloakInstanceController;
use rustcloak_operator::controller::realm_controller::KeycloakRealmController;
use rustcloak_operator::opts::Opts;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let opts = Opts::parse();
    println!("Hello, world!");
    log::info!("Starting rustcloak-operator");

    let client = kube::Client::try_default().await?;
    let api_controller =
        ControllerRunner::new(KeycloakAdminApiController::new(), &client);
    let instance_controller =
        ControllerRunner::new(KeycloakInstanceController::new(), &client);
    let realm_controller =
        ControllerRunner::new(KeycloakRealmController::new(), &client);
    tokio::try_join!(
        api_controller.run(),
        instance_controller.run(),
        realm_controller.run()
    )?;
    Ok(())
}
