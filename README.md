# rustcloak-operator

![Architecture](./arch.svg)

## TODO:

### Endpoints

With a bit of shell magic, I extracted all the endpoints of keycloak:

```
curl -L https://www.keycloak.org/docs-api/latest/rest-api/openapi.yaml -o keycloak_openapi.yaml
yq '.paths | to_entries | (.[] |= [.value.put.requestBody.content."application/json".schema.$ref, .key]) | filter(.0 and .1 == "*}") | group_by(.0) | .[] | ["* [ ] `" + (.0.0 | sub(".*/","")) + "`", "  * [ ] `" + .[].1 + "`"] | join("\n") + "\n"' keycloak_openapi.yaml
```

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

* [x] `IdentityProviderMapperRepresentation`
  * [ ] `/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`

* [x] `IdentityProviderRepresentation`
  * [x] `/admin/realms/{realm}/identity-provider/instances/{alias}`

* [x] `OrganizationRepresentation`
  * [x] `/admin/realms/{realm}/organizations/{id}`

* [x] `ProtocolMapperRepresentation`
  * [ ] `/admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models/{id}`
  * [ ] `/admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models/{id}`
  * [ ] `/admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/{id}`

* [x] `RealmRepresentation`
  * [x] `/admin/realms/{realm}`

* [x] `RequiredActionProviderRepresentation`
  * [ ] `/admin/realms/{realm}/authentication/required-actions/{alias}`

* [x] `ResourceRepresentation`
  * [ ] `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}`

* [x] `RoleRepresentation`
  * [ ] `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
  * [x] (Won't do) `/admin/realms/{realm}/roles-by-id/{role-id}`
  * [ ] `/admin/realms/{realm}/roles/{role-name}`

* [x] `ScopeRepresentation`
  * [ ] `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/{scope-id}`

* [x] `UserRepresentation`
  * [x] `/admin/realms/{realm}/users/{user-id}`

It seems that we have 14 different CRDs to implement.
