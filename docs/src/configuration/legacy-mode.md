# Legacy Mode

Rustcloak supports a mode called "legacy mode" in which it can control
the CRDs of the [keycloak-realms-operator][1].

Please note that Rustcloak cannot run alongside the
keycloak-realm-operator. It is a good idea to remove all objects
managed by the old controller before starting Rustcloak and redeploying
it for rustcloak to take over.

To enable legacy mode, enabled the `legacy: <mode>` option in the rustcloak
helm chart.

Rustcloak supports two legacy mode modes it can operate in:

- `all` mode: Rustcloak will manage all legacy keycloak objects
- `prudent` mode: Rustcloak will manage only the keycloak objects that have the
  `rustcloak.k8s.eboland.de/handle: "true"` annotation.

With `disabled` mode, rustcloak will not manage any legacy keycloak objects. This is the default.

rustcloak will manage the following legacy keycloak objects:

- [legacy-`ExternalKeycloak`][2]: a shim for [KeycloakInstance](../crds/keycloakinstance.md)
- [legacy-`KeycloakRealm`][5]: a shim for [KeycloakRealm](../crds/keycloakrealm.md)
- [legacy-`KeycloakClient`][3]: a shim for [KeycloakClient](../crds/keycloakclient.md)
- [legacy-`KeycloakUser`][4]: a shim for [KeycloakUser](../crds/keycloakuser.md)

<div class=warning>
rustcloak will not roll out legacy CRDs. You need to create them manually.
</div>

## Managing legacy objects

Rustcloak can handle whole trees of legacy objects. The discovery of parent objects is
done by selectors, using the [legacy-`ExternalKeycloak`][2] resource as a root object.

But most of the time you actually want to use the rustcloak resources instead and just
use the legacy mode while migrating larger setups to Rustcloak. For that, rustcloak
allows you to overwrite define the `rustcloak.k8s.eboland.de/*Ref` annotations on the legacy objects
that point to the rustcloak parent resources.

Example:

Given that you have a [legacy-`KeycloakRealm`][5] resource in rustcloak:

```yaml
apiVersion: rustcloak.k8s.eboland.de/v1
kind: KeycloakRealm
metadata:
  name: example-keycloakrealm
spec:
  instanceRef: keycloak-instance
  definition:
    realm: an-example-realm
    displayName: An Example Realm
```

You can now create a [legacy-`KeycloakClient`][3] resource:

```yaml
apiVersion: legacy.k8s.keycloak.org/v1alpha1
kind: KeycloakClient
metadata:
  annotations:
    rustcloak.k8s.eboland.de/realmRef: example-keycloakrealm
  name: example-keycloakclient
spec:
  client:
    clientId: example-client
    name: example-client
  realmSelector:
    matchLabels: {}
```

rustloak will now manage the [legacy-`KeycloakClient`][3] resource as if it was a child of
the [`KeycloakRealm`](../crds/keycloakrealm.md) resource. Note that if you run rustcloak in `prudent` mode,
you need to add the `rustcloak.k8s.eboland.de/handle: "true"` annotation to
the [legacy-`KeycloakClient`][3] resource.

If the `realmRef` annotation is not set, rustcloak will try to discover the parent object using the `realmSelector` field.
This only works if the parent object is a legacy resource.

The following annotations are supported:

- for [legacy-`KeycloakRealm`][5]:
  - `rustcloak.k8s.eboland.de/instanceRef`: The name of the [`KeycloakInstance`](../crds/keycloakinstance.md) resources
  - `rustcloak.k8s.eboland.de/handle`: If set to `true`, rustcloak will manage the object in `prudent` mode
- for [legacy-`KeycloakClient`][3]:
  - `rustcloak.k8s.eboland.de/realmRef`: The name of the [`KeycloakRealm`](../crds/keycloakrealm.md) resources
  - `rustcloak.k8s.eboland.de/handle`: If set to `true`, rustcloak will manage the object in `prudent` mode
- for [legacy-`KeycloakUser`][4]:
  - `rustcloak.k8s.eboland.de/realmRef`: The name of the [`KeycloakRealm`](../crds/keycloakrealm.md) resources
  - `rustcloak.k8s.eboland.de/handle`: If set to `true`, rustcloak will manage the object in `prudent` mode
- for [legacy-`ExternalKeycloak`][2]:
  - `rustcloak.k8s.eboland.de/handle`: If set to `true`, rustcloak will manage the object in `prudent` mode

[1]: https://github.com/keycloak/keycloak-realm-operator
[2]: https://github.com/keycloak/keycloak-realm-operator/blob/main/deploy/examples/external-keycloak.yaml
[3]: https://github.com/keycloak/keycloak-realm-operator/blob/main/deploy/examples/example-client.yaml
[4]: https://github.com/keycloak/keycloak-realm-operator/blob/main/deploy/examples/example-user.yaml
[5]: https://github.com/keycloak/keycloak-realm-operator/blob/main/deploy/examples/realm-legacy/example-realm.yaml
