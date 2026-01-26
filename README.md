<div align=center>
<img src=icon.svg>
</div>

# rustcloak-operator

This is yet another operator for Keycloak. The novelty of this one is
that it covers the whole Keycloak API and therefore allows to manage
keycloak instances completely as Kubernetes resources.

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

## Building

This project uses [Cargo](https://doc.rust-lang.org/cargo/) for builds.

### Using Make

A `Makefile` is provided for convenience:

```bash
make build        # Build in release mode
make check        # Run lint and test
make lint         # Run clippy
make test         # Run all tests
make fmt          # Check formatting
make fix-fmt      # Apply formatting
make build-docker # Build Docker image
```

### Using Cargo directly

```bash
cargo build --release
```

### Using Docker

```bash
docker build -t rustcloak-operator .
```

## Status

Rustcloak supports all simple CRUD endpoints of the Keycloak API as of Keycloak-26.
There are several other endpoints of Keycloak that do not follow a basic CRUD theme
though. While we're striving to support all of them, we are not there yet.

The integration with kubernetes needs to be improved as well. While rustcloak
is en par with other operators, we are strive to improve on the 
[Operator Capabilities Levels](https://sdk.operatorframework.io/docs/overview/operator-capabilities/).
