use crate::app_id;
use crate::error::{Error, Result};
use crate::morph::{Morph, Patcher};
use crate::{
    controller::LifecycleController,
    util::{ParentGetter, ParentRetrieve, Representation, Retrieve, ToPatch},
};
use k8s_openapi::serde_json::json;
use kube::ResourceExt;
use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::{
    Api,
    runtime::{Controller, controller::Action, watcher},
};
use log::debug;
use rustcloak_crd::{
    ClusterKeycloakApiObject, KeycloakApiObject, KeycloakApiObjectSpec,
    inner_spec::HasInnerSpec,
    refs::{HasParent, Ref},
    traits::Endpoint,
};
use rustcloak_crd::{
    KeycloakApiEndpoint, KeycloakApiEndpointPath, KeycloakRestObject,
};
use std::{fmt::Debug, marker::PhantomData, sync::Arc};

#[derive(Debug)]
pub struct RepresentationController<R> {
    phantom: PhantomData<R>,
}

impl<R> Default for RepresentationController<R> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<R> LifecycleController for RepresentationController<R>
where
    R: Representation + Endpoint,
    ParentGetter<R>: ParentRetrieve<R>,
    <<<R as HasInnerSpec>::InnerSpec as HasParent>::ParentRef as Ref>::Target:
        Endpoint,
{
    type Resource = R;
    const MODULE_PATH: &'static str = module_path!();

    fn prepare(
        &self,
        controller: Controller<R>,
        client: &kube::Client,
    ) -> Controller<R> {
        controller
            .owns(
                Api::<ClusterKeycloakApiObject>::all(client.clone()),
                watcher::Config::default(),
            )
            .owns(
                Api::<KeycloakApiObject>::all(client.clone()),
                watcher::Config::default(),
            )
    }

    async fn before_finalizer(
        &self,
        _client: &kube::Client,
        _resource: Arc<R>,
    ) -> Result<bool> {
        Ok(true)
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let ns = resource.meta().namespace.clone();
        let admin_api: Api<KeycloakApiObject> = Api::all(client.clone());
        let kind = Self::Resource::kind(&());
        let api_name =
            format!("{}-{}",
            <<R as HasInnerSpec>::InnerSpec as KeycloakRestObject>::API_PREFIX,
            resource.name_unchecked());
        let owner_ref = resource.owner_ref(&()).unwrap();

        let primary_key =
            <<R as HasInnerSpec>::InnerSpec as KeycloakRestObject>::ID_FIELD;
        let mut payload = resource.inner_spec().payload()?;
        payload
            .as_object_mut()
            .as_mut()
            .unwrap()
            .remove(primary_key);
        let immutable_payload =
            if let Some(primary_key_value) = resource.inner_spec().id() {
                serde_yaml::to_string(&json!({
                    primary_key: primary_key_value,
                }))?
            } else {
                "{}".to_string()
            }
            .into();
        let mut patcher = Patcher::new(payload);
        for (path, patch) in resource
            .inner_spec()
            .patches()
            .map(|x| x.patch_from.iter())
            .unwrap_or_default()
        {
            patcher.patch(path, patch)?;
        }
        let vars = patcher.vars;
        let payload = serde_yaml::to_string(&patcher.value)?;

        let parent_ref = resource.inner_spec().parent_ref();
        let parent =
            ParentGetter::<R>::get(client.clone(), parent_ref, &ns).await?;
        //let parent = resource
        //    .inner_spec()
        //    .parent_ref()
        //    .parent(client.clone())
        //    .await?;
        let instance_ref = if let Some(instance_ref) = resource.instance_ref() {
            instance_ref
        } else {
            parent.instance_ref().ok_or(Error::NoData)?
        };
        let resource_path = format!(
            "{}/{}",
            parent.resource_path().ok_or(Error::NoData)?,
            resource.inner_spec().mount_path()
        );

        let endpoint = KeycloakApiEndpoint {
            instance_ref: instance_ref.clone(),
            path_def: KeycloakApiEndpointPath::Path(resource_path.into()),
        };
        debug!(
            kind = kind, name = resource.name_unchecked(), namesapce = ns;
            "Resolved endpoint: {:?}",
            endpoint
        );

        let api_object = KeycloakApiObject {
            metadata: ObjectMeta {
                name: Some(api_name.to_string()),
                namespace: ns.clone(),
                owner_references: Some(vec![owner_ref]),
                labels: Some(
                    [(
                        app_id!("instanceRef").to_string(),
                        endpoint.instance_ref.clone().into(),
                    )]
                    .into(),
                ),
                ..Default::default()
            },
            spec: KeycloakApiObjectSpec {
                endpoint,
                options: resource.inner_spec().options().cloned(),
                immutable_payload,
                payload,
                vars,
            },
            status: None,
        };

        admin_api
            .patch(
                &api_name,
                &PatchParams::apply(app_id!()),
                &Patch::Apply(api_object),
            )
            .await?;

        if let Some(api_status) =
            admin_api.get_status(&api_name).await?.status.clone()
        {
            let api = Api::<Self::Resource>::all(client.clone());
            api.patch_status(
                &resource.name_unchecked(),
                &PatchParams::apply(app_id!()),
                &api_status.to_patch(),
            )
            .await?;
        }
        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        _client: &kube::Client,
        _resource: Arc<R>,
    ) -> Result<Action> {
        Ok(Action::await_change())
    }
}
