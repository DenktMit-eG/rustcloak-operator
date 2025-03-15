use crate::app_id;
use crate::error::{Error, Result};
use crate::{
    controller::LifecycleController,
    util::{
        ApiExt, ApiFactory, ParentGetter, ParentRetrieve, Representation,
        Retrieve, ToPatch,
    },
};
use k8s_openapi::serde_json::{self, json};
use k8s_openapi::{ClusterResourceScope, NamespaceResourceScope};
use kube::{
    Api, Resource, ResourceExt,
    api::{ObjectMeta, Patch, PatchParams},
    core::object::HasStatus,
    runtime::{Controller, controller::Action, watcher},
};
use log::debug;
use rustcloak_crd::{
    ClusterKeycloakApiObject, KeycloakApiEndpoint, KeycloakApiEndpointPath,
    KeycloakApiStatus, KeycloakRestObject,
};
use rustcloak_crd::{
    KeycloakApiObject, KeycloakApiObjectSpec,
    inner_spec::HasInnerSpec,
    refs::{HasParent, Ref},
    traits::Endpoint,
};
use serde::Serialize;
use serde::de::DeserializeOwned;
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

pub trait ApiObjectHelper {
    type ApiObject: Resource<DynamicType = ()>
        + Clone
        + Debug
        + DeserializeOwned
        + Serialize
        + HasStatus<Status = KeycloakApiStatus>
        + Send
        + Sync
        + 'static;
    fn create(meta: ObjectMeta, spec: KeycloakApiObjectSpec)
    -> Self::ApiObject;
    fn api(client: kube::Client, ns: &Option<String>) -> Api<Self::ApiObject>;
}

macro_rules! impl_api_object_helper {
    ($scope:ident, $type:ident) => {
        impl ApiObjectHelper for $scope {
            type ApiObject = $type;

            fn create(
                meta: ObjectMeta,
                spec: KeycloakApiObjectSpec,
            ) -> Self::ApiObject {
                Self::ApiObject {
                    metadata: meta,
                    spec: spec.into(),
                    status: None,
                }
            }

            fn api(
                client: kube::Client,
                ns: &Option<String>,
            ) -> Api<Self::ApiObject> {
                ApiExt::<Self::ApiObject>::api(client, ns)
            }
        }
    };
}
impl_api_object_helper!(ClusterResourceScope, ClusterKeycloakApiObject);
impl_api_object_helper!(NamespaceResourceScope, KeycloakApiObject);

#[async_trait::async_trait]
impl<R> LifecycleController for RepresentationController<R>
where
    R: Representation + Endpoint,
    ParentGetter<R>: ParentRetrieve<R>,
    <<<R as HasInnerSpec>::InnerSpec as HasParent>::ParentRef as Ref>::Target:
        Endpoint,
    R::Scope: ApiObjectHelper,
    ApiExt<R>: ApiFactory,
{
    type Resource = R;
    const MODULE_PATH: &'static str = module_path!();

    fn prepare(
        &self,
        controller: Controller<R>,
        client: &kube::Client,
    ) -> Controller<R> {
        controller.owns(
            Api::<<R::Scope as ApiObjectHelper>::ApiObject>::all(
                client.clone(),
            ),
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
        let ns = resource.namespace();
        let admin_api = R::Scope::api(client.clone(), &ns);
        let kind = Self::Resource::kind(&());
        let api_name =
            format!("{}-{}",
            <<R as HasInnerSpec>::InnerSpec as KeycloakRestObject>::API_PREFIX,
            resource.name_unchecked());
        let owner_ref = resource.owner_ref(&()).unwrap();

        let primary_key =
            <<R as HasInnerSpec>::InnerSpec as KeycloakRestObject>::ID_FIELD;
        let mut payload =
            if let Some(payload) = resource.inner_spec().definition() {
                serde_json::to_value(payload)?
            } else {
                serde_json::Value::Object(Default::default())
            };

        let id = payload
            .as_object_mut()
            .as_mut()
            .unwrap()
            .remove(primary_key);
        let immutable_payload = if let Some(id) = id {
            serde_yaml::to_string(&json!({
                primary_key: id,
            }))?
        } else {
            "{}".to_string()
        }
        .into();

        let payload = serde_yaml::to_string(&payload)?;

        let parent_ref = resource.inner_spec().parent_ref();
        let parent =
            ParentGetter::<R>::get(client.clone(), parent_ref, &ns).await?;
        let instance_ref = if let Some(instance_ref) = resource.instance_ref() {
            instance_ref
        } else {
            parent
                .instance_ref()
                .ok_or(Error::MissingInstanceReference)?
        };
        let init_workflow = resource.inner_spec().init_workflow();
        let resource_path = format!(
            "{}/{}",
            parent.resource_path().ok_or(Error::MissingResourcePath)?,
            init_workflow.mount_path
        );

        let endpoint = KeycloakApiEndpoint {
            instance_ref: instance_ref.clone(),
            init_workflow: Some(init_workflow.workflow),
            path_def: KeycloakApiEndpointPath::Path(resource_path.into()),
        };
        debug!(
            kind = kind, name = resource.name_unchecked(), namesapce = ns;
            "Resolved endpoint: {:?}",
            endpoint
        );

        let api_object = R::Scope::create(
            ObjectMeta {
                name: Some(api_name.to_string()),
                namespace: ns.clone(),
                owner_references: Some(vec![owner_ref]),
                labels: Some(
                    [(
                        app_id!("instanceRef").to_string(),
                        AsRef::<str>::as_ref(&endpoint.instance_ref.inner)
                            .to_string(),
                    )]
                    .into(),
                ),
                ..Default::default()
            },
            KeycloakApiObjectSpec {
                endpoint,
                options: resource.inner_spec().options().cloned(),
                immutable_payload,
                payload,
            },
        );

        admin_api
            .patch(
                &api_name,
                &PatchParams::apply(app_id!()),
                &Patch::Apply(api_object),
            )
            .await?;

        if let Some(api_status) =
            admin_api.get_status(&api_name).await?.status()
        {
            let api = ApiExt::<Self::Resource>::api(client.clone(), &ns);
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
