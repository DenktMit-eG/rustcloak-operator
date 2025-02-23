use actix_web::{
    get, middleware, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use clap::Parser;
use futures::{future, FutureExt};
use rustcloak_operator::{
    controller::{
        ControllerRunner, KeycloakApiObjectController,
        KeycloakClientSecretController, KeycloakInstanceController,
        KeycloakUserSecretController, LegacyClientController,
        LegacyInstanceController, LegacyRealmController, LegacyUserController,
        MorphController,
    },
    crd::*,
    error::Result,
    opts::{ControllerOpt, LegacyMode, Opts},
    shim::resource::EitherMarker,
};

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
    let mut controllers_str = opts.controllers.clone();

    if opts.legacy != LegacyMode::Disabled {
        controllers_str.extend_from_slice(&[
            ControllerOpt::LegacyInstance,
            ControllerOpt::LegacyRealm,
            ControllerOpt::LegacyClient,
            ControllerOpt::LegacyUser,
        ]);
    }

    if controllers_str.contains(&ControllerOpt::Api) {
        controllers.push(
            ControllerRunner::new(
                KeycloakApiObjectController::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&ControllerOpt::Instance) {
        controllers.push(
            ControllerRunner::new(
                KeycloakInstanceController::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&ControllerOpt::Realm) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakRealm>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&ControllerOpt::Client) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakClient>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&ControllerOpt::User) {
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
    if controllers_str.contains(&ControllerOpt::ClientScope) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakClientScope>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&ControllerOpt::Component) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakComponent>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&ControllerOpt::Group) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakGroup, EitherMarker>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&ControllerOpt::IdentityProvider) {
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
    if controllers_str.contains(&ControllerOpt::Organization) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakOrganization>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&ControllerOpt::ProtocolMapper) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakProtocolMapper, EitherMarker>::default(),
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
    if controllers_str.contains(&ControllerOpt::Resource) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakResource>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&ControllerOpt::Role) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakRole, EitherMarker>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&ControllerOpt::Scope) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakScope>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    let prudent = opts.legacy == LegacyMode::Prudent;
    if controllers_str.contains(&ControllerOpt::LegacyInstance) {
        controllers.push(
            ControllerRunner::new(
                LegacyInstanceController::new(prudent),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&ControllerOpt::LegacyRealm) {
        controllers.push(
            ControllerRunner::new(LegacyRealmController::new(prudent), &client)
                .run()
                .boxed(),
        );
    }
    if controllers_str.contains(&ControllerOpt::LegacyClient) {
        controllers.push(
            ControllerRunner::new(
                LegacyClientController::new(prudent),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&ControllerOpt::LegacyUser) {
        controllers.push(
            ControllerRunner::new(LegacyUserController::new(prudent), &client)
                .run()
                .boxed(),
        );
    }
    if controllers_str.contains(&ControllerOpt::ClientSecret) {
        controllers
            .push(KeycloakClientSecretController::new(&client).run().boxed());
    }
    if controllers_str.contains(&ControllerOpt::UserSecret) {
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
