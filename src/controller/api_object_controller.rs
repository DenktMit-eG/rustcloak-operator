use super::controller_runner::LifecycleController;
use crate::crd::KeycloakApiObject;
use crate::util::RefWatcher;
use crate::{
    api::KeycloakApiClient,
    app_id,
    crd::{KeycloakApiStatus, KeycloakInstance},
    error::{Error, Result},
    util::K8sKeycloakBuilder,
};
use async_stream::stream;
use async_trait::async_trait;
use k8s_openapi::{
    api::core::v1::{ConfigMap, Secret},
    DeepMerge,
};
use kube::runtime::watcher;
use kube::{
    api::PatchParams,
    runtime::{controller::Action, Controller},
    Api, ResourceExt,
};
use log::trace;
use reqwest::Url;
use reqwest::{header::LOCATION, Method, Response};
use serde_json::Value;
use std::str::FromStr;
use std::{ops::Deref, sync::Arc};
use tokio::sync::Notify;

/// This controller is responsible for applying the desired keycloak state in kubernetes to a
/// keycloak instance.
#[derive(Debug, Default)]
pub struct KeycloakApiObjectController {
    reconcile_notify: Arc<Notify>,
    secret_refs: Arc<RefWatcher<KeycloakApiObject, Secret>>,
    config_map_refs: Arc<RefWatcher<KeycloakApiObject, ConfigMap>>,
}

impl KeycloakApiObjectController {
    async fn keycloak(
        client: &kube::Client,
        resource: &KeycloakApiObject,
    ) -> Result<KeycloakApiClient> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let instance_api =
            Api::<KeycloakInstance>::namespaced(client.clone(), &ns);

        let instance_ref = &resource.spec.endpoint.instance_ref;
        let instance = instance_api
            .get_opt(instance_ref)
            .await?
            .ok_or(Error::NoInstance(ns, instance_ref.to_string()))?;

        K8sKeycloakBuilder::new(&instance, client)
            .with_token()
            .await
    }

    async fn request(
        &self,
        client: &KeycloakApiClient,
        method: Method,
        path: &str,
        payload: &Value,
    ) -> Result<Response> {
        let request = client.request(method, path);
        let request = if payload == &Value::Null {
            request
        } else {
            request.json(payload)
        };
        trace!("Request: {:?}", request);

        Ok(request.send().await?.error_for_status()?)
    }
}

#[async_trait]
impl LifecycleController for KeycloakApiObjectController {
    type Resource = KeycloakApiObject;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource> {
        let notify = self.reconcile_notify.clone();
        let secret_refs = self.secret_refs.clone();
        let config_map_refs = self.config_map_refs.clone();
        let secret_api = Api::<Secret>::all(client.clone());
        let config_map_api = Api::<ConfigMap>::all(client.clone());
        controller
            .reconcile_all_on(stream! {
                loop {
                    notify.notified().await;
                    yield;
                }
            })
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
        let api = Api::<KeycloakApiObject>::namespaced(client.clone(), &ns);
        let keycloak = Self::keycloak(client, &resource).await?;
        let mut payload = resource.resolve(client).await?;
        let immutable_payload = resource.spec.immutable_payload.deref();
        payload.merge_from(immutable_payload.clone());

        if let Some(path) = resource
            .status
            .as_ref()
            .and_then(|x| x.resource_path.as_ref())
        {
            self.request(&keycloak, Method::PUT, path, &payload).await?;

            api.patch_status(
                &name,
                &PatchParams::apply(app_id!()),
                &KeycloakApiStatus::ok("Applied").into(),
            )
            .await?;
        } else {
            let path = &resource.spec.endpoint.path;
            let response = self
                .request(&keycloak, Method::POST, path, &payload)
                .await?;
            let resource_url = Url::from_str(
                response
                    .headers()
                    .get(LOCATION)
                    .ok_or(Error::NoLocationHeader)?
                    .to_str()?,
            )?;
            let mut status = resource.status.clone().unwrap_or_default();
            status.resource_path = Some(resource_url.path().to_string());

            api.patch_status(
                &name,
                &PatchParams::apply(app_id!()),
                &status.into(),
            )
            .await?;
        }

        let ref_iter = resource
            .spec
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
        let Some(path) = resource
            .status
            .as_ref()
            .and_then(|x| x.clone().resource_path)
        else {
            return Ok(Action::await_change());
        };
        let keycloak = Self::keycloak(client, &resource).await?;
        let res = self
            .request(&keycloak, Method::DELETE, &path, &Value::Null)
            .await?;
        if res.status().as_u16() != 404 {
            res.error_for_status()?;
        }

        self.secret_refs.remove(&resource);
        self.config_map_refs.remove(&resource);

        // If we delete a resource, we let the controller reconcile all objects, so that
        // the resources that depend on the deleted resource are starting to fail.
        self.reconcile_notify.notify_one();

        Ok(Action::await_change())
    }
}
