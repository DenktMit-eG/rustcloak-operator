use crate::{
    api::KeycloakApiClient,
    app_id,
    controller::controller_runner::LifecycleController,
    error::{Error, Result},
    util::{ApiResolver, K8sKeycloakBuilder, RefWatcher, ToPatch},
};
use async_trait::async_trait;
use futures::{stream, StreamExt};
use k8s_openapi::{
    api::core::v1::{ConfigMap, Secret},
    DeepMerge, NamespaceResourceScope,
};
use kube::core::object::HasStatus;
use kube::runtime::watcher;
use kube::Resource;
use kube::{
    api::PatchParams,
    runtime::{controller::Action, Controller},
    Api, ResourceExt,
};
use log::warn;
use reqwest::StatusCode;
use rustcloak_crd::{
    inner_spec::HasInnerSpec, KeycloakApiEndpointPath, KeycloakApiObjectSpec,
    KeycloakApiStatus, KeycloakApiStatusEndpoint, KeycloakInstance,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::Notify;

/// This controller is responsible for applying the desired keycloak state in kubernetes to a
/// keycloak instance.
#[derive(Debug)]
pub struct KeycloakApiObjectController<R>
where
    R: Resource<DynamicType = ()>,
{
    reconcile_notify: Arc<Notify>,
    secret_refs: Arc<RefWatcher<R, Secret>>,
    config_map_refs: Arc<RefWatcher<R, ConfigMap>>,
}

impl<R> Default for KeycloakApiObjectController<R>
where
    R: Resource<DynamicType = ()>,
{
    fn default() -> Self {
        Self {
            reconcile_notify: Default::default(),
            secret_refs: Default::default(),
            config_map_refs: Default::default(),
        }
    }
}

impl<R> KeycloakApiObjectController<R>
where
    R: Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + HasInnerSpec<InnerSpec = KeycloakApiObjectSpec>
        + Clone
        + Debug
        + HasStatus<Status = KeycloakApiStatus>
        + DeserializeOwned,
{
    async fn resolve_path(
        &self,
        client: &kube::Client,
        ns: &str,
        resource: &R,
    ) -> Result<String> {
        let (parent_ref, sub_path) =
            match &resource.inner_spec().endpoint.path_def {
                KeycloakApiEndpointPath::Path(path) => {
                    return Ok(path.to_string())
                }
                KeycloakApiEndpointPath::Parent {
                    parent_ref,
                    sub_path,
                } => (parent_ref, sub_path),
            };
        let api = Api::<R>::namespaced(client.clone(), ns);
        let parent = api
            .get_opt(parent_ref)
            .await?
            .ok_or(Error::NoParent(ns.to_string(), parent_ref.to_string()))?;
        let parent_endpoint = parent
            .status()
            .as_ref()
            .and_then(|x| x.endpoint.as_ref())
            .ok_or(Error::NoResourcePath)?;
        Ok(format!(
            "{}/{}",
            parent_endpoint.resource_path.trim_end_matches('/'),
            sub_path.trim_start_matches('/')
        ))
    }

    async fn keycloak(
        client: &kube::Client,
        resource: &R,
    ) -> Result<KeycloakApiClient> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let instance_api =
            Api::<KeycloakInstance>::namespaced(client.clone(), &ns);

        let instance_ref = &resource.inner_spec().endpoint.instance_ref;
        let instance = instance_api
            .get_opt(instance_ref)
            .await?
            .ok_or(Error::NoInstance(ns, instance_ref.to_string()))?;

        K8sKeycloakBuilder::new(&instance, client)
            .with_token()
            .await
    }
}

