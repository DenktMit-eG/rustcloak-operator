use crate::{
    api::{
        KeycloakApiAuth, KeycloakApiAuthBuilder, KeycloakApiClient, OAuth2Token,
    },
    app_id,
    error::{Error, Result},
    util::ToPatch,
};
use chrono::{DateTime, Utc};
use k8s_openapi::api::core::v1::Secret;
use kube::{
    api::{Patch, PatchParams},
    core::object::HasStatus,
    runtime::reflector::ObjectRef,
    Api, Resource, ResourceExt,
};
use log::{debug, warn};
use oauth2::{ClientId, ClientSecret};
use rustcloak_crd::KeycloakInstance;
use serde_json::Value;
use std::{collections::HashMap, ops::Deref, sync::Arc, time::Duration};
use tokio::{
    sync::{Mutex, MutexGuard, Notify},
    task::JoinHandle,
    time,
};

use super::SecretUtils;

pub struct K8sKeycloakBuilder<'a> {
    auth: KeycloakApiAuth,
    instance: &'a KeycloakInstance,
    client: &'a kube::Client,
}

impl<'a> K8sKeycloakBuilder<'a> {
    pub fn new(
        instance: &'a KeycloakInstance,
        client: &'a kube::Client,
    ) -> Self {
        let mut builder = KeycloakApiAuthBuilder::default();
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

    pub async fn with_credentials(self) -> Result<KeycloakApiClient> {
        let ns = self.instance.namespace().ok_or(Error::NoNamespace)?;
        let kind = KeycloakInstance::kind(&());
        let secret_api = Api::<Secret>::namespaced(self.client.clone(), &ns);
        let credential_secret_name =
            self.instance.credential_secret_name().to_string();
        debug!(
            name = self.instance.name_unchecked(),
            namespace = ns,
            kind = kind;
            "Using keycloak with credential secret {credential_secret_name}"
        );

        let [user, password] = secret_api
            .get_opt(&credential_secret_name)
            .await?
            .ok_or(Error::NoCredentialSecret(ns, credential_secret_name))?
            .extract(&self.instance.spec.token)?;

        self.auth.login_with_credentials(&user, &password).await
    }

    pub async fn with_token(self) -> Result<KeycloakApiClient> {
        let ns = self.instance.namespace().ok_or(Error::NoNamespace)?;
        let kind = KeycloakInstance::kind(&());
        let secret_api = Api::<Secret>::namespaced(self.client.clone(), &ns);
        let token_secret_name = self.instance.token_secret_name().to_string();
        debug!(
            name = self.instance.name_unchecked(),
            namespace = ns,
            kind = kind;
            "Using keycloak with token secret {token_secret_name}"
        );

        let secret = secret_api
            .get_opt(&token_secret_name)
            .await?
            .ok_or(Error::NoTokenSecret(ns, token_secret_name))?;
        let token = OAuth2Token::from_secret(secret, self.instance);

        Ok(self.auth.into_client(token?))
    }
}

struct K8sKeycloakRefreshJob {
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
        let token_secret = token.clone().into_secret(&self.instance);
        api.patch(
            &token_secret.name_unchecked(),
            &PatchParams::apply(app_id!()),
            &Patch::Apply(token_secret),
        )
        .await?;

        Ok(())
    }

    pub async fn keycloak_with_credentials(&self) -> Result<KeycloakApiClient> {
        let keycloak = self.keycloak_builder().with_credentials().await?;

        self.update_token_secret(keycloak.token()).await?;

        Ok(keycloak)
    }

    pub async fn refresh(
        &self,
        keycloak: &mut KeycloakApiClient,
    ) -> Result<()> {
        keycloak.refresh().await?;

        self.update_token_secret(keycloak.token()).await?;

        Ok(())
    }

    pub async fn keycloak_with_token(&self) -> Result<KeycloakApiClient> {
        let mut keycloak = self.keycloak_builder().with_token().await?;
        let now = chrono::Utc::now();

        if keycloak.token().expires.is_some_and(|e| now > e) {
            self.refresh(&mut keycloak).await?;
        } else {
            // ping keycloak to make sure it's available
            keycloak.get::<Value>("admin/realms").await?;
        }

        Ok(keycloak)
    }

