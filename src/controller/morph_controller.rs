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
use serde::de::DeserializeOwned;
use serde_json::Value;

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

        let mut payload = resource.payload()?;
        let immutable_payload = Value::Object(
            R::IMMUTABLE_FIELDS
                .iter()
                .filter_map(|f| {
                    let v = payload.as_object_mut().unwrap().remove(*f);
                    v.map(|v| (f.to_string(), v))
                })
                .collect::<serde_json::Map<_, _>>(),
        )
        .into();
        let payload = payload.into();

        let path = resource.create_path(client.clone()).await?.into();

        let api_object = KeycloakApiObject {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: Some(ns.clone()),
                owner_references: Some(vec![owner_ref]),
                ..Default::default()
            },
            spec: KeycloakApiObjectSpec {
                path,
                api: resource.api().clone(),
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
