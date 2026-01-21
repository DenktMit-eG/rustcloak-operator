use anyhow::{Context, Result};
use either::Either;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use keycloak_client::{ApiAuth, ApiClient};
use kube::api::{Patch, PatchParams, PostParams};
use kube::core::object::HasStatus;
use kube::{Api, Client, Resource};
use rustcloak_crd::api_object::KeycloakApiObject;
use rustcloak_crd::authentication_flow::{KeycloakAuthenticationFlow, KeycloakAuthenticationFlowSpec};
use rustcloak_crd::client::{KeycloakClient as KeycloakClientCrd, KeycloakClientSpec};
use rustcloak_crd::client_scope::{KeycloakClientScope, KeycloakClientScopeSpec};
use rustcloak_crd::component::{KeycloakComponent, KeycloakComponentSpec};
use rustcloak_crd::filters::{
    is_default_auth_flow, is_default_client, is_default_client_scope, is_default_realm_role,
    is_excluded_realm,
};
use rustcloak_crd::group::{KeycloakGroup, KeycloakGroupSpec};
use rustcloak_crd::identity_provider::{KeycloakIdentityProvider, KeycloakIdentityProviderSpec};
use rustcloak_crd::instance::{
    ClusterInstanceRef, ClusterKeycloakInstance, ClusterKeycloakInstanceSpec, InstanceRef,
    KeycloakInstance, KeycloakInstanceCredentialReference, KeycloakInstanceSpec,
    NamespacedInstanceRef,
};
use rustcloak_crd::keycloak_types::{
    AuthenticationFlowRepresentation, ClientRepresentation, ClientScopeRepresentation,
    ComponentRepresentation, GroupRepresentation, IdentityProviderRepresentation, RealmRepresentation,
    RoleRepresentation, UserRepresentation,
};
use rustcloak_crd::naming::to_k8s_name;
use rustcloak_crd::realm::{ClusterRealmRef, KeycloakRealm, KeycloakRealmSpec, NamespacedRealmRef, RealmRef};
use rustcloak_crd::role::{KeycloakRole, KeycloakRoleSpec};
use rustcloak_crd::user::{KeycloakUser, KeycloakUserSpec, ParentRef as UserParentRef};
use rustcloak_crd::{KeycloakApiStatus, KeycloakApiStatusEndpoint};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use std::time::Duration;

use crate::auth::Credentials;
use crate::ImportArgs;

pub async fn run_import(args: ImportArgs) -> Result<()> {
    // Initialize K8s client (only if not dry-run or if we need credential secret)
    let k8s_client = if !args.dry_run || args.auth.credential_secret.is_some() {
        Some(Client::try_default().await.context("Failed to create K8s client")?)
    } else {
        None
    };

    // Resolve credentials
    let credentials = Credentials::resolve(&args.auth, &args.namespace, k8s_client.as_ref()).await?;

    // Authenticate with Keycloak
    let auth = ApiAuth::new(&args.keycloak_url);
    let keycloak = auth
        .login_with_credentials(&credentials.username, &credentials.password)
        .await
        .context("Failed to authenticate with Keycloak")?;

    // Build instance ref
    let instance_ref = if args.cluster_instance {
        InstanceRef {
            inner: Either::Right(ClusterInstanceRef::from(args.instance_name.clone())),
        }
    } else {
        InstanceRef {
            inner: Either::Left(NamespacedInstanceRef::from(args.instance_name.clone())),
        }
    };

    // Determine credential secret name
    let credential_secret_name = args
        .auth
        .credential_secret
        .clone()
        .unwrap_or_else(|| format!("{}-credentials", args.instance_name));

    // Create or print KeycloakInstance
    if args.cluster_instance {
        create_or_print_cluster_instance(
            &args.instance_name,
            &args.keycloak_url,
            &credential_secret_name,
            args.dry_run,
            k8s_client.as_ref(),
        )
        .await?;
    } else {
        create_or_print_instance(
            &args.instance_name,
            &args.namespace,
            &args.keycloak_url,
            &credential_secret_name,
            args.dry_run,
            k8s_client.as_ref(),
        )
        .await?;
    }

    // Fetch realms
    let realms: Vec<RealmRepresentation> = keycloak
        .get("admin/realms")
        .await
        .context("Failed to fetch realms")?;

    // Filter realms
    let realms: Vec<_> = realms
        .into_iter()
        .filter(|r| {
            let realm_name = r.realm.as_deref().unwrap_or("");
            // Filter by --realms flag if provided
            if !args.realms.is_empty() && !args.realms.iter().any(|x| x == realm_name) {
                return false;
            }
            // Filter excluded realms (like master) unless include_defaults
            if !args.include_defaults && is_excluded_realm(realm_name) {
                return false;
            }
            true
        })
        .collect();

    let importer = Importer {
        keycloak: &keycloak,
        k8s_client: k8s_client.as_ref(),
        namespace: &args.namespace,
        instance_ref: &instance_ref,
        include_defaults: args.include_defaults,
        dry_run: args.dry_run,
    };

    for realm in realms {
        importer.import_realm(realm).await?;
    }

    Ok(())
}

