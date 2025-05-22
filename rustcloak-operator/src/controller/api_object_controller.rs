use crate::{
    app_id,
    controller::controller_runner::LifecycleController,
    error::{Error, Result},
    util::{
        ApiExt, ApiFactory, JsonPatcher, K8sKeycloakBuilder, RefWatcher,
        Retrieve, Retriever, ToPatch,
    },
};
use async_trait::async_trait;
use either::for_both;
use futures::{StreamExt, stream};
use k8s_openapi::serde_json::Value;
use k8s_openapi::{
    DeepMerge,
    api::core::v1::{ConfigMap, Secret},
};
use keycloak_client::{ApiClient, StatusCode};
use kube::Resource;
use kube::core::object::HasStatus;
use kube::runtime::watcher;
use kube::{
    Api, ResourceExt,
    api::PatchParams,
    runtime::{Controller, controller::Action},
};
use log::warn;
use rustcloak_crd::{
    KeycloakApiStatus, KeycloakApiStatusEndpoint, KeycloakRestObject,
    ValueFrom,
    api_object::{
        ApiObjectRef, KeycloakApiEndpointParent, KeycloakApiEndpointPath,
        KeycloakApiObjectSpec,
    },
    inner_spec::HasInnerSpec,
    instance::InstanceRef,
    keycloak_types::UserRepresentation,
    user::KeycloakUserSpec,
};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::Notify;

/// This controller is responsible for applying the desired keycloak state in kubernetes to a
/// keycloak instance.
#[derive(Debug)]
pub struct ApiObjectController<R>
where
    R: Resource<DynamicType = ()>,
{
    reconcile_notify: Arc<Notify>,
    secret_refs: Arc<RefWatcher<R, Secret>>,
    config_map_refs: Arc<RefWatcher<R, ConfigMap>>,
}

impl<R> Default for ApiObjectController<R>
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

impl<R> ApiObjectController<R>
where
    R: Resource<DynamicType = ()>
        + HasInnerSpec<InnerSpec = KeycloakApiObjectSpec>
        + Clone
        + Debug
        + HasStatus<Status = KeycloakApiStatus>
        + DeserializeOwned
        + 'static,
{
    async fn resolve_path(
        &self,
        client: &kube::Client,
        ns: &Option<String>,
        resource: &R,
    ) -> Result<String> {
        let KeycloakApiEndpointParent {
            parent_ref,
            sub_path,
        } = match &resource.inner_spec().endpoint.path_def {
            KeycloakApiEndpointPath::Path(path) => {
                return Ok(path.to_string());
            }
            KeycloakApiEndpointPath::Parent(x) => x,
        };
        let parent =
            Retriever::<ApiObjectRef>::get(client.clone(), parent_ref, ns)
                .await?;
        let status = for_both!(parent, ref x => x.status());
        let parent_endpoint = status
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
    ) -> Result<ApiClient> {
        let ns = resource.namespace();

        let instance = Retriever::<InstanceRef>::get(
            client.clone(),
            &resource.inner_spec().endpoint.instance_ref,
            &ns,
        )
        .await?;

        for_both!(instance, ref instance => K8sKeycloakBuilder::new(instance, client))
            .with_token()
            .await
    }

    async fn get_workflow(
        &self,
        keycloak: ApiClient,
        _resource: &R,
        path: &str,
        _payload: &Value,
    ) -> Result<String> {
        // TODO: The only instance where the get workflow is used is for
        // service-users of clients. So this is tailored to that use case
        // and not generic
        if !path.starts_with("/admin/realms/")
            || !path.ends_with("/service-account-user")
        {
            return Err(Error::UnsupportedWorkflowMethod);
        }
        let user = keycloak.get::<UserRepresentation>(path).await?;
        let Some(user_id) = user.id.as_ref() else {
            return Err(Error::MissingField(
                KeycloakUserSpec::ID_FIELD.to_string(),
            ));
        };

        // (0)/(1)admin/(2)realms/(3)<realm_name>/(_)clients/(_)<client_id>/(_)service-account-user
        // =>
        // /admin/realms/<realm_name>/users/<user_id>
        let realm_path = path.split('/').take(4).collect::<Vec<_>>().join("/");
        let resource_path = format!("{}/users/{user_id}", realm_path);

        keycloak.put(&resource_path, &user).await?;

        Ok(resource_path)
    }

    async fn post_workflow(
        &self,
        keycloak: ApiClient,
        path: &str,
        payload: &Value,
    ) -> Result<String> {
        Ok(keycloak.post_location(path, &payload).await?)
    }
}

#[async_trait]
impl<R> LifecycleController for ApiObjectController<R>
where
    R: Resource<DynamicType = ()>
        + HasInnerSpec<InnerSpec = KeycloakApiObjectSpec>
        + HasStatus<Status = KeycloakApiStatus>
        + Send
        + Sync
        + Clone
        + Debug
        + DeserializeOwned
        + 'static,
    ApiExt<R>: ApiFactory,
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
        let ns = resource.namespace();
        let name = resource.name_unchecked();
        let api = ApiExt::<R>::api(client.clone(), &ns);
        let keycloak = Self::keycloak(client, &resource).await?;
        let spec = resource.inner_spec();
        let mut payload = serde_yaml::from_str(&spec.payload)?;

        if let Some(options) = spec.options.as_ref() {
            let mut patcher =
                JsonPatcher::with_client(client.clone(), ns.clone());
            for patch in options.patch_from.iter() {
                patcher.patch(&mut payload, patch).await?;

                match &patch.value_from {
                    ValueFrom::SecretKeyRef(secret_key_ref) => {
                        self.secret_refs
                            .add(&resource, [secret_key_ref.name.clone()]);
                    }
                    ValueFrom::ConfigMapKeyRef(config_map_key_ref) => {
                        self.config_map_refs
                            .add(&resource, [config_map_key_ref.name.clone()]);
                    }
                    ValueFrom::Value(_) => {}
                }
            }
        }

        let immutable_payload: Value =
            serde_yaml::from_str(&spec.immutable_payload.0)?;
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
                Err(keycloak_client::Error::ResponseError(
                    StatusCode::NOT_FOUND,
                    m,
                )) => {
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

        if !success {
            let path = self.resolve_path(client, &ns, &resource).await?;
            let resource_path =
                match resource.inner_spec().endpoint.init_workflow {
                    Some(keycloak_client::Method::GET) => {
                        self.get_workflow(keycloak, &resource, &path, &payload)
                            .await?
                    }
                    Some(keycloak_client::Method::POST) | None => {
                        self.post_workflow(keycloak, &path, &payload).await?
                    }
                    _ => Err(Error::UnsupportedWorkflowMethod)?,
                };
            let mut status = resource.status().cloned().unwrap_or_default();
            let realm_ref = spec.endpoint.realm.clone();
            status.endpoint = Some(KeycloakApiStatusEndpoint {
                realm: realm_ref,
                resource_path,
                instance: resource.inner_spec().endpoint.instance_ref.clone(),
            });

            api.patch_status(
                &name,
                &PatchParams::apply(app_id!()),
                &status.to_patch(),
            )
            .await?;
        }

        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let kind = R::kind(&());
        let name = resource.name_unchecked();
        let ns = resource.namespace();
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
            Err(keycloak_client::Error::ResponseError(
                StatusCode::NOT_FOUND,
                m,
            )) => {
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
