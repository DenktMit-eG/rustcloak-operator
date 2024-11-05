use actix_web::{
    get, middleware, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use anyhow::Result;
use clap::Parser;
use futures::{future, FutureExt};
use rustcloak_operator::controller::{
    ControllerRunner, KeycloakApiObjectController,
    KeycloakClientSecretController, KeycloakInstanceController,
    KeycloakUserSecretController, MorphController,
};
use rustcloak_operator::crd::*;
use rustcloak_operator::opts::{ControllerOpt, Opts};

#[get("/healthz")]
async fn health(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("healthy")
}

#[cfg(debug_assertions)]
fn init_logger() {
    pretty_env_logger::init();
}

#[cfg(not(debug_assertions))]
fn init_logger() {
    use structured_logger::{async_json::new_writer, Builder};

    Builder::with_level("info")
        .with_target_writer("*", new_writer(tokio::io::stdout()))
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    eprintln!(
        "Starting rustcloak-operator version {}",
        env!("CARGO_PKG_VERSION")
    );

    init_logger();

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
    if opts
        .controllers
        .contains(&ControllerOpt::AuthenticationFlow)
    {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakAuthenticationFlow>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts
        .controllers
        .contains(&ControllerOpt::AuthenticatorConfig)
    {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakAuthenticatorConfig>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts.controllers.contains(&ControllerOpt::ClientScope) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakClientScope>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts.controllers.contains(&ControllerOpt::Component) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakComponent>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts.controllers.contains(&ControllerOpt::Group) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakGroup>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts.controllers.contains(&ControllerOpt::IdentityProvider) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakIdentityProvider>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts
        .controllers
        .contains(&ControllerOpt::IdentityProviderMapper)
    {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakIdentityProviderMapper>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts.controllers.contains(&ControllerOpt::Organization) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakOrganization>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts.controllers.contains(&ControllerOpt::ProtocolMapper) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakProtocolMapper>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts
        .controllers
        .contains(&ControllerOpt::RequiredActionProvider)
    {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakRequiredActionProvider>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts.controllers.contains(&ControllerOpt::Resource) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakResource>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts.controllers.contains(&ControllerOpt::Role) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakRole>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts.controllers.contains(&ControllerOpt::Scope) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakScope>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if opts.controllers.contains(&ControllerOpt::ClientSecret) {
        controllers
            .push(KeycloakClientSecretController::new(&client).run().boxed());
    }
    if opts.controllers.contains(&ControllerOpt::UserSecret) {
        controllers
            .push(KeycloakUserSecretController::new(&client).run().boxed());
    }

    if let Some(sock_addr) = opts.metrics_addr {
        controllers.push(
            HttpServer::new(move || {
                App::new()
                    .wrap(middleware::Logger::default().exclude("/healthz"))
                    .service(health)
            })
            .bind(sock_addr)?
            .shutdown_timeout(5)
            .run()
            .then(|_| future::ready(Ok(())))
            .boxed(),
        );
    }

    futures::future::try_join_all(controllers).await?;
    Ok(())
}
