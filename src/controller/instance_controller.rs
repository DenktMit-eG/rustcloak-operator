use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{
    api::{KeycloakClient, OAuth2Token},
    app_id,
    crd::KeycloakApiStatus,
    error::{Error, Result},
    util::{K8sKeycloakBuilder, SecretUtils},
};
use async_trait::async_trait;
use k8s_openapi::api::core::v1::Secret;
use kube::{
    api::{Patch, PatchParams},
    runtime::{controller::Action, Controller},
    Api, ResourceExt,
};
use log::{debug, info, warn};
use tokio::{sync::Mutex, task::JoinHandle};

use super::controller_runner::LifetimeController;
use crate::crd::KeycloakInstance;

struct KeycloakSessionHandler {
    instance: Arc<KeycloakInstance>,
    client: kube::Client,
}

impl KeycloakSessionHandler {
    fn new(instance: Arc<KeycloakInstance>, client: kube::Client) -> Self {
        Self { instance, client }
    }

    fn keycloak_builder(&self) -> K8sKeycloakBuilder {
        K8sKeycloakBuilder::new(&self.instance, &self.client)
    }

    async fn update_token_secret(&self, token: &OAuth2Token) -> Result<()> {
        let ns = self.instance.namespace().ok_or(Error::NoNamespace)?;
        let api = Api::<Secret>::namespaced(self.client.clone(), &ns);
        let token_secret = Secret::from_token(token, &self.instance);

        api.patch(
            &token_secret.name_unchecked(),
            &PatchParams::apply(app_id!()),
            &Patch::Apply(token_secret),
        )
        .await?;

        Ok(())
    }

    async fn keycloak_from_credentials(&self) -> Result<KeycloakClient> {
        let keycloak = self.keycloak_builder().with_credentials().await?;

        self.update_token_secret(keycloak.token()).await?;

        Ok(keycloak)
    }

    async fn refresh(&self, keycloak: KeycloakClient) -> Result<()> {
        let mut keycloak = keycloak;
        keycloak.refresh().await?;

        self.update_token_secret(keycloak.token()).await?;

        Ok(())
    }

    async fn keycloak_from_somewhere(&self) -> Result<KeycloakClient> {
        debug!("Trying to get keycloak client, trying token first.");
        match self.keycloak_builder().with_token().await {
            Ok(keycloak) => {
                return Ok(keycloak);
            }
            Err(e) => debug!("{}", e),
        }

        debug!("Trying to get keycloak client, trying credentials next.");
        match self.keycloak_from_credentials().await {
            Ok(keycloak) => {
                return Ok(keycloak);
            }
            Err(e) => debug!("{}", e),
        }

        Err(Error::NoSecret)
    }

    async fn wait_for_expire(&self, keycloak: &KeycloakClient) -> Result<()> {
        let Some(expires) = keycloak.token().expires else {
            return Ok(());
        };

        let now = chrono::Utc::now();
        if expires > now {
            let timeout = (expires - now) * 5 / 6;
            let timeout = timeout.to_std().unwrap();
            info!(
                "Next token refresh at {expires} ({} seconds)",
                timeout.as_secs()
            );
            tokio::time::sleep(timeout).await;
        }
        Ok(())
    }

    async fn run_once(&self) -> Result<()> {
        let keycloak = self.keycloak_from_somewhere().await?;

        self.wait_for_expire(&keycloak).await?;

        match self.refresh(keycloak).await {
            Ok(_) => info!("Token refreshed"),
            Err(e) => {
                warn!("Error refreshing token: {}, trying to login", e);
                self.keycloak_from_credentials().await?;
            }
        }
        Ok(())
    }

    async fn run(self) -> Result<()> {
        let ns = self.instance.namespace().ok_or(Error::NoNamespace)?;

        info!(
            "Starting refresh loop for keycloak instance {}/{}",
            ns,
            self.instance.name_unchecked()
        );
        loop {
            match self.run_once().await {
                Ok(_) => {}
                Err(e) => {
                    warn!("Error in keycloak session handler: {}, sleeping for 5 seconds", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }
    }
}

type JoinHandlesInner = HashMap<(String, String), JoinHandle<()>>;

#[derive(Debug, Default)]
struct JoinHandles(JoinHandlesInner);

impl Drop for JoinHandles {
    fn drop(&mut self) {
        for (_, handle) in self.0.drain() {
            handle.abort();
        }
    }
}

impl AsRef<JoinHandlesInner> for JoinHandles {
    fn as_ref(&self) -> &JoinHandlesInner {
        &self.0
    }
}

impl Deref for JoinHandles {
    type Target = JoinHandlesInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JoinHandles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Default)]
pub struct KeycloakInstanceController {
    refresh_job_handles: Mutex<JoinHandles>,
}

#[async_trait]
impl LifetimeController for KeycloakInstanceController {
    type Resource = KeycloakInstance;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        _client: &kube::Client,
    ) -> Controller<Self::Resource> {
        controller
        /*controller.owns(
            Api::<Secret>::all(client.clone()),
            watcher::Config::default(),
        )*/
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let name = resource.name_unchecked();
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let session_handler =
            KeycloakSessionHandler::new(resource.clone(), client.clone());
        let instance_api =
            Api::<KeycloakInstance>::namespaced(client.clone(), &ns);

        let handle = tokio::spawn(async move {
            if let Err(e) = session_handler.run().await {
                log::error!("Error in keycloak session handler: {}", e);
                let _ = instance_api
                    .patch_status(
                        &resource.name_unchecked(),
                        &PatchParams::apply(app_id!()),
                        &Patch::Merge(KeycloakApiStatus::from_error(e)),
                    )
                    .await;
            }
        });

        if let Some(old_handle) = self
            .refresh_job_handles
            .lock()
            .await
            .insert((name, ns), handle)
        {
            old_handle.abort();
        }
        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        _client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let name = resource.name_unchecked();
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        if let Some(handle) =
            self.refresh_job_handles.lock().await.remove(&(name, ns))
        {
            handle.abort();
        }
        Ok(Action::await_change())
    }
}
