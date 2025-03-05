use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, get, middleware,
};
use clap::Parser;
use futures::{FutureExt, future};
use kube::Resource;
use rustcloak_operator::{
    controller::{
        ApiObjectController, ClientCredentialController, ControllerRunner,
        ConverterController, InstanceController, LegacyClientController,
        LegacyInstanceController, LegacyRealmController, LegacyUserController,
        RepresentationController, UserCredentialController,
    },
    crd::*,
    error::Result,
    opts::{LegacyMode, Opts, legacy_kinds},
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
    let mut controllers_str = opts.controllers.clone();
    if controllers_str.is_empty() {
        controllers_str =
            map_all_crds!(Crd => Crd::kind(&()).to_string()).collect();
    }

    if opts.legacy != LegacyMode::Disabled {
        controllers_str.extend_from_slice(&legacy_kinds());
    }

    // All Rest CRDs
    let mut controllers = map_rest_crds!( Crd => {
        controllers_str.contains(&Crd::kind(&()).to_string()).then(|| {
            ControllerRunner::new(
                RepresentationController::<Crd>::default(),
                &client,
            )
            .run()
            .boxed()
        })
    })
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    // Plumbing CRDs
    if controllers_str
        .contains(&ClusterKeycloakApiObject::kind(&()).to_string())
    {
        controllers.push(
            ControllerRunner::new(
                ApiObjectController::<ClusterKeycloakApiObject>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&KeycloakApiObject::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::new(
                ApiObjectController::<KeycloakApiObject>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&ClusterKeycloakInstance::kind(&()).to_string())
    {
        controllers.push(
            ControllerRunner::new(
                InstanceController::<ClusterKeycloakInstance>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&KeycloakInstance::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::new(
                InstanceController::<KeycloakInstance>::default(),
                &client,
            )
            .run()
            .boxed(),
        );
    }

    // Credential CRDs
    if controllers_str.contains(&KeycloakUserCredential::kind(&()).to_string())
    {
        controllers.extend([
            ControllerRunner::new(UserCredentialController::default(), &client)
                .run()
                .boxed(),
            ConverterController::<KeycloakUser, KeycloakUserCredential>::new(
                &client,
            )
            .run()
            .boxed(),
        ]);
    }
    if controllers_str
        .contains(&KeycloakClientCredential::kind(&()).to_string())
    {
        controllers.extend([
            ControllerRunner::new(ClientCredentialController::default(), &client) .run() .boxed(),
            ConverterController::<KeycloakClient, KeycloakClientCredential>::new(&client).run().boxed()
        ])
    }

    // Legacy CRDs
    let prudent = opts.legacy == LegacyMode::Prudent;

    if controllers_str.contains(&"LegacyInstance".to_string()) {
        controllers.push(
            ControllerRunner::new(
                LegacyInstanceController::new(prudent),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&"LegacyRealm".to_string()) {
        controllers.push(
            ControllerRunner::new(LegacyRealmController::new(prudent), &client)
                .run()
                .boxed(),
        );
    }
    if controllers_str.contains(&"LegacyClient".to_string()) {
        controllers.push(
            ControllerRunner::new(
                LegacyClientController::new(prudent),
                &client,
            )
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&"LegacyUser".to_string()) {
        controllers.push(
            ControllerRunner::new(LegacyUserController::new(prudent), &client)
                .run()
                .boxed(),
        );
    }

    // Metrics
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
