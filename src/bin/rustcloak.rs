use actix_web::{
    get, middleware, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use anyhow::Result;
use clap::Parser;
use futures::{future, FutureExt};
use rustcloak_operator::controller::{
    ControllerRunner, KeycloakApiObjectController, KeycloakInstanceController,
    MorphController,
};
use rustcloak_operator::crd::{KeycloakClient, KeycloakRealm, KeycloakUser};
use rustcloak_operator::opts::{ControllerOpt, Opts};

#[get("/healthz")]
async fn health(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("healthy")
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    pretty_env_logger::init();

    let client = kube::Client::try_default().await?;
    let mut controllers = vec![];
    if opts.controllers.contains(&ControllerOpt::Api) {
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
            ControllerRunner::new(
                MorphController::<KeycloakRealm>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts.controllers.contains(&ControllerOpt::Client) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakClient>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts.controllers.contains(&ControllerOpt::User) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakUser>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }

    if let Some(sock_addr) = opts.metrics_addr {
        let server = HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default().exclude("/healthz"))
                .service(health)
        })
        .bind(sock_addr)?
        .shutdown_timeout(5)
        .run()
        .then(|_| future::ready(Ok(())));

        controllers.push(server.boxed());
    }

    futures::future::try_join_all(controllers).await?;
    Ok(())
}
