use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::{
    app_id,
    error::Result,
    util::{ApiExt, ApiFactory, FromError, StatusTrait, wait_for_crd},
};
use async_trait::async_trait;
use k8s_openapi::serde_json::json;
use kube::{
    Api, Resource as KubeResource, ResourceExt,
    api::{Patch, PatchParams},
    core::object::HasStatus,
    runtime::{
        Controller,
        controller::{self, Action},
        watcher,
    },
};
use log::{debug, info, warn};
use prometheus::{IntCounter, Opts, register_int_counter};
use serde::{Serialize, de::DeserializeOwned};

use crate::error::*;
use futures::StreamExt;
use kube::runtime::finalizer::{Error as FinalizerError, Event, finalizer};
use std::fmt::Debug;
use std::hash::Hash;

#[async_trait]
pub trait LifecycleController {
    type Resource: Clone + KubeResource + Debug + 'static + Send + Sync;
    const MODULE_PATH: &'static str;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource>
    where
        <Self::Resource as kube::Resource>::DynamicType: Eq + Hash + From<()>;

    async fn before_finalizer(
        &self,
        _client: &kube::Client,
        _resource: Arc<Self::Resource>,
    ) -> Result<bool> {
        Ok(true)
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action>;
    async fn cleanup(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action>;
}

pub struct ControllerRunner<C> {
    controller: C,
    client: kube::Client,
    prometheus_reconsiles: IntCounter,
    prometheus_reconsiles_success: IntCounter,
    prometheus_reconsiles_fail: IntCounter,
}

impl<C> ControllerRunner<C>
where
    C: LifecycleController + Sync + Send + 'static,
    C::Resource: KubeResource<DynamicType = ()>
        + Clone
        + HasStatus<Status: FromError + Serialize + Send + Sync + StatusTrait>
        + Debug
        + 'static
        + Send
        + Sync
        + Serialize
        + DeserializeOwned
        + Debug,
    <C::Resource as KubeResource>::DynamicType:
        Default + Eq + Hash + Clone + Debug + Unpin + From<()>,
    ApiExt<C::Resource>: ApiFactory<Resource = C::Resource>,
{
    fn setup_metrics() -> (IntCounter, IntCounter, IntCounter) {
        let pod_namespace = std::env::var("POD_NAMESPACE")
            .unwrap_or_else(|_| "<unknown>".to_string());
        let pod_name = std::env::var("POD_NAME")
            .unwrap_or_else(|_| "<unknown>".to_string());
        let common_labels: HashMap<_, _> = [
            ("kind".to_string(), C::Resource::kind(&()).to_string()),
            ("group".to_string(), C::Resource::group(&()).to_string()),
            ("controller_pod_name".to_string(), pod_name),
            ("controller_pod_namespace".to_string(), pod_namespace),
        ]
        .into();

        let prometheus_reconsiles = register_int_counter!(Opts {
            namespace: "rustcloak".to_string(),
            subsystem: "controller".to_string(),
            name: "reconciles".to_string(),
            help: "Number of started reconciles".to_string(),
            const_labels: common_labels.clone(),
            variable_labels: vec![],
        })
        .unwrap();
        let prometheus_reconsiles_success = register_int_counter!(Opts {
            namespace: "rustcloak".to_string(),
            subsystem: "controller".to_string(),
            name: "reconciles_success".to_string(),
            help: "Number of successful reconciles".to_string(),
            const_labels: common_labels.clone(),
            variable_labels: vec![],
        })
        .unwrap();
        let prometheus_reconsiles_fail = register_int_counter!(Opts {
            namespace: "rustcloak".to_string(),
            subsystem: "controller".to_string(),
            name: "reconciles_fail".to_string(),
            help: "Number of failed reconciles".to_string(),
            const_labels: common_labels.clone(),
            variable_labels: vec![],
        })
        .unwrap();

        (
            prometheus_reconsiles,
            prometheus_reconsiles_success,
            prometheus_reconsiles_fail,
        )
    }
    pub fn new(controller: C, client: &kube::Client) -> Self {
        let client = client.clone();

        let (
            prometheus_reconsiles,
            prometheus_reconsiles_success,
            prometheus_reconsiles_fail,
        ) = Self::setup_metrics();

        ControllerRunner {
            controller,
            client,
            prometheus_reconsiles,
            prometheus_reconsiles_success,
            prometheus_reconsiles_fail,
        }
    }

    pub async fn run(self) -> Result<()> {
        let api = Api::<C::Resource>::all(self.client.clone());
        let config = controller::Config::default().concurrency(2);
        let kind = C::Resource::kind(&());

        wait_for_crd::<C::Resource, C>(&self.client).await?;

        info!(target: C::MODULE_PATH, kind = kind; "starting controller for {kind}");

        let controller = Controller::new(api, watcher::Config::default())
            .with_config(config)
            .shutdown_on_signal();
        let controller = self.controller.prepare(controller, &self.client);
        let self_arc = Arc::new(self);

        controller
            .run(Self::reconcile, Self::error_policy, self_arc.clone())
            .for_each(|res| async {
                if res.is_err() {
                    self_arc.prometheus_reconsiles_fail.inc();
                } else {
                    self_arc.prometheus_reconsiles_success.inc();
                }
                match res {
                    Ok((o, _)) => {
                        info!(target: C::MODULE_PATH,
                            kind = kind,
                            namespace = o.namespace,
                            name = o.name;
                            "reconciled"
                        )
                    }
                    Err(controller::Error::ReconcilerFailed(e, o)) => {
                        warn!(target: C::MODULE_PATH,
                            namespace = o.namespace,
                            name = o.name,
                            kind = kind;
                            "reconciling failed: {e}",
                        )
                    }
                    Err(controller::Error::QueueError(e)) => {
                        warn!(target: C::MODULE_PATH, kind = kind; "Queue Error: {}", e)
                    },
                    Err(controller::Error::RunnerError(e)) => {
                        warn!(target: C::MODULE_PATH, kind = kind; "Runner Error: {}", e)
                    },
                    Err(e) => {
                        warn!(target: C::MODULE_PATH, kind = kind; "{}", e)
                    }
                }
            })
            .await;
        Ok(())
    }

    async fn reconcile(
        resource: Arc<C::Resource>,
        ctx: Arc<Self>,
    ) -> Result<Action> {
        let ns = resource.namespace();
        let name = resource.name_unchecked();
        let api = ApiExt::<C::Resource>::api(ctx.client.clone(), &ns);
        let client = ctx.client.clone();
        let kind = C::Resource::kind(&());
        ctx.prometheus_reconsiles.inc();

        debug!(
            kind = kind,
            namespace = ns,
            name = name;
            "start reconciling"
        );

        match ctx
            .controller
            .before_finalizer(&client, resource.clone())
            .await
        {
            Ok(true) => {
                debug!(
                    kind = kind,
                    namespace = ns,
                    name = name;
                    "handling resource, before_finalizer returned true"
                )
            }
            Ok(false) => {
                debug!(
                    kind = kind,
                    namespace = ns,
                    name = name;
                    "skipping resource, before_finalizer returned false"
                );
                return Ok(Action::await_change());
            }
            Err(e) => {
                Self::handle_error(ctx.clone(), &resource, &e).await?;
                return Err(e);
            }
        }

        match finalizer(
            &api,
            app_id!("finalizer"),
            resource.clone(),
            |event| async {
                match event {
                    Event::Apply(resource) => {
                        ctx.controller.apply(&client, resource).await
                    }
                    Event::Cleanup(resource) => {
                        ctx.controller.cleanup(&client, resource).await
                    }
                }
            },
        )
        .await
        {
            Ok(action) => Ok(action),
            Err(FinalizerError::ApplyFailed(e))
            | Err(FinalizerError::CleanupFailed(e)) => {
                Self::handle_error(ctx.clone(), &resource, &e).await?;
                Err(e)
            }
            Err(e) => Err(Error::Other(Box::new(e))),
        }
    }

    async fn handle_error(
        ctx: Arc<Self>,
        resource: &C::Resource,
        e: &Error,
    ) -> Result<()> {
        let ns = resource.namespace();
        let name = resource.name_unchecked();
        let api: Api<C::Resource> = ApiExt::api(ctx.client.clone(), &ns);
        let attempts = resource
            .status()
            .and_then(|s| s.reconcile_attempts())
            .unwrap_or(0);
        let mut status = <C::Resource as HasStatus>::Status::from_error(e);
        status.set_reconcile_attempts(Some(attempts + 1));
        log::error!(
            kind = C::Resource::kind(&()),
            namespace = resource.namespace().unwrap_or_default(),
            name = name;
            "error: {e}"
        );
        let patch = Patch::Merge(json!({
            "status": status,
        }));

        api.patch_status(&name, &PatchParams::apply(app_id!()), &patch)
            .await?;

        Ok(())
    }
    fn error_policy(
        resource: Arc<C::Resource>,
        _error: &Error,
        _ctx: Arc<Self>,
    ) -> Action {
        let attempts = resource
            .status()
            .and_then(|s| s.reconcile_attempts())
            .unwrap_or(0);
        let backoff_seconds = if attempts < 10 {
            // for the first 10 attempts, we retry every 10 seconds
            10
        } else {
            // after that, we increase the backoff by 60 seconds for each attempt
            (attempts - 9) * 60
        };
        Action::requeue(Duration::from_secs(backoff_seconds))
    }
}
