# Patches

In order to load arbitrary values from secrets and configmaps, you can use the `patchFrom` field of most `Keycloak*` resources.

Take this example:
you have the following definition:

```yaml
apiVersion: rustcloak.k8s.eboland.de/v1
kind: KeycloakRealm
metadata:
  name: example-keycloakrealm
spec:
  instanceRef: keycloak-instance
  definition:
    realm: an-example-realm
    identityProviders:
      - alias: example-identity-provider
        providerId: example-provider
        enabled: true
        config:
          secret: "secret"
```

Instead of storing the secret in the definition, you can store it in a secret and reference it like this:

```yaml
apiVersion: rustcloak.k8s.eboland.de/v1
kind: KeycloakRealm
metadata:
  name: example-keycloakrealm
spec:
  instanceRef: keycloak-instance
  definition:
    realm: an-example-realm
    identityProviders:
      - alias: example-identity-provider
        providerId: example-provider
        enabled: true
        config:
          secret: "" # must have a dummy value, must be not null
  patchFrom:
    "identityProviders.0.config.secret":
      secretKeyRef:
        name: my-secret
        key: IDENTITY_PROVIDER_SECRET
        valueAs: auto # optional
```

By default the value is autodetected. You can change this behavior by explicitely setting the
`valueAs` field. The following values are supported:

- `auto`: Autodetect the value type. This is the default.
- `string`: The secret value is interpreted as a string
- `number`: The secret value is interpreted as a number
- `yaml`: the value is interpreted as a YAML object (default for auto detected objects)
- `json`: the value is interpreted as a JSON object

## Note:

For managing passwords of [`KeycloakUsers`][1] and client credentials of [`KeycloakClients`][2] there are dedicated resources available. Please refer to the documentation of the respective resources for more information.

[1]: ../crds/keycloakuser.md
[2]: ../crds/keycloakclient.md
