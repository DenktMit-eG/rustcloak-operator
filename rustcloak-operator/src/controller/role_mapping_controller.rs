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
use log::warn;
use rustcloak_crd::{
    ClientRef, InstanceRef, KeycloakApiStatus, KeycloakApiStatusEndpoint,
    KeycloakRoleMapping, KeycloakRoleRef, RoleMappingParentRef, RoleNameOrRef,
    RoleRef,
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
            Retriever::<InstanceRef>::get(client.clone(), instance_ref, ns)
                .await?;
        for_both!(&instance, instance => K8sKeycloakBuilder::new(instance, client))
            .with_token()
            .await
    }

    async fn mapping_path(
        &self,
        resource: &KeycloakRoleMapping,
        keycloak: &ApiClient,
        client: &kube::Client,
        ns: &Option<String>,
    ) -> Result<String> {
        let client_ref = match &resource.spec.role {
            RoleNameOrRef::RoleRef(role_ref) => {
                let role =
                    Retriever::<RoleRef>::get(client.clone(), role_ref, ns)
                        .await?;
                role.spec.parent_ref.inner.right()
            }
            RoleNameOrRef::KeycloakRole {
                keycloak_role: KeycloakRoleRef { client_ref, .. },
            } => client_ref.clone(),
        };
        let Some(client_ref) = client_ref else {
            return Ok("role-mappings/realm".to_string());
        };

        let client_resource =
            Retriever::<ClientRef>::get(client.clone(), &client_ref, ns)
                .await?;
        let client_path = client_resource
            .resource_path()
            .ok_or(Error::MissingResourcePath)?;
        let client_id = keycloak
            .get::<UserRepresentation>(client_path)
            .await?
            .id
            .ok_or(Error::MissingField("id".to_string()))?;
        Ok(format!("role-mappings/clients/{client_id}"))
    }

    async fn role_from_available(
        &self,
        resource_path: &str,
        resource: &KeycloakRoleMapping,
        keycloak: &ApiClient,
        client: &kube::Client,
        ns: &Option<String>,
    ) -> Result<Option<RoleRepresentation>> {
        self.role_from(
            "/available",
            resource_path,
            resource,
            keycloak,
            client,
            ns,
        )
        .await
    }

    async fn role_from_applied(
        &self,
        resource_path: &str,
        resource: &KeycloakRoleMapping,
        keycloak: &ApiClient,
        client: &kube::Client,
        ns: &Option<String>,
    ) -> Result<Option<RoleRepresentation>> {
        self.role_from("", resource_path, resource, keycloak, client, ns)
            .await
    }

    async fn role_from(
        &self,
        endpoint: &str,
        resource_path: &str,
        resource: &KeycloakRoleMapping,
        keycloak: &ApiClient,
        client: &kube::Client,
        ns: &Option<String>,
    ) -> Result<Option<RoleRepresentation>> {
        match &resource.spec.role {
            RoleNameOrRef::RoleRef(role_ref) => {
                let role =
                    Retriever::<RoleRef>::get(client.clone(), role_ref, ns)
                        .await?;
                let path =
                    role.resource_path().ok_or(Error::MissingResourcePath)?;
                Ok(Some(keycloak.get::<RoleRepresentation>(path).await?))
            }
            RoleNameOrRef::KeycloakRole {
                keycloak_role: KeycloakRoleRef { name, .. },
            } => {
                let available_path = format!("{resource_path}{endpoint}");
                let available_roles = keycloak
                    .get::<Vec<RoleRepresentation>>(&available_path)
                    .await?;

                Ok(available_roles
                    .into_iter()
                    .find(|role| role.name.as_ref() == Some(name)))
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
        let subject_ref = &resource.spec.subject;
        let subject = Retriever::<RoleMappingParentRef>::get(
            client.clone(),
            subject_ref,
            &ns,
        )
        .await?;

        let instance_ref = if let Some(instance_ref) = resource.instance_ref() {
            instance_ref
        } else {
            subject
                .instance_ref()
                .ok_or(Error::MissingInstanceReference)?
        };

        let keycloak = self.keycloak(instance_ref, client, &ns).await?;

        let subject_path =
            subject.resource_path().ok_or(Error::MissingResourcePath)?;
        let sub_path =
            self.mapping_path(&resource, &keycloak, client, &ns).await?;
        let resource_path = format!("{subject_path}/{sub_path}");

        let Some(role_representation) = self
            .role_from_available(
                &resource_path,
                &resource,
                &keycloak,
                client,
                &ns,
            )
            .await?
        else {
            warn!("Role not found in available roles. skipping");
            return Ok(Action::await_change());
        };

        if role_representation.id.is_none() {
            return Err(Error::MissingField("id".to_string()));
        }
        if role_representation.name.is_none() {
            return Err(Error::MissingField("name".to_string()));
        }

        keycloak.post(&resource_path, [role_representation]).await?;

        let mut status = KeycloakApiStatus::ok("Applied");
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
        let Some(role) = self
            .role_from_applied(
                &endpoint.resource_path,
                &resource,
                &keycloak,
                client,
                &ns,
            )
            .await?
        else {
            warn!(
                "Role not found in applied roles. Assuming it's already removed"
            );
            return Ok(Action::await_change());
        };

        keycloak
            .delete_with_payload(&endpoint.resource_path, [role])
            .await?;
        Ok(Action::await_change())
    }
}
