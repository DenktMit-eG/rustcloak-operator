<div align=center>
<img src=icon.svg>
</div>

# rustcloak-operator

**Manage Keycloak Realms in Kubernetes!** No more excuses for writing shellscipts! No more excuses for manual steps in the deployment!

Rustcloak is a keycloak-operator for kubernetes.
It is built in Rust using the [kube-rs](https://kube.rs/) library.
In contrast to other keycloak operators, Rustcloak aims to cover
the full API of Keycloak.


## Features

While Rustcloak is still unfinished, it already supports the following features:

* Lifecycle management of the following Resources:
  * [AuthenticationFlowRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlAuthenticationFlowRepresentation)
  * [AuthenticatorConfigRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlAuthenticatorConfigRepresentation)
  * [ClientRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlClientRepresentation)
  * [ClientScopeRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlClientScopeRepresentation)
  * [ComponentRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlComponentRepresentation)
  * [GroupRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlGroupRepresentation)
  * [IdentityProviderMapperRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlIdentityProviderMapperRepresentation)
  * [IdentityProviderRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlIdentityProviderRepresentation)
  * [OrganizationRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlOrganizationRepresentation)
  * [ProtocolMapperRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlProtocolMapperRepresentation)
  * [RealmRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlRealmRepresentation)
  * [RequiredActionProviderRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlRequiredActionProviderRepresentation)
  * [ResourceRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlResourceRepresentation)
  * [RoleRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlRoleRepresentation)
  * [ScopeRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlScopeRepresentation)
  * [UserRepresentation](https://www.keycloak.org/docs-api/latest/rest-api/index.htmlUserRepresentation)
* Role Management for Groups and Users using the [RoleMapping](https://rustcloak.withlazers.dev/crds/keycloakrolemapping.html) resource.
* Prometheus Metrics
* Secret Management for Keycloak Users and Clients.
* Rustcloak can manage Keycloak instances in a multi-tenant setup across namespaces
  using the [ClusterKeycloakRealm](https://rustcloak.withlazers.dev/crds/clusterkeycloakrealm.html)
  and [ClusterKeycloakInstance](https://rustcloak.withlazers.dev/crds/clusterkeycloakinstance.html)
  resources.
* Compatibility mode for the abandoned [keycloak-realm-operator](https://github.com/keycloak/keycloak-realm-operator).

## State

While Rustcloak is still in development and not feature complete

* [ ] make the RoleMapping resource use selectors instead of parent references
* [ ] introduce a GroupMapping resource that works similar to the RoleMapping resource
* [ ] Send Kubernetes Events
* [ ] Populate the .status field of the CRDs
    * [x] Set .status.ready state correctly
    * [x] Set .status.phase
    * [x] Set .status.message
    * [ ] Update state transitions in .status.conditions
    * [ ] Update phase in .status.phase
* [ ] Improve resiliency against reconcilation loops: 

# Comments on the License

The AGPL license can seem daunting at first, so here are some
clarifications on how we interpret it in Rustcloak:

* **CRD Manifests**: Custom Resource Definitions (CRD) manifests are
  configurations, not modifications of Rustcloak, and are therefore not
  considered derived work under the AGPL.

* **Using the Official Docker Image**: If you use the official
  Rustcloak Docker image without changes, you only need to provide a link
  to [the repository](https://github.com/withlazers/rustcloak-operator)
  to those who directly interact with Rustcloak itself. This does not
  include the end-users of applications managed by Rustcloak, but may apply
  if you provide rustcloak as a service to others.

* **Using Rustcloak in Your Own Docker Image**: If you include
  Rustcloak in a custom Docker image without modifying its code, the same
  rules apply as when using the official image.

* **Modifying Rustcloak’s Code**: If you make any changes to the
  Rustcloak binary, AGPL requirements mean you may need to share the
  modified source code with users who interact with your modified
  instance. To simplify compliance, we encourage contributing these
  changes back to the [upstream project](https://github.com/withlazers/rustcloak-operator).
