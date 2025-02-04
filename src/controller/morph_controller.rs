use std::sync::Arc;

use super::controller_runner::LifecycleController;
use crate::{
    app_id,
    crd::{
        HasApiObject, HasRoute, KeycloakApiEndpoint, KeycloakApiObject,
        KeycloakApiObjectSpec, KeycloakApiStatus,
    },
    endpoint::{path::Path, query::Query},
    error::{Error, Result},
    morph::{Morph, Patcher},
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
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use up_impl::{Container, HasContainer, HasQuery, HasUp, Up};

#[derive(Debug)]
pub struct MorphController<T> {
    phantom: std::marker::PhantomData<T>,
}

impl<T> Default for MorphController<T> {
    fn default() -> Self {
        Self {
            phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<R> LifecycleController for MorphController<R>
where
    R: Send
        + Sync
        + Serialize
        + DeserializeOwned
        + std::fmt::Debug
        + Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + HasApiObject
        + HasRoute
        + HasStatus<Status = KeycloakApiStatus>
        + Clone
        + HasUp<Up: HasContainer>
        + HasContainer<Container: Container>
        + HasQuery<Query = Query<R, String>>
        + 'static,
    Up<R>: Path + Send + Sync,
    <R::Up as HasContainer>::Container: Container<
        UserData = (kube::Client, String),
        Key = R::UpKey,
        Error = Error,
    >,
    R::UpKey: Send + Sync,
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
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let admin_api: Api<KeycloakApiObject> =
            Api::namespaced(client.clone(), &ns);
        let kind = Self::Resource::kind(&());
        let name = format!("{}-{}", R::prefix(), resource.name_unchecked());
        let owner_ref = resource.owner_ref(&()).unwrap();

        let primary_key = R::id_ident();
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

        let resource = Arc::unwrap_or_clone(resource);
        let resource =
            Up::<Self::Resource>::with((client.clone(), ns.clone()), resource)
                .await?;
        let Some(endpoint_path) = resource.endpoint_path() else {
            // TODO: proper error handling
            return Err(Error::NoData);
        };
        let instance_ref = resource.instance_ref().to_string();
        let endpoint = KeycloakApiEndpoint::new(&instance_ref, &endpoint_path);
        debug!(
            kind = kind, name = resource.name_unchecked(), namesapce = ns;
            "Resolved endpoint: {:?}",
            endpoint
        );

        let api_object = KeycloakApiObject {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: Some(ns.clone()),
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
                &api_status.into(),
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