    pub async fn keycloak_from_somewhere(&self) -> Result<KeycloakApiClient> {
        let name = self.instance.name_unchecked();
        let ns = self.instance.namespace().ok_or(Error::NoNamespace)?;
        let kind = KeycloakInstance::kind(&());
        debug!(
            name = name,
            namespace = ns,
            kind = kind;
            "Trying to get keycloak client, trying token first."
        );
        match self.keycloak_with_token().await {
            Err(e) => {
                debug!(
                    name = name,
                    namespace = ns,
                    kind = kind;
                    "Error while getting secret from token: {e}; Trying credentials next.");
                self.keycloak_with_credentials().await
            }
            ok => ok,
        }
    }

    pub async fn wait(&self, duration: Duration) -> Result<bool> {
        match time::timeout(duration, self.stopper.notified()).await {
            Ok(_) => {
                let name = self.instance.name_unchecked();
                let ns = self.instance.namespace().ok_or(Error::NoNamespace)?;
                let kind = KeycloakInstance::kind(&());
                debug!(
                    name = name,
                    namespace = ns,
                    kind = kind;
                    "Stop notification received"
                );
                Ok(false)
            }
            Err(_) => Ok(true),
        }
    }

    pub async fn wait_for_expire(
        &self,
        expires: DateTime<Utc>,
    ) -> Result<bool> {
        let name = self.instance.name_unchecked();
        let ns = self.instance.namespace().ok_or(Error::NoNamespace)?;
        let kind = KeycloakInstance::kind(&());
        let now = chrono::Utc::now();
        let timeout = if expires > now {
            let timeout = (expires - now) * 5 / 6;
            timeout.to_std().unwrap()
        } else if self.instance.status().is_none_or(|s| s.ready) {
            debug!(
                name = name,
                namespace = ns,
                kind = kind;
                "Token already expired, refreshing now."
            );
            Duration::from_secs(0)
        } else {
            debug!(
                name = name,
                namespace = ns,
                kind = kind;
                "Token is expired, but instance is not ready, waiting 5 seconds"
            );
            Duration::from_secs(5)
        };
        debug!(
            name = name,
            namespace = ns,
            kind = kind;
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
        let name = self.instance.name_unchecked();
        let ns = self.instance.namespace().ok_or(Error::NoNamespace)?;
        match self.refresh(&mut keycloak).await {
            Ok(_) => {
                debug!(
                    kind = KeycloakInstance::kind(&()),
                    name = name,
                    namespace = ns;
                    "Token refreshed"
                )
            }
            Err(e) => {
                warn!(
                    kind = KeycloakInstance::kind(&()),
                    name = name,
                    namespace = ns;
                    "Error refreshing token: {}, trying to login",
                    e
                );
                self.keycloak_with_credentials().await?;
            }
        }
        Ok(())
    }
}

type RefreshStoreInner =
    HashMap<ObjectRef<KeycloakInstance>, (JoinHandle<()>, Arc<Notify>)>;

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
        resource: &Arc<KeycloakInstance>,
        client: kube::Client,
    ) -> Result<()> {
        let resource = resource.clone();
        let mut jobs = self.jobs.lock().await;
        self.cancel_refresh_locked(&mut jobs, &resource).await?;

        let kind = KeycloakInstance::kind(&());
        let session_handler = K8sKeycloakRefreshJob::new(resource, client);
        let instance = &session_handler.instance;
        let ns = instance.namespace().ok_or(Error::NoNamespace)?;
        let name = instance.name_unchecked();
        let instance_api = Api::<KeycloakInstance>::namespaced(
            session_handler.client.clone(),
            &ns,
        );
        let key = ObjectRef::from(instance.deref());

        let keycloak = session_handler.keycloak_from_somewhere().await?;

        if let Some(expires) = keycloak.token().expires {
            debug!(
                name = name,
                namespace = ns,
                kind = kind;
                "Scheduling token refresh"
            );
            let stopper = session_handler.stopper();
            let handle = tokio::spawn(async move {
                if let Err(e) = session_handler.run_expire(expires).await {
                    log::error!("Error in keycloak session handler: {}", e);
                    if let Err(e) = instance_api
                        .patch_status(
                            &name,
                            &PatchParams::apply(app_id!()),
                            &e.to_patch(),
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
        instance: &KeycloakInstance,
    ) -> Result<()> {
        let key = ObjectRef::from(instance);

        if let Some((handle, stopper)) = jobs.0.remove(&key) {
            stopper.notify_one();
            let _ = handle.await;
        }
        Ok(())
    }

    pub async fn cancel_refresh(
        &self,
        instance: &KeycloakInstance,
    ) -> Result<()> {
        let mut jobs = self.jobs.lock().await;
        self.cancel_refresh_locked(&mut jobs, instance).await
    }
}