struct Importer<'a> {
    keycloak: &'a ApiClient,
    k8s_client: Option<&'a Client>,
    namespace: &'a str,
    instance_ref: &'a InstanceRef,
    include_defaults: bool,
    dry_run: bool,
}

impl<'a> Importer<'a> {
    async fn import_realm(&self, realm: RealmRepresentation) -> Result<()> {
        let realm_name = realm.realm.clone().unwrap_or_default();
        let k8s_name = to_k8s_name(&realm_name);

        eprintln!("Importing realm: {realm_name} -> {k8s_name}");

        // Create realm CRD
        let realm_crd = KeycloakRealm {
            metadata: ObjectMeta {
                name: Some(k8s_name.clone()),
                namespace: Some(self.namespace.to_string()),
                ..Default::default()
            },
            spec: KeycloakRealmSpec {
                options: None,
                parent_ref: self.instance_ref.clone(),
                definition: Some(realm),
            },
            status: None,
        };

        let resource_path = format!("admin/realms/{realm_name}");
        let realm_ref = self.build_realm_ref(&k8s_name);

        self.apply_resource::<KeycloakRealm>(realm_crd, &resource_path, None)
            .await?;

        // Import child resources
        self.import_clients(&realm_name, &k8s_name, &realm_ref)
            .await?;
        self.import_client_scopes(&realm_name, &k8s_name, &realm_ref)
            .await?;
        self.import_users(&realm_name, &k8s_name, &realm_ref)
            .await?;
        self.import_groups(&realm_name, &k8s_name, &realm_ref)
            .await?;
        self.import_roles(&realm_name, &k8s_name, &realm_ref)
            .await?;
        self.import_identity_providers(&realm_name, &k8s_name, &realm_ref)
            .await?;
        self.import_auth_flows(&realm_name, &k8s_name, &realm_ref)
            .await?;
        self.import_components(&realm_name, &k8s_name, &realm_ref)
            .await?;

        Ok(())
    }

    fn build_realm_ref(&self, realm_k8s_name: &str) -> RealmRef {
        if self.instance_ref.inner.is_right() {
            // Cluster instance -> cluster realm ref
            RealmRef {
                inner: Either::Right(ClusterRealmRef::from(realm_k8s_name.to_string())),
            }
        } else {
            RealmRef {
                inner: Either::Left(NamespacedRealmRef::from(realm_k8s_name.to_string())),
            }
        }
    }

    async fn import_clients(
        &self,
        realm_name: &str,
        realm_k8s_name: &str,
        realm_ref: &RealmRef,
    ) -> Result<()> {
        let clients: Vec<ClientRepresentation> = self
            .keycloak
            .get(&format!("admin/realms/{realm_name}/clients"))
            .await
            .context("Failed to fetch clients")?;

        for client in clients {
            let client_id = client.client_id.as_deref().unwrap_or("");

            // Filter default clients
            if !self.include_defaults && is_default_client(client_id) {
                continue;
            }

            let id = client.id.clone().unwrap_or_default();
            let k8s_name = format!("{realm_k8s_name}-{}", to_k8s_name(client_id));

            eprintln!("  Importing client: {client_id} -> {k8s_name}");

            let crd = KeycloakClientCrd {
                metadata: ObjectMeta {
                    name: Some(k8s_name.clone()),
                    namespace: Some(self.namespace.to_string()),
                    ..Default::default()
                },
                spec: KeycloakClientSpec {
                    options: None,
                    parent_ref: realm_ref.clone(),
                    definition: Some(client),
                    client_secret: None,
                },
                status: None,
            };

            let resource_path = format!("admin/realms/{realm_name}/clients/{id}");
            self.apply_resource::<KeycloakClientCrd>(crd, &resource_path, Some(realm_ref))
                .await?;
        }

        Ok(())
    }

