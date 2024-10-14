use std::sync::Arc;

use crate::util::JobHandler;
use crate::{
    crd::KeycloakAdminSession,
    error::{Error, Result},
};
use async_trait::async_trait;
use kube::{
    runtime::{controller::Action, Controller},
    Resource,
};
use tokio::sync::Mutex;

use super::controller_runner::LifetimeController;

#[derive(Debug, Default)]
pub struct KeycloakAdminSessionController {
    keepalive_jobs: Mutex<JobHandler<String>>,
    terminate_session_jobs: Mutex<JobHandler<String>>,
}

impl KeycloakAdminSessionController {
    async fn keepalive(
        client: kube::Client,
        resource: Arc<KeycloakAdminSession>,
    ) {
    }

    async fn terminate_session(
        client: kube::Client,
        resource: Arc<KeycloakAdminSession>,
    ) {
    }
}

#[async_trait]
impl LifetimeController for KeycloakAdminSessionController {
    type Resource = KeycloakAdminSession;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        _client: &kube::Client,
    ) -> Controller<Self::Resource> {
        controller
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let uid = resource.meta().uid.as_ref().ok_or(Error::NoUid)?;
        self.keepalive_jobs.lock().await.schedule(
            uid,
            resource.spec.next_keepalive,
            Self::keepalive(client.clone(), resource.clone()),
        )?;
        self.terminate_session_jobs.lock().await.schedule(
            uid,
            resource.spec.valid_until,
            Self::terminate_session(client.clone(), resource.clone()),
        )?;
        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        _client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let uid = resource.meta().uid.as_ref().ok_or(Error::NoUid)?;
        self.keepalive_jobs.lock().await.abort(uid);
        self.terminate_session_jobs.lock().await.abort(uid);
        Ok(Action::await_change())
    }
}
