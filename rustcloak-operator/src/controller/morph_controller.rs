use std::{fmt::Debug, sync::Arc};

use crate::{
    app_id,
    controller::controller_runner::LifecycleController,
    error::{Error, Result},
    morph::Patcher,
    shim::resource::{
        InstanceShim, ParentShim, ResourceMarker, ResourceShim, RestShim,
    },
    util::ToPatch,
};
use async_trait::async_trait;
use k8s_openapi::NamespaceResourceScope;
use kube::{
    api::{ObjectMeta, Patch, PatchParams},
    core::object::HasStatus,
    runtime::{controller::Action, watcher, Controller},
    Api, Resource, ResourceExt,
};
use log::debug;
use rustcloak_crd::{
    traits::Endpoint, KeycloakApiEndpoint, KeycloakApiEndpointPath,
    KeycloakApiObject, KeycloakApiObjectSpec, KeycloakApiStatus,
    KeycloakRestObject,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

#[derive(Debug)]
pub struct MorphController<R, M = ResourceMarker> {
    phantom: std::marker::PhantomData<(R, M)>,
}

impl<R, M> Default for MorphController<R, M> {
    fn default() -> Self {
        Self {
            phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<R, M> LifecycleController for MorphController<R, M>
where
    R: Send
        + Sync
        + Serialize
        + DeserializeOwned
        + Debug
        + Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + HasStatus<Status = KeycloakApiStatus>
        + KeycloakRestObject
        + Clone
        + Endpoint
        + 'static,
    R::Definition: Send + Sync + Serialize + DeserializeOwned,
    R::ParentRef: AsRef<str>,
    R::ParentObject: Send + Sync + Clone + Debug + DeserializeOwned + Endpoint,
    ResourceShim<R>: ParentShim<M>,
    <ResourceShim<R> as ParentShim<M>>::Parent: Endpoint + Send + Sync,
    M: Send + Sync,
{
    type Resource = R;
    const MODULE_PATH: &'static str = module_path!();

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource> {
        controller.owns(
            Api::<KeycloakApiObject>::all(client.clone()),
            watcher::Config::default(),
        )
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let resource = ResourceShim::new(&resource, &client);
        let ns = resource.namespace()?;
        let admin_api: Api<KeycloakApiObject> =
            Api::namespaced(client.clone(), &ns);
        let kind = Self::Resource::kind(&());
        let name = resource.api_name()?;
        let owner_ref = resource.owner_ref(&()).unwrap();

        let primary_key = R::ID_FIELD;
        let mut payload = resource.payload()?;
        payload
            .as_object_mut()
            .as_mut()
            .unwrap()
            .remove(primary_key);
        let primary_key_value = resource.id();
        let immutable_payload = serde_yaml::to_string(&json!({
            primary_key: primary_key_value,
        }))?
        .into();
        let mut patcher = Patcher::new(payload);
        for (path, patch) in resource
            .patches()
            .map(|x| x.patch_from.iter())
            .unwrap_or_default()
        {
            patcher.patch(path, patch)?;
        }
        let vars = patcher.vars;
        let payload = serde_yaml::to_string(&patcher.value)?;

        let parent = resource.parent().await?;
        let instance_ref = if let Ok(instance_ref) = resource.instance_ref() {
            instance_ref
        } else {
            parent.instance_ref().ok_or(Error::NoData)?
        };
        let resource_path = format!(
            "{}/{}",
            parent.resource_path().ok_or(Error::NoData)?,
            resource.mount_path()
        );

        let endpoint = KeycloakApiEndpoint {
            instance_ref: instance_ref.to_string().into(),
            path_def: KeycloakApiEndpointPath::Path(resource_path.into()),
        };
        debug!(
            kind = kind, name = resource.name_unchecked(), namesapce = ns;
            "Resolved endpoint: {:?}",
            endpoint
        );

        let api_object = KeycloakApiObject {
            metadata: ObjectMeta {
                name: Some(name.to_string()),
                namespace: Some(ns.to_string()),
                owner_references: Some(vec![owner_ref]),
                labels: Some(
                    [(
                        app_id!("instanceRef").to_string(),
                        endpoint.instance_ref.0.clone(),
                    )]
                    .into(),
                ),
                ..Default::default()
            },
            spec: KeycloakApiObjectSpec {
                endpoint,
                options: resource.options().cloned(),
                immutable_payload,
                payload,
                vars,
            },
            status: None,
        };

        admin_api
            .patch(
                &name,
                &PatchParams::apply(app_id!()),
                &Patch::Apply(api_object),
            )
            .await?;

        if let Some(api_status) =
            admin_api.get_status(&name).await?.status.clone()
        {
            let api = Api::<Self::Resource>::namespaced(client.clone(), &ns);
            api.patch_status(
                &resource.name_unchecked(),
                &PatchParams::apply(app_id!()),
                &api_status.to_patch(),
            )
            .await?;
        }
        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        _client: &kube::Client,
        _resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        Ok(Action::await_change())
    }
}
