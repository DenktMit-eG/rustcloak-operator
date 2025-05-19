use crate::{
    app_id,
    controller::LifecycleController,
    error::{Error, Result},
    util::{
        ApiExt, ApiFactory, K8sKeycloakBuilder, Retrieve, Retriever, ToPatch,
    },
};
use async_trait::async_trait;
use either::{Either, for_both};
use keycloak_client::ApiClient;
use kube::{
    ResourceExt,
    api::PatchParams,
    runtime::{Controller, controller::Action},
};
use log::{debug, warn};
use rustcloak_crd::{
    KeycloakApiStatus, KeycloakApiStatusEndpoint, KeycloakRoleMapping,
    KeycloakRoleRef, RoleMappingParentRef, RoleNameOrRef,
    client::ClientRef,
    group::KeycloakGroup,
    instance::InstanceRef,
    keycloak_types::{ClientRepresentation, RoleRepresentation},
    realm::RealmRef,
    role::RoleRef,
    traits::Endpoint,
    user::KeycloakUser,
};
use std::sync::Arc;

type Subject = Either<KeycloakUser, KeycloakGroup>;
enum SubPath {
    Realm,
    Client(Box<ClientRepresentation>),
}

impl SubPath {
    async fn from_role_name(
        keycloak_role_ref: &KeycloakRoleRef,
        realm_ref: &RealmRef,
        keycloak: &ApiClient,
        client: &kube::Client,
        ns: &Option<String>,
    ) -> Result<(String, Self)> {
        let name = keycloak_role_ref.name.clone();
        let Some(client_ref) = &keycloak_role_ref.client_ref else {
            return Ok((name, SubPath::Realm));
        };

        let sub_path = match client_ref {
            rustcloak_crd::ClientNameOrRef::ClientRef(client_ref) => {
                SubPath::from_client_ref(client_ref, keycloak, client, ns)
                    .await?
            }
            rustcloak_crd::ClientNameOrRef::KeycloakClient { client_id } => {
                let realm =
                    Retriever::<RealmRef>::get(client.clone(), realm_ref, ns)
                        .await?;
                let path =
                    realm.resource_path().ok_or(Error::MissingResourcePath)?;
                let path = format!(
                    "{path}/clients?clientId={client_id}",
                    client_id = urlencoding::encode(client_id)
                );
                let mut client: Vec<ClientRepresentation> =
                    keycloak.get(&path).await?;
                let representation = client.pop().ok_or_else(|| {
                    Error::NoSuchKeycloakClient(client_id.clone())
                })?;

                SubPath::Client(Box::new(representation))
            }
        };

        Ok((name, sub_path))
    }

    async fn from_role_ref(
        role_ref: &RoleRef,
        keycloak: &ApiClient,
        client: &kube::Client,
        ns: &Option<String>,
    ) -> Result<(String, Self)> {
        let role =
            Retriever::<RoleRef>::get(client.clone(), role_ref, ns).await?;
        let sub_path = if let Either::Right(client_ref) = &*role.spec.parent_ref
        {
            SubPath::from_client_ref(client_ref, keycloak, client, ns).await?
        } else {
            SubPath::Realm
        };

        let name = role
            .spec
            .definition
            .and_then(|x| x.name)
            .ok_or(Error::MissingName)?;
        Ok((name, sub_path))
    }

    async fn from_client_ref(
        client_ref: &ClientRef,
        keycloak: &ApiClient,
        client: &kube::Client,
        ns: &Option<String>,
    ) -> Result<SubPath> {
        let resource =
            Retriever::<ClientRef>::get(client.clone(), client_ref, ns).await?;
        let representation: ClientRepresentation = keycloak
            .get(resource.resource_path().ok_or(Error::MissingResourcePath)?)
            .await?;
        Ok(SubPath::Client(Box::new(representation)))
    }

    fn into_path(self, subject: &Subject) -> Result<String> {
        let resource_path =
            subject.resource_path().ok_or(Error::MissingResourcePath)?;
        if let Self::Client(representation) = self {
            let client_id = representation.id.ok_or(Error::MissingClientId)?;
            Ok(format!("{resource_path}/role-mappings/clients/{client_id}"))
        } else {
            Ok(format!("{resource_path}/role-mappings/realm"))
        }
    }
}

