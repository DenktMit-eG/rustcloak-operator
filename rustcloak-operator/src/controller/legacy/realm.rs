use super::{find_name, should_handle_prudent};
use crate::{
    app_id,
    controller::controller_runner::LifecycleController,
    error::{Error, Result},
};
use async_trait::async_trait;
use k8s_openapi::serde_json;
use keycloak_crd::KeycloakRealm as LegacyRealm;
use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::runtime::watcher;
use kube::{
    Api,
    runtime::{Controller, controller::Action},
};
use kube::{Resource, ResourceExt};
use rustcloak_crd::{KeycloakInstance, KeycloakRealm, KeycloakRealmSpec};
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
                labels: resource.meta().labels.clone(),
                annotations: resource.meta().annotations.clone(),
                ..Default::default()
            },
            spec: KeycloakRealmSpec {
                options: None,
                parent_ref: find_name::<KeycloakInstance>(
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
