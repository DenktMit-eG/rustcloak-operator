use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get};
use clap::Parser;
use futures::{FutureExt, future};
use kube::Resource;
use log::info;
use rustcloak_crd::{
    KeycloakClientCredential, KeycloakRoleMapping, KeycloakUserCredential,
    api_object::{ClusterKeycloakApiObject, KeycloakApiObject},
    client::KeycloakClient,
    instance::{ClusterKeycloakInstance, KeycloakInstance},
    map_all_crds, map_rest_crds,
    user::KeycloakUser,
};
use rustcloak_operator::{
    controller::{
        ApiObjectController, ClientCredentialController, ControllerRunner,
        ConverterController, InstanceController, LegacyClientController,
        LegacyInstanceController, LegacyRealmController, LegacyUserController,
        RepresentationController, RoleMappingController,
        UserCredentialController,
    },
    error::Result,
    metrics::metrics,
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

fn main() -> Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .max_blocking_threads(8)
        .build()
        .unwrap()
        .block_on(async { async_main().await })
}

async fn async_main() -> Result<()> {
    let opts = Opts::parse();

    init_logger();

    info!(
        "Starting rustcloak-operator version {}",
        env!("CARGO_PKG_VERSION")
    );

    let client = kube::Client::try_default().await?;
    let mut controllers_str = opts.controllers.clone();
    if controllers_str.is_empty() {
        controllers_str.extend_from_slice(
            &map_all_crds!(Crd => Crd::kind(&()).to_string()),
        );
    }

    if opts.legacy != LegacyMode::Disabled {
        controllers_str.extend_from_slice(&legacy_kinds());
    }

    // All Rest CRDs
    let mut controllers = map_rest_crds!(Crd => {
        if controllers_str.contains(&Crd::kind(&()).to_string()) {
            Ok(Some(
                ControllerRunner::create(
                    RepresentationController::<Crd>::default(),
                    &client,
                )?
                .run()
                .boxed(),
            ))
        } else {
            Ok(None)
        }
    })
    .into_iter()
    .filter_map(|x| x.transpose())
    .collect::<Result<Vec<_>>>()?;

    // Plumbing CRDs
    if controllers_str
        .contains(&ClusterKeycloakApiObject::kind(&()).to_string())
    {
        controllers.push(
            ControllerRunner::create(
                ApiObjectController::<ClusterKeycloakApiObject>::default(),
                &client,
            )?
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&KeycloakApiObject::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::create(
                ApiObjectController::<KeycloakApiObject>::default(),
                &client,
            )?
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&ClusterKeycloakInstance::kind(&()).to_string())
    {
        controllers.push(
            ControllerRunner::create(
                InstanceController::<ClusterKeycloakInstance>::default(),
                &client,
            )?
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&KeycloakInstance::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::create(
                InstanceController::<KeycloakInstance>::default(),
                &client,
            )?
            .run()
            .boxed(),
        );
    }

    // Credential CRDs
    if controllers_str.contains(&KeycloakUserCredential::kind(&()).to_string())
    {
        controllers.extend([
            ControllerRunner::create(
                UserCredentialController::default(),
                &client,
            )?
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
            ControllerRunner::create(
                ClientCredentialController::default(),
                &client,
            )?
            .run()
            .boxed(),
            ConverterController::<
                KeycloakClient,
                KeycloakClientCredential,
            >::new(&client)
            .run()
            .boxed(),
        ])
    }

    // RoleMapping CRDs
    if controllers_str.contains(&KeycloakRoleMapping::kind(&()).to_string()) {
        controllers.push(
            ControllerRunner::create(
                RoleMappingController::default(),
                &client,
            )?
            .run()
            .boxed(),
        );
    }

    // Legacy CRDs
    let prudent = opts.legacy == LegacyMode::Prudent;

    if controllers_str.contains(&"LegacyInstance".to_string()) {
        controllers.push(
            ControllerRunner::create(
                LegacyInstanceController::new(prudent),
                &client,
            )?
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&"LegacyRealm".to_string()) {
        controllers.push(
            ControllerRunner::create(
                LegacyRealmController::new(prudent),
                &client,
            )?
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&"LegacyClient".to_string()) {
        controllers.push(
            ControllerRunner::create(
                LegacyClientController::new(prudent),
                &client,
            )?
            .run()
            .boxed(),
        );
    }
    if controllers_str.contains(&"LegacyUser".to_string()) {
        controllers.push(
            ControllerRunner::create(
                LegacyUserController::new(prudent),
                &client,
            )?
            .run()
            .boxed(),
        );
    }

    // Metrics
    if let Some(sock_addr) = opts.metrics_addr {
        controllers.push(
            HttpServer::new(move || {
                App::new().service(health).service(metrics)
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
