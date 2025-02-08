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

## Managing legacy objects

Rustcloak can handle whole trees of legacy objects. The discovery of parent objects is
done by selectors, using the [`ExternalKeycloak`][2] resource as a root object.

But most of the time you actually want to use the rustcloak resources instead and just
use the legacy mode while migrating larger setups to Rustcloak. For that, rustcloak
allows you to overwrite define the `rustcloak.k8s.eboland.de/*Ref` annotations on the legacy objects
that point to the rustcloak parent resources.

Example:

Given that you have a `KeycloakRealm` resource in rustcloak:

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

You can now create a legacy `KeycloakLient` resource:

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

rustloak will now manage the `KeycloakClient` resource as if it was a child of
the `KeycloakRealm` resource. Note that if you run rustcloak in `prudent` mode,
you need to add the `rustcloak.k8s.eboland.de/handle: "true"` annotation to
the `KeycloakClient` resource.

The following annotations are supported:

- for legacy `KeycloakRealm`:
  - `rustcloak.k8s.eboland.de/instanceRef`: The name of the `KeycloakInstance` resources
- for legacy `KeycloakClient`:
  - `rustcloak.k8s.eboland.de/realmRef`: The name of the `KeycloakRealm` resources
- for legacy `KeycloakUser`:
  - `rustcloak.k8s.eboland.de/realmRef`: The name of the `KeycloakRealm` resources

[1]: https://github.com/keycloak/keycloak-realm-operator
[2]: https://github.com/keycloak/keycloak-realm-operator/blob/main/deploy/examples/external-keycloak.yaml