#[derive(Default)]
pub struct RoleMappingController {}

impl RoleMappingController {
    async fn init(
        &self,
        client: &kube::Client,
        resource: &KeycloakRoleMapping,
    ) -> Result<(InstanceRef, ApiClient, Option<String>, Subject)> {
        let ns = resource.namespace();
        let subject_ref = &resource.spec.subject;
        let subject = Retriever::<RoleMappingParentRef>::get(
            client.clone(),
            subject_ref,
            &ns,
        )
        .await?;

        let instance_ref =
            if let Some(instance_ref) = resource.instance_ref() {
                instance_ref
            } else {
                subject
                    .instance_ref()
                    .ok_or(Error::MissingInstanceReference)?
            }
            .clone();

        let instance =
            Retriever::<InstanceRef>::get(client.clone(), &instance_ref, &ns)
                .await?;
        let keycloak = for_both!(&instance, instance => K8sKeycloakBuilder::new(instance, client))
            .with_token()
            .await?;

        Ok((instance_ref, keycloak, ns, subject))
    }

    async fn get_role_name_and_path(
        &self,
        resource: &KeycloakRoleMapping,
        subject: &Subject,
        keycloak: &ApiClient,
        client: &kube::Client,
        ns: &Option<String>,
    ) -> Result<(String, String)> {
        let realm_ref = subject.realm_ref().ok_or(Error::MissingRealmRef)?;

        let (role_name, sub_path) = match &resource.spec.role {
            RoleNameOrRef::RoleRef(role_ref) => {
                SubPath::from_role_ref(role_ref, keycloak, client, ns).await?
            }
            RoleNameOrRef::KeycloakRole { role } => {
                SubPath::from_role_name(role, &realm_ref, keycloak, client, ns)
                    .await?
            }
        };

        let path = sub_path.into_path(subject)?;

        Ok((role_name, path))
    }

    async fn role(
        &self,
        resource_path: &str,
        role_name: &str,
        keycloak: &ApiClient,
    ) -> Result<Option<RoleRepresentation>> {
        debug!("Getting roles from {resource_path}");
        let roles = keycloak
            .get::<Vec<RoleRepresentation>>(resource_path)
            .await?;

        Ok(roles
            .into_iter()
            .find(|role| role.name.as_deref() == Some(role_name)))
    }

    async fn available_role(
        &self,
        resource_path: &str,
        role_name: &str,
        keycloak: &ApiClient,
    ) -> Result<Option<RoleRepresentation>> {
        let resource_path = format!("{resource_path}/available");
        self.role(&resource_path, role_name, keycloak).await
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
        let (instance_ref, keycloak, ns, subject) =
            self.init(client, &resource).await?;

        let (name, resource_path) = self
            .get_role_name_and_path(&resource, &subject, &keycloak, client, &ns)
            .await?;

        let realm_ref = subject.realm_ref().ok_or(Error::MissingRealmRef)?;

        if self.role(&resource_path, &name, &keycloak).await?.is_none() {
            debug!("Apply role {name}");
            let Some(role_representation) = self
                .available_role(&resource_path, &name, &keycloak)
                .await?
            else {
                return Err(Error::NoSuchRoleOnKeycloak(name));
            };

            if role_representation.id.is_none() {
                return Err(Error::MissingField("id".to_string()));
            }
            if role_representation.name.is_none() {
                return Err(Error::MissingField("name".to_string()));
            }

            keycloak.post(&resource_path, [role_representation]).await?;
        } else {
            debug!("Role {name} already applied");
        }

        let mut status = KeycloakApiStatus::ok("Applied");
        status.endpoint = Some(KeycloakApiStatusEndpoint {
            realm: Some(realm_ref),
            resource_path,
            instance: instance_ref,
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
        let Some(endpoint) = resource.endpoint() else {
            return Ok(Action::await_change());
        };

        let (_, keycloak, ns, subject) = self.init(client, &resource).await?;

        let (name, _) = self
            .get_role_name_and_path(&resource, &subject, &keycloak, client, &ns)
            .await?;

        let Some(role) =
            self.role(&endpoint.resource_path, &name, &keycloak).await?
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
