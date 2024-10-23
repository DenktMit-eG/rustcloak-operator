use crate::{
    api::{KeycloakAuth, KeycloakAuthBuilder, KeycloakClient, OAuth2Token},
    app_id,
    crd::{KeycloakApiStatus, KeycloakInstance},
    error::{Error, Result},
};
use chrono::{DateTime, Utc};
use http::Method;
use k8s_openapi::api::core::v1::Secret;
use kube::{
    api::{Patch, PatchParams},
    core::object::HasStatus,
    Api, ResourceExt,
};
use log::{debug, info, warn};
use oauth2::{ClientId, ClientSecret};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{
    sync::{Mutex, MutexGuard, Notify},
    task::JoinHandle,
    time,
};

use super::SecretUtils;

pub struct K8sKeycloakBuilder<'a> {
    auth: KeycloakAuth,
    instance: &'a KeycloakInstance,
    client: &'a kube::Client,
}

impl<'a> K8sKeycloakBuilder<'a> {
    pub fn new(
        instance: &'a KeycloakInstance,
        client: &'a kube::Client,
    ) -> Self {
        let mut builder = KeycloakAuthBuilder::default();
        builder.url(instance.spec.base_url.clone());
        if let Some(realm) = &instance.spec.realm {
            builder.realm(realm.clone());
        }

        if let Some(client) = &instance.spec.client {
            builder.client_id(ClientId::new(client.id.clone()));
            if let Some(secret) = &client.secret {
                builder.client_secret(ClientSecret::new(secret.clone()));
            }
        }

        K8sKeycloakBuilder {
            auth: builder.build().unwrap(),
            instance,
            client,
        }
    }

    pub async fn with_credentials(self) -> Result<KeycloakClient> {
        let ns = self.instance.namespace().ok_or(Error::NoNamespace)?;
        let secret_api = Api::<Secret>::namespaced(self.client.clone(), &ns);
        let credential_secret_name =
            self.instance.credential_secret_name().to_string();
        debug!( "Using keycloak with credential secret {ns}/{credential_secret_name}");

        let (user, password) = secret_api
            .get_opt(&credential_secret_name)
            .await?
            .ok_or(Error::NoCredentialSecret(ns, credential_secret_name))?
            .credentials(&self.instance.spec.credentials)?;

        self.auth.login_with_credentials(&user, &password).await
    }

    pub async fn with_token(self) -> Result<KeycloakClient> {
        let ns = self.instance.namespace().ok_or(Error::NoNamespace)?;
        let secret_api = Api::<Secret>::namespaced(self.client.clone(), &ns);
        let token_secret_name = self.instance.token_secret_name().to_string();
        debug!("Using keycloak with token secret {ns}/{token_secret_name}");

        let token = secret_api
            .get_opt(&token_secret_name)
            .await?
            .ok_or(Error::NoTokenSecret(ns, token_secret_name))?
            .token(self.instance)?;

        Ok(self.auth.into_client(token))
    }
}

pub struct K8sKeycloakRefreshJob {
    instance: Arc<KeycloakInstance>,
    client: kube::Client,
    stopper: Arc<Notify>,
}

impl K8sKeycloakRefreshJob {
    pub fn new(instance: Arc<KeycloakInstance>, client: kube::Client) -> Self {
        let stopper = Arc::new(Notify::new());
        Self {
            instance,
            client,
            stopper,
        }
    }

    pub fn stopper(&self) -> Arc<Notify> {
        self.stopper.clone()
    }

    pub fn keycloak_builder(&self) -> K8sKeycloakBuilder {
        K8sKeycloakBuilder::new(&self.instance, &self.client)
    }