    async fn import_client_scopes(
        &self,
        realm_name: &str,
        realm_k8s_name: &str,
        realm_ref: &RealmRef,
    ) -> Result<()> {
        let scopes: Vec<ClientScopeRepresentation> = self
            .keycloak
            .get(&format!("admin/realms/{realm_name}/client-scopes"))
            .await
            .context("Failed to fetch client scopes")?;

        for scope in scopes {
            let scope_name = scope.name.as_deref().unwrap_or("");

            // Filter default scopes
            if !self.include_defaults && is_default_client_scope(scope_name) {
                continue;
            }

            let id = scope.id.clone().unwrap_or_default();
            let k8s_name = format!("{realm_k8s_name}-{}", to_k8s_name(scope_name));

            eprintln!("  Importing client scope: {scope_name} -> {k8s_name}");

            let crd = KeycloakClientScope {
                metadata: ObjectMeta {
                    name: Some(k8s_name.clone()),
                    namespace: Some(self.namespace.to_string()),
                    ..Default::default()
                },
                spec: KeycloakClientScopeSpec {
                    options: None,
                    parent_ref: realm_ref.clone(),
                    is_template: Some(true),
                    definition: Some(scope),
                },
                status: None,
            };

            let resource_path = format!("admin/realms/{realm_name}/client-scopes/{id}");
            self.apply_resource::<KeycloakClientScope>(crd, &resource_path, Some(realm_ref))
                .await?;
        }

        Ok(())
    }

    async fn import_users(
        &self,
        realm_name: &str,
        realm_k8s_name: &str,
        realm_ref: &RealmRef,
    ) -> Result<()> {
        let users: Vec<UserRepresentation> = self
            .keycloak
            .get(&format!("admin/realms/{realm_name}/users"))
            .await
            .context("Failed to fetch users")?;

        for user in users {
            let username = user.username.as_deref().unwrap_or("");
            let id = user.id.clone().unwrap_or_default();
            let k8s_name = format!("{realm_k8s_name}-{}", to_k8s_name(username));

            eprintln!("  Importing user: {username} -> {k8s_name}");

            let crd = KeycloakUser {
                metadata: ObjectMeta {
                    name: Some(k8s_name.clone()),
                    namespace: Some(self.namespace.to_string()),
                    ..Default::default()
                },
                spec: KeycloakUserSpec {
                    options: None,
                    parent_ref: UserParentRef::with_realm(realm_ref.clone()),
                    definition: Some(user),
                    user_secret: None,
                },
                status: None,
            };

            let resource_path = format!("admin/realms/{realm_name}/users/{id}");
            self.apply_resource::<KeycloakUser>(crd, &resource_path, Some(realm_ref))
                .await?;
        }

        Ok(())
    }

    async fn import_groups(
        &self,
        realm_name: &str,
        realm_k8s_name: &str,
        realm_ref: &RealmRef,
    ) -> Result<()> {
        let groups: Vec<GroupRepresentation> = self
            .keycloak
            .get(&format!("admin/realms/{realm_name}/groups"))
            .await
            .context("Failed to fetch groups")?;

        for group in groups {
            let group_name = group.name.as_deref().unwrap_or("");
            let id = group.id.clone().unwrap_or_default();
            let k8s_name = format!("{realm_k8s_name}-{}", to_k8s_name(group_name));

            eprintln!("  Importing group: {group_name} -> {k8s_name}");

            let crd = KeycloakGroup {
                metadata: ObjectMeta {
                    name: Some(k8s_name.clone()),
                    namespace: Some(self.namespace.to_string()),
                    ..Default::default()
                },
                spec: KeycloakGroupSpec {
                    options: None,
                    parent_ref: rustcloak_crd::either::UntaggedEither {
                        inner: Either::Left(realm_ref.clone()),
                    },
                    definition: Some(group),
                },
                status: None,
            };

            let resource_path = format!("admin/realms/{realm_name}/groups/{id}");
            self.apply_resource::<KeycloakGroup>(crd, &resource_path, Some(realm_ref))
                .await?;
        }

        Ok(())
    }