#[async_trait]
impl<R> LifecycleController for KeycloakApiObjectController<R>
where
    R: Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + HasInnerSpec<InnerSpec = KeycloakApiObjectSpec>
        + HasStatus<Status = KeycloakApiStatus>
        + Send
        + Sync
        + Clone
        + Debug
        + DeserializeOwned
        + 'static,
{
    type Resource = R;
    const MODULE_PATH: &'static str = module_path!();

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource> {
        let secret_refs = self.secret_refs.clone();
        let config_map_refs = self.config_map_refs.clone();
        let secret_api = Api::<Secret>::all(client.clone());
        let config_map_api = Api::<ConfigMap>::all(client.clone());
        let notify = self.reconcile_notify.clone();
        controller
            .reconcile_all_on(stream::repeat(()).then(move |()| {
                let notify = notify.clone();
                async move {
                    notify.notified().await;
                }
            }))
            .watches(secret_api, watcher::Config::default(), move |secret| {
                secret_refs.watch(&secret)
            })
            .watches(
                config_map_api,
                watcher::Config::default(),
                move |config_map| config_map_refs.watch(&config_map),
            )
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let name = resource.name_unchecked();
        let api = Api::<R>::namespaced(client.clone(), &ns);
        let keycloak = Self::keycloak(client, &resource).await?;
        let mut payload = resource.resolve(client).await?;
        let immutable_payload: Value =
            serde_yaml::from_str(&resource.inner_spec().immutable_payload.0)?;
        payload.merge_from(immutable_payload.clone());
        let mut success = false;
        let kind = R::kind(&());

        if let Some(endpoint) =
            resource.status().as_ref().and_then(|s| s.endpoint.as_ref())
        {
            match keycloak.put(&endpoint.resource_path, &payload).await {
                Ok(_) => {
                    success = true;
                    api.patch_status(
                        &name,
                        &PatchParams::apply(app_id!()),
                        &KeycloakApiStatus::ok("Applied").to_patch(),
                    )
                    .await?;
                }
                Err(Error::KeycloakError(StatusCode::NOT_FOUND, m)) => {
                    warn!(
                        kind = kind,
                        name = name,
                        namespace = ns,
                        path = endpoint.resource_path;
                        "Failed to update resource at path, try recreating. (Message: {})",
                        m
                    );
                }
                x => {
                    warn!("{:#?}", x);
                    x?
                }
            }
        }
        println!("success: {}", success);

        if !success {
            let path = self.resolve_path(client, &ns, &resource).await?;
            warn!("path: {}", path);
            let resource_path = keycloak.post_location(&path, &payload).await?;
            let mut status = resource.status().cloned().unwrap_or_default();
            status.endpoint = Some(KeycloakApiStatusEndpoint {
                resource_path,
                instance_ref: resource
                    .inner_spec()
                    .endpoint
                    .instance_ref
                    .0
                    .clone(),
            });

            api.patch_status(
                &name,
                &PatchParams::apply(app_id!()),
                &status.to_patch(),
            )
            .await?;
        }

        let ref_iter = resource
            .inner_spec()
            .vars
            .iter()
            .filter_map(|v| v.value_from.as_ref());
        let secret_refs = ref_iter
            .clone()
            .filter_map(|v| v.secret_key_ref.as_ref().map(|r| &r.name));
        self.secret_refs.add(&resource, secret_refs);
        let config_map_refs = ref_iter
            .filter_map(|v| v.config_map_key_ref.as_ref().map(|r| &r.name));
        self.config_map_refs.add(&resource, config_map_refs);

        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let kind = R::kind(&());
        let name = resource.name_unchecked();
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let Some(endpoint) =
            resource.status().as_ref().and_then(|s| s.endpoint.as_ref())
        else {
            // If the resource has no resource URL we expect that it never got created, so it's
            // safe to delete the resource.
            return Ok(Action::await_change());
        };
        let keycloak = match Self::keycloak(client, &resource).await {
            Ok(k) => k,
            Err(Error::NoInstance(_, _)) => {
                warn!(
                    kind = kind,
                    name = name,
                    namespace = ns,
                    path = endpoint.resource_path;
                    "Keycloak instance not found, assuming you want to unmanage the whole keycloak instance."
                );
                return Ok(Action::await_change());
            }
            Err(e) => Err(e)?,
        };
        match keycloak.delete(&endpoint.resource_path).await {
            Err(Error::KeycloakError(StatusCode::NOT_FOUND, m)) => {
                warn!(
                    kind = kind,
                    name = name,
                    namespace = ns,
                    path = endpoint.resource_path;
                    "Resource not found, assuming it's already deleted. Message: {}", m);
            }
            x => x?,
        }

        self.secret_refs.remove(&resource);
        self.config_map_refs.remove(&resource);

        // If we delete a resource, we let the controller reconcile all objects, so that
        // the resources that depend on the deleted resource are starting to fail.
        self.reconcile_notify.notify_one();

        Ok(Action::await_change())
    }
}