    pub async fn update_token_secret(&self, token: &OAuth2Token) -> Result<()> {
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

    pub async fn keycloak_with_credentials(&self) -> Result<KeycloakClient> {
        let keycloak = self.keycloak_builder().with_credentials().await?;

        self.update_token_secret(keycloak.token()).await?;

        Ok(keycloak)
    }

    pub async fn refresh(&self, keycloak: &mut KeycloakClient) -> Result<()> {
        keycloak.refresh().await?;

        self.update_token_secret(keycloak.token()).await?;

        Ok(())
    }

    pub async fn keycloak_with_token(&self) -> Result<KeycloakClient> {
        let mut keycloak = self.keycloak_builder().with_token().await?;
        let now = chrono::Utc::now();

        if keycloak.token().expires.map_or(false, |e| now > e) {
            self.refresh(&mut keycloak).await?;
        } else {
            // ping keycloak to make sure it's available
            keycloak
                .request(Method::GET, "")
                .send()
                .await?
                .error_for_status()?;
        }

        Ok(keycloak)
    }

    pub async fn keycloak_from_somewhere(&self) -> Result<KeycloakClient> {
        debug!("Trying to get keycloak client, trying token first.");
        match self.keycloak_with_token().await {
            Err(e) => {
                debug!("{}", e);
                debug!(
                    "Trying to get keycloak client, trying credentials next."
                );
                self.keycloak_with_credentials().await
            }
            ok => ok,
        }
    }

    pub async fn wait(&self, duration: Duration) -> Result<bool> {
        match time::timeout(duration, self.stopper.notified()).await {
            Ok(_) => {
                debug!("Stop notification received");
                Ok(false)
            }
            Err(_) => Ok(true),
        }
    }

    pub async fn wait_for_expire(
        &self,
        expires: DateTime<Utc>,
    ) -> Result<bool> {
        let now = chrono::Utc::now();
        let timeout = if expires > now {
            let timeout = (expires - now) * 5 / 6;
            timeout.to_std().unwrap()
        } else if self.instance.status().map_or(true, |s| s.ready) {
            debug!("Token already expired, refreshing now.");
            Duration::from_secs(0)
        } else {
            info!("Token is expired, but instance is not ready, waiting 5 seconds");
            Duration::from_secs(5)
        };
        info!(
            "Next token refresh at {expires} ({} seconds)",
            timeout.as_secs()
        );
        self.wait(timeout).await
    }

    async fn run_expire(&self, expires: DateTime<Utc>) -> Result<()> {
        if !(self.wait_for_expire(expires).await?) {
            return Ok(());
        }

        let mut keycloak = self.keycloak_builder().with_token().await?;
        match self.refresh(&mut keycloak).await {
            Ok(_) => info!("Token refreshed"),
            Err(e) => {
                warn!("Error refreshing token: {}, trying to login", e);
                self.keycloak_with_credentials().await?;
            }
        }
        Ok(())
    }
}

type RefreshStoreInner =
    HashMap<(String, String), (JoinHandle<()>, Arc<Notify>)>;

#[derive(Default, Debug)]
struct RefreshStore(RefreshStoreInner);

#[derive(Default, Debug)]
pub struct K8sKeycloakRefreshManager {
    jobs: Mutex<RefreshStore>,
}

impl Drop for RefreshStore {
    fn drop(&mut self) {
        for (_, (handle, _)) in self.0.drain() {
            handle.abort();
        }
    }
}

impl K8sKeycloakRefreshManager {
    pub async fn schedule_refresh(
        &self,
        session_handler: K8sKeycloakRefreshJob,
    ) -> Result<()> {
        let instance = &session_handler.instance;
        let ns = instance.namespace().ok_or(Error::NoNamespace)?;
        let name = instance.name_unchecked();
        let instance_api = Api::<KeycloakInstance>::namespaced(
            session_handler.client.clone(),
            &ns,
        );
        let key = (ns.to_string(), name.to_string());

        let mut jobs = self.jobs.lock().await;
        self.cancel_refresh_locked(&mut jobs, &session_handler)
            .await?;

        let keycloak = session_handler.keycloak_from_somewhere().await?;

        if let Some(expires) = keycloak.token().expires {
            info!("Scheduling token refresh for {}/{}", ns, name);
            let stopper = session_handler.stopper();
            let handle = tokio::spawn(async move {
                if let Err(e) = session_handler.run_expire(expires).await {
                    log::error!("Error in keycloak session handler: {}", e);
                    if let Err(e) = instance_api
                        .patch_status(
                            &name,
                            &PatchParams::apply(app_id!()),
                            &KeycloakApiStatus::from(e).into(),
                        )
                        .await
                    {
                        log::error!("Error updating status: {}", e);
                    }
                }
            });
            jobs.0.insert(key, (handle, stopper));
        }

        Ok(())
    }

    async fn cancel_refresh_locked(
        &self,
        jobs: &mut MutexGuard<'_, RefreshStore>,
        keycloak: &K8sKeycloakRefreshJob,
    ) -> Result<()> {
        let name = keycloak.instance.name_unchecked();
        let ns = keycloak.instance.namespace().ok_or(Error::NoNamespace)?;
        let key = (ns.to_string(), name.to_string());

        if let Some((handle, stopper)) = jobs.0.remove(&key) {
            stopper.notify_one();
            let _ = handle.await;
        }
        Ok(())
    }

    pub async fn cancel_refresh(
        &self,
        keycloak: K8sKeycloakRefreshJob,
    ) -> Result<()> {
        let mut jobs = self.jobs.lock().await;
        self.cancel_refresh_locked(&mut jobs, &keycloak).await
    }
}
