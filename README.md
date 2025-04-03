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

# Example

Create a connection to a Keycloak instance
```yaml
apiVersion: rustcloak.k8s.eboland.de/v1
kind: KeycloakInstance
metadata:
  name: keycloak-instance
spec:
  baseUrl: http://keycloak-keycloakx-http:80/auth
  credentials:
    # Rustcloak will take care of creating this secret with a random password.
    create: true
    passwordKey: KEYCLOAK_ADMIN_PASSWORD
    secretName: keycloak-admin
    usernameKey: KEYCLOAK_ADMIN
```

Create a realm
```yaml
apiVersion: rustcloak.k8s.eboland.de/v1beta1
kind: KeycloakRealm
metadata:
  name: example-keycloakrealm
spec:
  instanceRef: keycloak-instance
  definition:
    realm: an-example-realm
    displayName: "Hello World"
```

Create a client
```yaml
apiVersion: rustcloak.k8s.eboland.de/v1beta1
kind: KeycloakClient
metadata:
  name: example-keycloakclient
spec:
  realmRef: example-keycloakrealm
  definition:
    clientId: example-client
    name: An example client
  clientSecret:
    secretName: example-keycloakclient
```

Note that this client will yield a secret that stores the client id and the client 
id in a Kubernetes secret.

## Status

Rustcloak supports all simple CRUD endpoints of the Keycloak API as of Keycloak-26.
There are several other endpoints of Keycloak that do not follow a basic CRUD theme
though. While we're striving to support all of them, we are not there yet.

The integration with kubernetes needs to be improved as well. While rustcloak
is en par with other operators, we are strive to improve on the 
[Operator Capabilities Levels](https://sdk.operatorframework.io/docs/overview/operator-capabilities/).

# Comments on the License

The AGPL license can seem daunting at first, so here are some
clarifications on how we interpret it in Rustcloak:

* **CRD Manifests**: Custom Resource Definitions (CRD) are managed in 
  a dedicated crate licensed under BSD-2-clause.

* **Using the Official Docker Image**: If you use the official
  Rustcloak Docker image without changes, you only need to provide a link
  to [the repository](https://github.com/DenktMit-eG/rustcloak-operator)
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
  changes back to the [upstream project](https://github.com/DenktMit-eG/rustcloak-operator).
