use std::sync::Arc;

use super::controller_runner::LifecycleController;
use crate::{
    app_id,
    crd::{KeycloakApiObject, KeycloakApiObjectSpec},
    error::{Error, Result},
    morph::ToApiObject,
};
use async_trait::async_trait;
use kube::{
    api::{ObjectMeta, Patch, PatchParams},
    runtime::{controller::Action, watcher, Controller},
    Api, Resource, ResourceExt,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

#[derive(Debug)]
pub struct MorphController<T: ToApiObject> {
    phantom: std::marker::PhantomData<T>,
}

impl<T: ToApiObject> Default for MorphController<T> {
    fn default() -> Self {
        Self {
            phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<R> LifecycleController for MorphController<R>
where
    R: ToApiObject
        + Resource<DynamicType = ()>
        + Send
        + Sync
        + 'static
        + Clone
        + std::fmt::Debug
        + Serialize
        + DeserializeOwned,
{
    type Resource = R;

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
        let name = format!("{}{}", R::PREFIX, resource.name_unchecked());
        let owner_ref = resource.owner_ref(&()).unwrap();

        let primary_key = R::primary_key();
        let mut payload = serde_json::to_value(resource.definition())?;
        payload
            .as_object_mut()
            .as_mut()
            .unwrap()
            .remove(primary_key);
        let primary_key_value = resource.primary_key_value_r()?;
        let immutable_payload = json!({
            primary_key: primary_key_value,
        })
        .into();
        let payload = payload.into();

        let endpoint = resource.create_endpoint(client.clone()).await?;

        let api_object = KeycloakApiObject {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: Some(ns.clone()),
                owner_references: Some(vec![owner_ref]),
                ..Default::default()
            },
            spec: KeycloakApiObjectSpec {
                endpoint,
                options: resource.options().cloned(),
                immutable_payload,
                payload,
                vars: None,
            },
            status: Default::default(),
        };

        admin_api
            .patch(
                &name,
                &PatchParams::apply(app_id!()),
                &Patch::Apply(api_object),
            )
            .await?;

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