    async fn import_roles(
        &self,
        realm_name: &str,
        realm_k8s_name: &str,
        realm_ref: &RealmRef,
    ) -> Result<()> {
        let roles: Vec<RoleRepresentation> = self
            .keycloak
            .get(&format!("admin/realms/{realm_name}/roles"))
            .await
            .context("Failed to fetch roles")?;

        for role in roles {
            let role_name = role.name.clone().unwrap_or_default();

            // Filter default roles
            if !self.include_defaults && is_default_realm_role(&role_name) {
                continue;
            }

            let k8s_name = format!("{realm_k8s_name}-{}", to_k8s_name(&role_name));

            eprintln!("  Importing role: {role_name} -> {k8s_name}");

            let crd = KeycloakRole {
                metadata: ObjectMeta {
                    name: Some(k8s_name.clone()),
                    namespace: Some(self.namespace.to_string()),
                    ..Default::default()
                },
                spec: KeycloakRoleSpec {
                    options: None,
                    parent_ref: rustcloak_crd::either::UntaggedEither {
                        inner: Either::Left(realm_ref.clone()),
                    },
                    definition: Some(role),
                },
                status: None,
            };

            // Roles use name as ID in API path
            let resource_path = format!("admin/realms/{realm_name}/roles/{role_name}");
            self.apply_resource::<KeycloakRole>(crd, &resource_path, Some(realm_ref))
                .await?;
        }

        Ok(())
    }

    async fn import_identity_providers(
        &self,
        realm_name: &str,
        realm_k8s_name: &str,
        realm_ref: &RealmRef,
    ) -> Result<()> {
        let idps: Vec<IdentityProviderRepresentation> = self
            .keycloak
            .get(&format!("admin/realms/{realm_name}/identity-provider/instances"))
            .await
            .context("Failed to fetch identity providers")?;

        for idp in idps {
            let alias = idp.alias.clone().unwrap_or_default();
            let k8s_name = format!("{realm_k8s_name}-{}", to_k8s_name(&alias));

            eprintln!("  Importing identity provider: {alias} -> {k8s_name}");

            let crd = KeycloakIdentityProvider {
                metadata: ObjectMeta {
                    name: Some(k8s_name.clone()),
                    namespace: Some(self.namespace.to_string()),
                    ..Default::default()
                },
                spec: KeycloakIdentityProviderSpec {
                    options: None,
                    parent_ref: realm_ref.clone(),
                    definition: Some(idp),
                },
                status: None,
            };

            let resource_path =
                format!("admin/realms/{realm_name}/identity-provider/instances/{alias}");
            self.apply_resource::<KeycloakIdentityProvider>(crd, &resource_path, Some(realm_ref))
                .await?;
        }

        Ok(())
    }

    async fn import_auth_flows(
        &self,
        realm_name: &str,
        realm_k8s_name: &str,
        realm_ref: &RealmRef,
    ) -> Result<()> {
        let flows: Vec<AuthenticationFlowRepresentation> = self
            .keycloak
            .get(&format!("admin/realms/{realm_name}/authentication/flows"))
            .await
            .context("Failed to fetch authentication flows")?;

        for flow in flows {
            let alias = flow.alias.as_deref().unwrap_or("");

            // Filter default flows
            if !self.include_defaults && is_default_auth_flow(alias) {
                continue;
            }

            // Skip built-in flows
            if flow.built_in == Some(true) {
                continue;
            }

            let id = flow.id.clone().unwrap_or_default();
            let k8s_name = format!("{realm_k8s_name}-{}", to_k8s_name(alias));

            eprintln!("  Importing auth flow: {alias} -> {k8s_name}");

            let crd = KeycloakAuthenticationFlow {
                metadata: ObjectMeta {
                    name: Some(k8s_name.clone()),
                    namespace: Some(self.namespace.to_string()),
                    ..Default::default()
                },
                spec: KeycloakAuthenticationFlowSpec {
                    options: None,
                    parent_ref: realm_ref.clone(),
                    definition: Some(flow),
                },
                status: None,
            };

            let resource_path = format!("admin/realms/{realm_name}/authentication/flows/{id}");
            self.apply_resource::<KeycloakAuthenticationFlow>(crd, &resource_path, Some(realm_ref))
                .await?;
        }

        Ok(())
    }

