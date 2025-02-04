use super::super::controller_runner::LifecycleController;
use super::{find_name, should_handle_prudent};
use crate::app_id;
use crate::crd::KeycloakRealmSpec;
use crate::error::Error;
use crate::{crd::KeycloakRealm, error::Result};
use async_trait::async_trait;
use keycloak_crd::{
    ExternalKeycloak as LegacyInstance, KeycloakRealm as LegacyRealm,
};
use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::runtime::watcher;
use kube::{
    runtime::{controller::Action, Controller},
    Api,
};
use kube::{Resource, ResourceExt};
use std::sync::Arc;

#[derive(Debug)]
pub struct LegacyRealmController {
    prudent: bool,
}

impl LegacyRealmController {
    pub fn new(prudent: bool) -> Self {
        Self { prudent }
    }
}

#[async_trait]
impl LifecycleController for LegacyRealmController {
    type Resource = LegacyRealm;
    const MODULE_PATH: &'static str = module_path!();

    async fn before_finalizer(
        &self,
        _client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<bool> {
        Ok(should_handle_prudent(resource.meta(), self.prudent))
    }

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource> {
        controller.owns(
            Api::<KeycloakRealm>::all(client.clone()),
            watcher::Config::default(),
        )
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let name = resource.name_unchecked();
        let namespace = resource.namespace().ok_or(Error::NoNamespace)?;
        let owner_ref = resource.owner_ref(&()).unwrap();
        let api = Api::<KeycloakRealm>::namespaced(client.clone(), &namespace);
        let definition = serde_json::to_value(&resource.spec.realm)?;
        let instance = KeycloakRealm {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: Some(namespace.clone()),
                owner_references: Some(vec![owner_ref]),
                ..Default::default()
            },
            spec: KeycloakRealmSpec {
                options: None,
                instance_ref: find_name::<LegacyInstance>(
                    client,
                    &namespace,
                    &resource.spec.instance_selector,
                    &resource.metadata,
                    "instance_ref",
                )
                .await?
                .into(),
                definition: serde_json::from_value(definition)?,
                patches: None,
            },
            status: None,
        };
        api.patch(
            &name,
            &PatchParams::apply(app_id!()),
            &Patch::Apply(instance),
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
