use std::{ops::Deref, sync::Arc};

use crate::{
    api::KeycloakClient,
    app_id,
    crd::{KeycloakApiStatus, KeycloakInstance},
    error::{Error, Result},
    util::K8sKeycloakBuilder,
};
use async_stream::stream;
use async_trait::async_trait;
use k8s_openapi::DeepMerge;
use kube::{
    api::PatchParams,
    runtime::{controller::Action, Controller},
    Api, ResourceExt,
};
use log::trace;
use reqwest::{Method, Response};
use serde_json::Value;
use tokio::sync::Notify;

use super::controller_runner::LifecycleController;
use crate::crd::KeycloakApiObject;

/// This controller is responsible for applying the desired keycloak state in kubernetes to a
/// keycloak instance.
#[derive(Debug)]
pub struct KeycloakApiObjectController {
    reconcile_notify: Arc<Notify>,
}

impl Default for KeycloakApiObjectController {
    fn default() -> Self {
        let reconcile_notify = Arc::new(Notify::new());
        Self { reconcile_notify }
    }
}

impl KeycloakApiObjectController {
    async fn keycloak(
        client: &kube::Client,
        resource: &KeycloakApiObject,
    ) -> Result<KeycloakClient> {
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
        client: &KeycloakClient,
        method: Method,
        path: &str,
        payload: &Value,
    ) -> Result<Option<Response>> {
        let request = client.request(method, path);
        let request = if payload == &Value::Null {
            request
        } else {
            request.json(payload)
        };
        trace!("Request: {:?}", request);
        let res = request.send().await?;

        if res.status().as_u16() == 404 {
            Ok(None)
        } else {
            Ok(Some(res.error_for_status()?))
        }
    }
}

#[async_trait]
impl LifecycleController for KeycloakApiObjectController {
    type Resource = KeycloakApiObject;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        _client: &kube::Client,
    ) -> Controller<Self::Resource> {
        let notify = self.reconcile_notify.clone();
        controller.reconcile_all_on(stream! {
            loop {
                notify.notified().await;
                yield;
            }
        })
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let name = resource.name_unchecked();
        let path = &resource.spec.endpoint.path;
        let keycloak = Self::keycloak(client, &resource).await?;
        let mut payload = resource.resolve(client).await?;
        let immutable_payload = resource.spec.immutable_payload.deref();
        payload.merge_from(immutable_payload.clone());
        // First try to PUT, if we get a 404, try to POST
        if self
            .request(&keycloak, Method::PUT, path, &payload)
            .await?
            .is_none()
        {
            let path = path.rsplit_once('/').unwrap().0;
            self.request(&keycloak, Method::POST, path, &payload)
                .await?;
        }

        let api = Api::<KeycloakApiObject>::namespaced(client.clone(), &ns);
        api.patch_status(
            &name,
            &PatchParams::apply(app_id!()),
            &KeycloakApiStatus::ok("Applied").into(),
        )
        .await?;

        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let path = &resource.spec.endpoint.path;
        let keycloak = Self::keycloak(client, &resource).await?;
        self.request(&keycloak, Method::DELETE, path, &Value::Null)
            .await?;

        // If we delete a resource, we let the controller reconcile all objects, so that
        // the resources that depend on the deleted resource are starting to fail.
        self.reconcile_notify.notify_one();

        Ok(Action::await_change())
    }
}
