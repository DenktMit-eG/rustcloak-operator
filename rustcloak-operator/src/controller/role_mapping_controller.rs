use crate::{
    app_id,
    controller::LifecycleController,
    error::{Error, Result},
    util::{
        ApiExt, ApiFactory, K8sKeycloakBuilder, Retrieve, Retriever, ToPatch,
    },
};
use async_trait::async_trait;
use either::for_both;
use keycloak_client::ApiClient;
use kube::{
    ResourceExt,
    api::PatchParams,
    runtime::{Controller, controller::Action},
};
use rustcloak_crd::{
    ClientRef, InstanceRef, KeycloakApiStatusEndpoint, KeycloakRoleMapping,
    RoleMappingParentRef, RoleNameOrRef, RoleRef,
    keycloak_types::{RoleRepresentation, UserRepresentation},
    traits::Endpoint,
};
use std::sync::Arc;

#[derive(Default)]
pub struct RoleMappingController {}

impl RoleMappingController {
    async fn keycloak(
        &self,
        instance_ref: &InstanceRef,
        client: &kube::Client,
        ns: &Option<String>,
    ) -> Result<ApiClient> {
        let instance =
            Retriever::<InstanceRef>::get(client.clone(), instance_ref, &ns)
                .await?;
        Ok(for_both!(&instance, instance => K8sKeycloakBuilder::new(instance, client))
            .with_token()
            .await?)
    }
    async fn mapping_path(
        &self,
        parent_path: &str,
        resource: &KeycloakRoleMapping,
        keycloak: &ApiClient,
        client: &kube::Client,
        ns: &Option<String>,
    ) -> Result<String> {
        let Some(client_ref) = &resource.spec.client_ref else {
            return Ok(format!("{parent_path}/role-mappings/realm"));
        };
        let client =
            Retriever::<ClientRef>::get(client.clone(), client_ref, &ns)
                .await?;
        let client_path =
            client.resource_path().ok_or(Error::MissingResourcePath)?;
        // TODO: correct error code
        let client_id = keycloak
            .get::<UserRepresentation>(&client_path)
            .await?
            .id
            .ok_or(Error::MissingField("id".to_string()))?;
        Ok(format!("{parent_path}/role-mappings/clients/{client_id}"))
    }

    async fn role_representation(
        &self,
        resource_path: &str,
        resource: &KeycloakRoleMapping,
        keycloak: &ApiClient,
        client: &kube::Client,
        ns: &Option<String>,
    ) -> Result<RoleRepresentation> {
        match &resource.spec.role_ref {
            RoleNameOrRef::Ref(role_ref) => {
                let role =
                    Retriever::<RoleRef>::get(client.clone(), role_ref, &ns)
                        .await?;
                let path =
                    role.resource_path().ok_or(Error::MissingResourcePath)?;
                Ok(keycloak.get::<RoleRepresentation>(&path).await?)
            }
            RoleNameOrRef::Name { role_name } => {
                let available_path = format!("{resource_path}/available");
                let available_roles = keycloak
                    .get::<Vec<RoleRepresentation>>(&available_path)
                    .await?;

                Ok(available_roles
                    .into_iter()
                    .find(|role| role.name.as_ref() == Some(role_name))
                    .ok_or(Error::MissingField(role_name.to_string()))?)
            }
        }
    }
}

#[async_trait]
impl LifecycleController for RoleMappingController {
    type Resource = KeycloakRoleMapping;
    const MODULE_PATH: &'static str = module_path!();

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        _client: &kube::Client,
    ) -> Controller<Self::Resource> {
        controller
    }

    async fn before_finalizer(
        &self,
        _client: &kube::Client,
        _resource: Arc<Self::Resource>,
    ) -> Result<bool> {
        Ok(true)
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let ns = resource.namespace();
        let parent_ref = &resource.spec.parent_ref;
        let parent = Retriever::<RoleMappingParentRef>::get(
            client.clone(),
            &parent_ref,
            &ns,
        )
        .await?;

        let instance_ref = if let Some(instance_ref) = resource.instance_ref() {
            instance_ref
        } else {
            parent
                .instance_ref()
                .ok_or(Error::MissingInstanceReference)?
        };

        let keycloak = self.keycloak(instance_ref, client, &ns).await?;

        let parent_path =
            parent.resource_path().ok_or(Error::MissingResourcePath)?;
        let resource_path = self
            .mapping_path(&parent_path, &resource, &keycloak, client, &ns)
            .await?;

        let role_representation = self
            .role_representation(
                &resource_path,
                &resource,
                &keycloak,
                client,
                &ns,
            )
            .await?;

        if role_representation.id.is_none() {
            return Err(Error::MissingField("id".to_string()));
        }
        if role_representation.name.is_none() {
            return Err(Error::MissingField("name".to_string()));
        }

        keycloak
            .post_location(&resource_path, role_representation)
            .await?;

        let mut status = resource.status.clone().unwrap_or_default();
        status.endpoint = Some(KeycloakApiStatusEndpoint {
            resource_path,
            instance: instance_ref.clone(),
        });
        status.ready = true;

        let api = ApiExt::<KeycloakRoleMapping>::api(client.clone(), &ns);
        api.patch_status(
            &resource.name_unchecked(),
            &PatchParams::apply(app_id!()),
            &status.to_patch(),
        )
        .await?;

        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let ns = resource.namespace();
        let Some(endpoint) = resource.endpoint() else {
            return Ok(Action::await_change());
        };

        let keycloak = self.keycloak(&endpoint.instance, client, &ns).await?;
        let role = self
            .role_representation(
                &endpoint.resource_path,
                &resource,
                &keycloak,
                client,
                &ns,
            )
            .await?;

        keycloak
            .delete_with_payload(&endpoint.resource_path, role)
            .await?;
        Ok(Action::await_change())
    }
}
