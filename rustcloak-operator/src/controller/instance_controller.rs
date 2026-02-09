use std::sync::Arc;

use crate::{
    app_id,
    controller::controller_runner::LifecycleController,
    error::{Error, Result},
    util::{
        ApiExt, ApiFactory, K8sKeycloakRefreshManager, RefWatcher, ToPatch,
    },
};
use async_trait::async_trait;
use k8s_openapi::{ByteString, api::core::v1::Secret};
use kube::{
    Api, Resource, ResourceExt,
    api::{ListParams, ObjectMeta, PatchParams, PostParams},
    core::object::HasStatus,
    runtime::{Controller, controller::Action, watcher},
};
use log::debug;
use randstr::randstr;
use rustcloak_crd::{
    KeycloakApiStatus,
    api_object::{ClusterKeycloakApiObject, KeycloakApiObject},
    inner_spec::HasInnerSpec,
    instance::{
        ClusterKeycloakInstance, KeycloakInstance, KeycloakInstanceSpec,
    },
    traits::SecretKeyNames,
};
use serde::de::DeserializeOwned;

shorter_bounds::alias!(
    pub trait Instance: Resource<DynamicType = ()>
        + RefName
        + HasStatus<Status = KeycloakApiStatus>
        + HasInnerSpec<InnerSpec = KeycloakInstanceSpec>
        + Send
        + Sync
        + std::fmt::Debug
        + Clone
        + DeserializeOwned
        + 'static
);

trait RefName {
    const REF_NAME: &'static str;
}

impl RefName for KeycloakInstance {
    const REF_NAME: &'static str = app_id!("instanceRef");
}

impl RefName for ClusterKeycloakInstance {
    const REF_NAME: &'static str = app_id!("clusterInstanceRef");
}

#[derive(Debug)]
pub struct InstanceController<R>
where
    R: Instance,
{
    manager: K8sKeycloakRefreshManager<R>,
    secret_refs: Arc<RefWatcher<R, Secret>>,
}

impl<R> Default for InstanceController<R>
where
    R: Instance,
{
    fn default() -> Self {
        Self {
            manager: K8sKeycloakRefreshManager::default(),
            secret_refs: Arc::new(RefWatcher::default()),
        }
    }
}

impl<R> InstanceController<R>
where
    R: Instance,
{
    fn secret_namespace(resource: &R) -> Option<String> {
        let spec = resource.inner_spec();
        if let Some(ns) = resource.namespace() {
            Some(ns)
        } else {
            spec.credentials.namespace.clone()
        }
    }
    async fn create_secret(
        &self,
        client: &kube::Client,
        resource: Arc<R>,
    ) -> Result<()> {
        let spec = resource.inner_spec();
        let secret_name = &spec.credentials.secret_name;
        let ns = Self::secret_namespace(&resource);
        let secret_api = ApiExt::<Secret>::api(client.clone(), &ns);
        let [username_key, password_key] = spec.credentials.secret_key_names();

        let username = "rustcloak-admin".to_string();
        let password = randstr()
            .must_upper()
            .must_lower()
            .must_digit()
            .must_symbol()
            .len(32)
            .build()
            .generate();
        let data = [
            (username_key.to_string(), ByteString(username.into_bytes())),
            (password_key.to_string(), ByteString(password.into_bytes())),
        ]
        .into();
        let owner_ref = resource.owner_ref(&()).unwrap();

        let secret = Secret {
            data: Some(data),
            metadata: ObjectMeta {
                name: Some(secret_name.to_string()),
                owner_references: Some(vec![owner_ref]),
                ..Default::default()
            },
            type_: Some("Opaque".to_string()),
            ..Default::default()
        };

        secret_api.create(&PostParams::default(), &secret).await?;
        Ok(())
    }

    async fn cleanup_api_objects<A>(
        &self,
        client: &kube::Client,
        ns: &Option<String>,
        resource_name: &str,
    ) -> Result<()>
    where
        ApiExt<A>: ApiFactory<Resource = A>,
        A: Resource,
    {
        let selector = format!("{}={}", R::REF_NAME, resource_name);
        let api = ApiExt::<KeycloakApiObject>::api(client.clone(), ns);
        let list = api
            .list_metadata(&ListParams::default().labels(&selector))
            .await?;
        if !list.items.is_empty() {
            let items = list.items.iter().map(|item| item.name_any()).collect();
            return Err(Error::ResourceInUseForDeletion(items));
        }
        Ok(())
    }
}

#[async_trait]
impl<R> LifecycleController for InstanceController<R>
where
    R: Instance,
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
        let secret_api = Api::<Secret>::all(client.clone());
        controller
            .owns(secret_api.clone(), watcher::Config::default())
            .watches(secret_api, watcher::Config::default(), move |secret| {
                secret_refs.watch(&secret)
            })
    }

    async fn before_finalizer(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<bool> {
        let spec = resource.inner_spec();
        debug!("Scheduling refresh for resource {}", resource.name_any());
        match self
            .manager
            .schedule_refresh(&resource, client.clone())
            .await
        {
            Err(Error::NoCredentialSecret(x, y)) => {
                if spec.credentials.create.unwrap_or(false) {
                    self.create_secret(client, resource.clone()).await?;
                } else {
                    Err(Error::NoCredentialSecret(x, y))?;
                }
            }
            x => x?,
        };

        Ok(true)
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let ns = resource.namespace();
        let spec = resource.inner_spec();
        let api = ApiExt::<Self::Resource>::api(client.clone(), &ns);

        self.secret_refs
            .add(&resource, [spec.credential_secret_name()]);

        api.patch_status(
            &resource.name_unchecked(),
            &PatchParams::apply(app_id!()),
            &KeycloakApiStatus::ok("Authenticated").to_patch(),
        )
        .await?;

        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let ns = resource.namespace();

        let resource_name = resource.name_any();
        self.cleanup_api_objects::<KeycloakApiObject>(
            client,
            &ns,
            &resource_name,
        )
        .await?;
        self.cleanup_api_objects::<ClusterKeycloakApiObject>(
            client,
            &ns,
            &resource_name,
        )
        .await?;

        self.manager.cancel_refresh(&resource).await?;
        self.secret_refs.remove(&resource);
        Ok(Action::await_change())
    }
}
