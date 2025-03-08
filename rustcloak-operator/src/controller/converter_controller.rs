use std::fmt::Debug;
use std::{sync::Arc, time::Duration};

use crate::util::{ApiExt, ApiFactory};
use crate::{app_id, error::*};
use crate::{error::Result, util::wait_for_crd};
use futures::StreamExt;
use kube::ResourceExt;
use kube::api::{DeleteParams, ObjectMeta, Patch, PatchParams};
use kube::core::object::HasStatus;
use kube::{
    Api, Resource as KubeResource,
    runtime::{
        Controller,
        controller::{self, Action},
        watcher,
    },
};
use log::{info, warn};
use rustcloak_crd::traits::Endpoint;
use rustcloak_crd::{
    KeycloakApiStatus, KeycloakClient, KeycloakClientCredential,
    KeycloakClientCredentialSpec, KeycloakUser, KeycloakUserCredential,
    KeycloakUserCredentialSpec,
};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait ConvertTo<T> {
    fn convert(&self) -> Option<T>;
}

impl ConvertTo<KeycloakUserCredential> for KeycloakUser {
    fn convert(&self) -> Option<KeycloakUserCredential> {
        let resource_path = self.resource_path()?.to_string();
        let instance_ref = self.instance_ref()?.clone();
        let user_secret = self.spec.user_secret.as_ref()?.clone();
        if self.spec.parent_ref.is_right() {
            return None;
        }

        let owner_ref = self.owner_ref(&()).unwrap();
        Some(KeycloakUserCredential {
            metadata: ObjectMeta {
                name: self.metadata.name.clone(),
                namespace: self.metadata.namespace.clone(),
                owner_references: Some(vec![owner_ref]),
                ..Default::default()
            },
            spec: KeycloakUserCredentialSpec {
                instance_ref,
                resource_path,
                user_secret,
            },
            status: None,
        })
    }
}

impl ConvertTo<KeycloakClientCredential> for KeycloakClient {
    fn convert(&self) -> Option<KeycloakClientCredential> {
        let resource_path = self.resource_path()?.to_string();
        let instance_ref = self.instance_ref()?.clone();
        let client_secret = self.spec.client_secret.as_ref()?.clone();

        let owner_ref = self.owner_ref(&()).unwrap();
        Some(KeycloakClientCredential {
            metadata: ObjectMeta {
                name: self.metadata.name.clone(),
                namespace: self.metadata.namespace.clone(),
                owner_references: Some(vec![owner_ref]),
                ..Default::default()
            },
            spec: KeycloakClientCredentialSpec {
                instance_ref,
                resource_path,
                client_secret,
            },
            status: None,
        })
    }
}

pub struct ConverterController<F, T> {
    phantom: std::marker::PhantomData<(F, T)>,
    client: kube::Client,
}

impl<F, T> ConverterController<F, T>
where
    F: KubeResource<DynamicType = ()>
        + Clone
        + HasStatus<Status = KeycloakApiStatus>
        + Debug
        + 'static
        + Send
        + Sync
        + Serialize
        + DeserializeOwned
        + Debug
        + ConvertTo<T>,
    T: KubeResource<DynamicType = ()>
        + Clone
        + HasStatus<Status = KeycloakApiStatus>
        + Debug
        + 'static
        + Send
        + Sync
        + Serialize
        + DeserializeOwned
        + Debug,
    ApiExt<T>: ApiFactory<Resource = T>,
{
    pub fn new(client: &kube::Client) -> Self {
        let client = client.clone();
        ConverterController {
            client,
            phantom: std::marker::PhantomData,
        }
    }

    pub async fn run(self) -> Result<()> {
        let from_api = Api::<F>::all(self.client.clone());
        let to_api = Api::<T>::all(self.client.clone());
        let config = controller::Config::default().concurrency(2);
        let kind = F::kind(&());
        wait_for_crd::<KeycloakUser, Self>(&self.client).await?;

        info!(kind = kind; "starting converter controller");
        Controller::new(from_api, watcher::Config::default())
            .with_config(config)
            .shutdown_on_signal()
            .owns(to_api.clone(), watcher::Config::default())
            .run(Self::reconcile, Self::error_policy, Arc::new(self))
            .for_each(|res| async {
                match res {
                    Ok((o, _)) => {
                        info!(
                            namespace = o.namespace,
                            name = o.name,
                            kind = kind;
                            "reconciled",
                        )
                    }
                    Err(controller::Error::ReconcilerFailed(e, o)) => {
                        warn!(
                            namespace = o.namespace,
                            name = o.name,
                            kind = kind;
                            "{e}",
                        )
                    }
                    Err(e) => warn!( kind = kind; "{e}"),
                }
            })
            .await;
        Ok(())
    }

    pub async fn reconcile(resource: Arc<F>, ctx: Arc<Self>) -> Result<Action> {
        match Self::reconcile_inner(resource.clone(), ctx.clone()).await {
            Ok(action) => Ok(action),
            Err(e) => {
                Self::handle_error(ctx, &resource, &e).await?;
                Err(e)
            }
        }
    }

    pub async fn reconcile_inner(
        resource: Arc<F>,
        ctx: Arc<Self>,
    ) -> Result<Action> {
        let ns = resource.namespace();
        let to_api = ApiExt::<T>::api(ctx.client.clone(), &ns);
        let name = resource.name_unchecked();
        let kind = F::kind(&());

        if let Some(converted) = resource.convert() {
            to_api
                .patch(
                    &name,
                    &PatchParams::apply(app_id!()),
                    &Patch::Apply(converted),
                )
                .await?;
        } else {
            match to_api.delete(&name, &DeleteParams::default()).await {
                Err(kube::Error::Api(kube::core::ErrorResponse {
                    code: 404,
                    message: m,
                    ..
                })) => {
                    warn!(
                    kind = kind,
                    name = name,
                    namespace = ns;
                    "Resource not found, assuming it's already deleted. Message: {}", m);
                }
                x => {
                    x?;
                }
            }
        }

        Ok(Action::await_change())
    }

    pub async fn handle_error(
        _ctx: Arc<Self>,
        _resource: &Arc<F>,
        _e: &Error,
    ) -> Result<()> {
        Ok(())
    }

    fn error_policy(
        _resource: Arc<F>,
        _error: &Error,
        _ctx: Arc<Self>,
    ) -> Action {
        Action::requeue(Duration::from_secs(5))
    }
}