    async fn import_components(
        &self,
        realm_name: &str,
        realm_k8s_name: &str,
        realm_ref: &RealmRef,
    ) -> Result<()> {
        let components: Vec<ComponentRepresentation> = self
            .keycloak
            .get(&format!("admin/realms/{realm_name}/components"))
            .await
            .context("Failed to fetch components")?;

        for component in components {
            let component_name = component.name.as_deref().unwrap_or("");
            let id = component.id.clone().unwrap_or_default();
            // Include short ID suffix to avoid name collisions (components can have same name)
            let id_suffix = &id[..8.min(id.len())];
            let k8s_name = format!(
                "{realm_k8s_name}-{}-{}",
                to_k8s_name(component_name),
                id_suffix
            );

            eprintln!("  Importing component: {component_name} -> {k8s_name}");

            let crd = KeycloakComponent {
                metadata: ObjectMeta {
                    name: Some(k8s_name.clone()),
                    namespace: Some(self.namespace.to_string()),
                    ..Default::default()
                },
                spec: KeycloakComponentSpec {
                    options: None,
                    parent_ref: realm_ref.clone(),
                    definition: Some(component),
                },
                status: None,
            };

            let resource_path = format!("admin/realms/{realm_name}/components/{id}");
            self.apply_resource::<KeycloakComponent>(crd, &resource_path, Some(realm_ref))
                .await?;
        }

        Ok(())
    }

    async fn apply_resource<T>(
        &self,
        resource: T,
        resource_path: &str,
        realm_ref: Option<&RealmRef>,
    ) -> Result<()>
    where
        T: Resource<Scope = k8s_openapi::NamespaceResourceScope>
            + Clone
            + Serialize
            + DeserializeOwned
            + HasStatus<Status = KeycloakApiStatus>
            + std::fmt::Debug,
        <T as Resource>::DynamicType: Default,
    {
        let status = KeycloakApiStatus {
            endpoint: Some(KeycloakApiStatusEndpoint {
                resource_path: resource_path.to_string(),
                instance: self.instance_ref.clone(),
                realm: realm_ref.cloned(),
            }),
            ready: true,
            status: "Imported".to_string(),
            message: "Imported from existing Keycloak".to_string(),
            conditions: vec![],
            api_object_ref: None, // Will be set after we get it from the operator
        };

        if self.dry_run {
            // Output YAML with status embedded for dry-run
            let mut value = serde_json::to_value(&resource)?;
            value["status"] = serde_json::to_value(&status)?;
            let yaml = serde_yaml::to_string(&value)?;
            println!("---\n{yaml}");
        } else {
            let client = self.k8s_client.expect("K8s client required for apply");
            let api: Api<T> = Api::namespaced(client.clone(), self.namespace);

            let name = resource
                .meta()
                .name
                .as_ref()
                .expect("Resource must have name");

            // Step 1: Create resource
            api.create(&PostParams::default(), &resource)
                .await
                .with_context(|| format!("Failed to create resource {name}"))?;

            // Step 2: Wait for operator to set api_object_ref in status
            let api_object_name = loop {
                let res = api
                    .get_status(name)
                    .await
                    .with_context(|| format!("Failed to get status for {name}"))?;
                if let Some(ref res_status) = res.status() {
                    if let Some(ref api_ref) = res_status.api_object_ref {
                        break api_ref.clone();
                    }
                }
                tokio::time::sleep(Duration::from_millis(500)).await;
            };

            // Step 3: Patch status on the referenced KeycloakApiObject
            let api_object_api: Api<KeycloakApiObject> =
                Api::namespaced(client.clone(), self.namespace);
            let mut api_object_status = status.clone();
            api_object_status.api_object_ref = Some(api_object_name.clone());
            let patch = Patch::Merge(json!({ "status": api_object_status }));
            api_object_api
                .patch_status(&api_object_name, &PatchParams::apply("rustcloak-cli"), &patch)
                .await
                .with_context(|| {
                    format!("Failed to patch status for KeycloakApiObject {api_object_name}")
                })?;
        }

        Ok(())
    }
}

