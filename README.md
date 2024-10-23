# rustcloak-operator

![Architecture](./arch.svg)

## TODO:

### Sessions

Currently rustcloak does not handle sessions well

* [x] The operator must schedule session keepalives. The timing of this
  keepalives should be queried from the keycloak realm settings. The next
  session keepalive should be saved in the instance status object.
* [x] Similiar to the keepalives, the operator must schedule renewals of the session.
* [x] Currently to retrieve a session token, we make use of The
     [keycloak crate](https://crates.io/crates/keycloak), which doesn't support
     a proper oauth flow. We should switch to the
     [oauth2 crate](https://crates.io/crates/oauth2) instead, which supports a
     proper oauth flow with refresh tokens.

Currently we have a session controller and an instance controller. Both controllers
do similiar things, maybe it's a good idea to have a single controller for both

### Endpoints

With a bit of shell magic, I extracted all the endpoints of keycloak:

```
yq '.paths | to_entries | .[] | select(.value.put.requestBody.content."application/json" and .key == "*}") | {.key: .value.put.requestBody.content."application/json".schema.$ref}' keycloak_openapi.yaml | sort -k 2 | yq . | uniq -f 1 --group
```

* [ ] `#/components/schemas/AuthenticationFlowRepresentation`
  * [ ] `/admin/realms/{realm}/authentication/flows/{id}`

* [ ] `#/components/schemas/AuthenticatorConfigRepresentation`
  * [ ] `/admin/realms/{realm}/authentication/config/{id}`

* [x] `#/components/schemas/ClientRepresentation`
  * [x] `/admin/realms/{realm}/clients/{client-uuid}`

* [ ] `#/components/schemas/ClientScopeRepresentation`
  * [ ] `/admin/realms/{realm}/client-scopes/{client-scope-id}`
  * [ ] `/admin/realms/{realm}/client-templates/{client-scope-id}`

* [ ] `#/components/schemas/ComponentRepresentation`
  * [ ] `/admin/realms/{realm}/components/{id}`

* [ ] `#/components/schemas/GroupRepresentation`
  * [ ] `/admin/realms/{realm}/groups/{group-id}`

* [ ] `#/components/schemas/IdentityProviderMapperRepresentation`
  * [ ] `/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`

* [ ] `#/components/schemas/IdentityProviderRepresentation`
  * [ ] `/admin/realms/{realm}/identity-provider/instances/{alias}`

* [ ] `#/components/schemas/ProtocolMapperRepresentation`
  * [ ] `/admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models/{id}`
  * [ ] `/admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models/{id}`
  * [ ] `/admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/{id}`

* [x] `#/components/schemas/RealmRepresentation`
  * [x] `/admin/realms/{realm}`

* [ ] `#/components/schemas/RequiredActionProviderRepresentation`
  * [ ] `/admin/realms/{realm}/authentication/required-actions/{alias}`

* [ ] `#/components/schemas/ResourceRepresentation`
  * [ ] `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}`

* [ ] `#/components/schemas/RoleRepresentation`
  * [ ] `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
  * [x] `/admin/realms/{realm}/roles-by-id/{role-id}`: Won't implement this one, as it's not needed
  * [ ] `/admin/realms/{realm}/roles/{role-name}`

* [ ] `#/components/schemas/ScopeRepresentation`
  * [ ] `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/{scope-id}`

* [x] `#/components/schemas/UserRepresentation`
  * [x] `/admin/realms/{realm}/users/{user-id}`

It seems that we have 14 different CRDs to implement.
