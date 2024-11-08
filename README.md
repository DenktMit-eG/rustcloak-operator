<div align=center>
<img src=icon.svg>
</div>

# rustcloak-operator

This is yet another operator for Keycloak. The novelty of this one is
that it covers the whole Keycloak API and therefore allows to manage
keycloak instances completely as Kubernetes resources.

Currently rustcloak does not support subgroups.

rustcloak supports a legacy mode where it can be used as a drop-in
replacement for the
[keycloak-realm-operator](https://github.com/keycloak/keycloak-realm-operator).

![Architecture](./arch.svg)

## TODO:

* [ ] Support Subgroups
* [ ] Send Kubernetes Events
* [ ] Update state transitions in .status.conditions
* [ ] Update phase in .status.phase
* [ ] Add prometheus metrics

## Endpoints

Table of implemented endpoints:

* [x] `AuthenticationFlowRepresentation`
  * [x] `/admin/realms/{realm}/authentication/flows/{id}`

* [x] `AuthenticatorConfigRepresentation`
  * [x] `/admin/realms/{realm}/authentication/config/{id}`

* [x] `ClientRepresentation`
  * [x] `/admin/realms/{realm}/clients/{client-uuid}`

* [x] `ClientScopeRepresentation`
  * [x] `/admin/realms/{realm}/client-scopes/{client-scope-id}`
  * [x] `/admin/realms/{realm}/client-templates/{client-scope-id}`

* [x] `ComponentRepresentation`
  * [x] `/admin/realms/{realm}/components/{id}`

* [x] `GroupRepresentation`
  * [x] `/admin/realms/{realm}/groups/{group-id}`
  * [ ] `/admin/realms/{realm}/groups/{group-id}/children`

* [x] `IdentityProviderMapperRepresentation`
  * [x] `/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`

* [x] `IdentityProviderRepresentation`
  * [x] `/admin/realms/{realm}/identity-provider/instances/{alias}`

* [x] `OrganizationRepresentation`
  * [x] `/admin/realms/{realm}/organizations/{id}`

* [x] `ProtocolMapperRepresentation`
  * [x] `/admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models/{id}`
  * [x] `/admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models/{id}`
  * [x] `/admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/{id}`

* [x] `RealmRepresentation`
  * [x] `/admin/realms/{realm}`

* [x] `RequiredActionProviderRepresentation`
  * [x] `/admin/realms/{realm}/authentication/required-actions/{alias}`

* [x] `ResourceRepresentation`
  * [x] `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}`

* [x] `RoleRepresentation`
  * [x] `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
  * [x] `/admin/realms/{realm}/roles/{role-name}`

* [x] `ScopeRepresentation`
  * [x] `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/{scope-id}`

* [x] `UserRepresentation`
  * [x] `/admin/realms/{realm}/users/{user-id}`

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
