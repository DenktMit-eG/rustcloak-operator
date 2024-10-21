use anyhow::Result;
use clap::Parser;
use futures::FutureExt;
use rustcloak_operator::controller::ControllerRunner;
use rustcloak_operator::controller::KeycloakApiObjectController;
use rustcloak_operator::controller::KeycloakInstanceController;
use rustcloak_operator::controller::KeycloakRealmController;
use rustcloak_operator::opts::ControllerOpt;
use rustcloak_operator::opts::Opts;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    pretty_env_logger::init();

    let client = kube::Client::try_default().await?;
    let mut controllers = vec![];
    if opts.controllers.contains(&ControllerOpt::AdminApi) {
        controllers.push(
            ControllerRunner::new(
                KeycloakApiObjectController::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts.controllers.contains(&ControllerOpt::Instance) {
        controllers.push(
            ControllerRunner::new(
                KeycloakInstanceController::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts.controllers.contains(&ControllerOpt::Realm) {
        controllers.push(
            ControllerRunner::new(KeycloakRealmController::default(), &client)
                .run()
                .boxed(),
        );
    }

    futures::future::try_join_all(controllers).await?;
    Ok(())
}
