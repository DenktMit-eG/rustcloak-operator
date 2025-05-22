use super::{find_name, should_handle_prudent};
use crate::{
    app_id,
    controller::controller_runner::LifecycleController,
    error::Result,
    util::{ApiExt, ApiFactory},
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
use rustcloak_crd::{
    instance::{InstanceRef, KeycloakInstance},
    realm::{KeycloakRealm, KeycloakRealmSpec},
};
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
        let ns = resource.namespace();
        let owner_ref = resource.owner_ref(&()).unwrap();
        let api = ApiExt::<KeycloakRealm>::api(client.clone(), &ns);
        let definition = serde_json::to_value(&resource.spec.realm)?;
        let parent_ref = InstanceRef::from(
            find_name::<KeycloakInstance>(
                client,
                &ns,
                &resource.spec.instance_selector,
                &resource.metadata,
                "instance_ref",
            )
            .await?
            .map_either(|l| l.into(), |r| r.into()),
        );

        let instance = KeycloakRealm {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: ns.clone(),
                owner_references: Some(vec![owner_ref]),
                labels: resource.meta().labels.clone(),
                annotations: resource.meta().annotations.clone(),
                ..Default::default()
            },
            spec: KeycloakRealmSpec {
                parent_ref,
                options: None,
                definition: serde_json::from_value(definition)?,
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
