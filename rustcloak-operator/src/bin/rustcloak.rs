use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, get, middleware,
};
use clap::Parser;
use futures::{FutureExt, future};
use kube::Resource;
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
    opts::{LegacyMode, Opts, legacy_kinds},
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
    use structured_logger::{Builder, async_json::new_writer};

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
        controllers_str.extend_from_slice(&legacy_kinds());
    }

    if controllers_str.contains(&KeycloakApiObject::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::new(
                KeycloakApiObjectController::<KeycloakApiObject>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&KeycloakInstance::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::new(
                KeycloakInstanceController::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&KeycloakRealm::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakRealm>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&KeycloakClient::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakClient>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&KeycloakUser::kind(&()).to_string()) {
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
        .contains(&KeycloakAuthenticationFlow::kind(&()).to_string())
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
        .contains(&KeycloakAuthenticatorConfig::kind(&()).to_string())
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
    if controllers_str.contains(&KeycloakClientScope::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakClientScope>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&KeycloakComponent::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakComponent>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&KeycloakGroup::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakGroup, EitherMarker>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str
        .contains(&KeycloakIdentityProvider::kind(&()).to_string())
    {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakIdentityProvider>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str
        .contains(&KeycloakIdentityProviderMapper::kind(&()).to_string())
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
    if controllers_str.contains(&KeycloakOrganization::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakOrganization>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&KeycloakProtocolMapper::kind(&()).to_string())
    {
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
        .contains(&KeycloakRequiredActionProvider::kind(&()).to_string())
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
    if controllers_str.contains(&KeycloakResource::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakResource>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&KeycloakRole::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::new(
                MorphController::<KeycloakRole, EitherMarker>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&KeycloakScope::kind(&()).to_string()) {
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
    if controllers_str.contains(&format!(
        "Legacy{}",
        keycloak_crd::ExternalKeycloak::kind(&())
    )) {
        controllers.push(
            ControllerRunner::new(
                LegacyInstanceController::new(prudent),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str
        .contains(&format!("Legacy{}", keycloak_crd::KeycloakRealm::kind(&())))
    {
        controllers.push(
            ControllerRunner::new(LegacyRealmController::new(prudent), &client)
                .run()
                .boxed(),
        );
    }
    if controllers_str.contains(&format!(
        "Legacy{}",
        keycloak_crd::KeycloakClient::kind(&())
    )) {
        controllers.push(
            ControllerRunner::new(
                LegacyClientController::new(prudent),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str
        .contains(&format!("Legacy{}", keycloak_crd::KeycloakUser::kind(&())))
    {
        controllers.push(
            ControllerRunner::new(LegacyUserController::new(prudent), &client)
                .run()
                .boxed(),
        );
    }
    if controllers_str.contains(&KeycloakClient::kind(&()).to_string()) {
        controllers
            .push(KeycloakClientSecretController::new(&client).run().boxed());
    }
    if controllers_str.contains(&KeycloakUser::kind(&()).to_string()) {
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