async fn create_or_print_instance(
    instance_name: &str,
    namespace: &str,
    keycloak_url: &str,
    credential_secret_name: &str,
    dry_run: bool,
    k8s_client: Option<&Client>,
) -> Result<()> {
    let instance = KeycloakInstance {
        metadata: ObjectMeta {
            name: Some(instance_name.to_string()),
            namespace: Some(namespace.to_string()),
            ..Default::default()
        },
        spec: KeycloakInstanceSpec {
            base_url: keycloak_url.to_string(),
            realm: None,
            credentials: KeycloakInstanceCredentialReference {
                create: Some(false),
                secret_name: credential_secret_name.to_string(),
                username_key: None,
                password_key: None,
            },
            token: None,
            client: None,
        },
        status: None,
    };

    let status = KeycloakApiStatus {
        endpoint: None,
        ready: true,
        status: "Imported".to_string(),
        message: "Imported from existing Keycloak".to_string(),
        conditions: vec![],
        api_object_ref: None,
    };

    if dry_run {
        eprintln!("Creating KeycloakInstance: {instance_name}");
        let mut value = serde_json::to_value(&instance)?;
        value["status"] = serde_json::to_value(&status)?;
        let yaml = serde_yaml::to_string(&value)?;
        println!("---\n{yaml}");
    } else {
        let client = k8s_client.expect("K8s client required for apply");
        let api: Api<KeycloakInstance> = Api::namespaced(client.clone(), namespace);

        // Check if instance already exists
        match api.get(instance_name).await {
            Ok(_) => {
                eprintln!("KeycloakInstance '{instance_name}' already exists, skipping creation");
            }
            Err(kube::Error::Api(e)) if e.code == 404 => {
                eprintln!("Creating KeycloakInstance: {instance_name}");
                api.create(&PostParams::default(), &instance)
                    .await
                    .with_context(|| {
                        format!("Failed to create KeycloakInstance '{instance_name}'")
                    })?;

                // Patch status
                let patch = Patch::Merge(json!({ "status": status }));
                api.patch_status(instance_name, &PatchParams::apply("rustcloak-cli"), &patch)
                    .await
                    .with_context(|| {
                        format!("Failed to patch status for KeycloakInstance '{instance_name}'")
                    })?;
            }
            Err(e) => return Err(e).context("Failed to check if KeycloakInstance exists"),
        }
    }

    Ok(())
}

async fn create_or_print_cluster_instance(
    instance_name: &str,
    keycloak_url: &str,
    credential_secret_name: &str,
    dry_run: bool,
    k8s_client: Option<&Client>,
) -> Result<()> {
    let instance = ClusterKeycloakInstance {
        metadata: ObjectMeta {
            name: Some(instance_name.to_string()),
            ..Default::default()
        },
        spec: ClusterKeycloakInstanceSpec {
            spec: KeycloakInstanceSpec {
                base_url: keycloak_url.to_string(),
                realm: None,
                credentials: KeycloakInstanceCredentialReference {
                    create: Some(false),
                    secret_name: credential_secret_name.to_string(),
                    username_key: None,
                    password_key: None,
                },
                token: None,
                client: None,
            },
        },
        status: None,
    };

    let status = KeycloakApiStatus {
        endpoint: None,
        ready: true,
        status: "Imported".to_string(),
        message: "Imported from existing Keycloak".to_string(),
        conditions: vec![],
        api_object_ref: None,
    };

    if dry_run {
        eprintln!("Creating ClusterKeycloakInstance: {instance_name}");
        let mut value = serde_json::to_value(&instance)?;
        value["status"] = serde_json::to_value(&status)?;
        let yaml = serde_yaml::to_string(&value)?;
        println!("---\n{yaml}");
    } else {
        let client = k8s_client.expect("K8s client required for apply");
        let api: Api<ClusterKeycloakInstance> = Api::all(client.clone());

        // Check if instance already exists
        match api.get(instance_name).await {
            Ok(_) => {
                eprintln!(
                    "ClusterKeycloakInstance '{instance_name}' already exists, skipping creation"
                );
            }
            Err(kube::Error::Api(e)) if e.code == 404 => {
                eprintln!("Creating ClusterKeycloakInstance: {instance_name}");
                api.create(&PostParams::default(), &instance)
                    .await
                    .with_context(|| {
                        format!("Failed to create ClusterKeycloakInstance '{instance_name}'")
                    })?;

                // Patch status
                let patch = Patch::Merge(json!({ "status": status }));
                api.patch_status(instance_name, &PatchParams::apply("rustcloak-cli"), &patch)
                    .await
                    .with_context(|| {
                        format!(
                            "Failed to patch status for ClusterKeycloakInstance '{instance_name}'"
                        )
                    })?;
            }
            Err(e) => return Err(e).context("Failed to check if ClusterKeycloakInstance exists"),
        }
    }

    Ok(())
}
