# Quick Start

## Pre-requisites

* A running Kubernetes cluster
* `kubectl` installed and configured to use the cluster
* `helm` installed

## Install Rustcloak

```bash
helm repo add withlazers https://charts.withlazers.dev
helm install rustcloak withlazers/rustcloak-operator
```

## Install Keycloak

Prepare a `keycloak-values.yaml` file with the following content:

```yaml
command:
  - "/opt/keycloak/bin/kc.sh"
  - "start-dev"
  - "--http-port=8080"
  - "--hostname-strict=false"
extraEnv: |
  - name: JAVA_OPTS_APPEND
    value: >-
      -Djgroups.dns.query={{ include "keycloak.fullname" . }}-headless
extraEnvFrom: |
  - secretRef:
      name: keycloak-admin
```

Then install Keycloak:

```bash
helm repo add codecentric https://codecentric.github.io/helm-charts:
helm install keycloak codecentric/keycloakx -f keycloak-values.yaml
```

If you see an "CreateContainerConfigError" error, this is expected. Keycloak is waiting for its admin credentials.


## Tell Rustcloak about Keycloak

With Keycloak running, you can now create a `KeycloakInstance` resource linking Rustcloak to Keycloak:

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

Now keycloak should start up and Rustcloak should be able to connect to it. You can extract the login credentials from the secret:

```bash
echo "$(kubectl get secrets keycloak-admin --template={{.data.KEYCLOAK_ADMIN_PASSWORD}} | base64 -d)"
```

The user name is `rustcloak-admin`

## Create a Realm

With the `KeycloakInstance` resource in place, you can now create a `KeycloakRealm` resource:

```yaml
apiVersion: rustcloak.k8s.eboland.de/v1
kind: KeycloakRealm
metadata:
  name: example-keycloakrealm
spec:
  instanceRef: keycloak-instance
  definition:
    realm: an-example-realm
```

## Create a Client

With the `KeycloakRealm` resource in place, you can now create a `KeycloakClient` resource:

```yaml
apiVersion: rustcloak.k8s.eboland.de/v1
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

Rustcloak will retrieve the client secret from Keycloak and store it in a secret named `example-keycloakclient`:

```yaml
# kubectl get secret  example-keycloakclient -o yaml
apiVersion: v1
data:
  client_id: ZXhhbXBsZS1jbGllbnQ=
  client_secret: enNWVmhCYjlqSWtvbjJHdTY5TXZ1R1IwYlFsVTMxbGE=
kind: Secret
metadata:
  creationTimestamp: "2024-11-09T19:20:39Z"
  name: example-keycloakclient
  namespace: default
  ownerReferences:
  - apiVersion: rustcloak.k8s.eboland.de/v1
    kind: KeycloakClient
    name: example-keycloakclient
    uid: d6529acf-5410-4090-ad01-767c68cbf426
  resourceVersion: "1097"
  uid: 16c9244e-e2fd-4dc2-8175-13f5f8dd54c3
type: Opaque
```

## Create a User

With the `KeycloakRealm` resource in place, you can now create a `KeycloakUser` resource:

```yaml
apiVersion: rustcloak.k8s.eboland.de/v1
kind: KeycloakUser
metadata:
  name: example-keycloakuser
spec:
  realmRef: example-keycloakrealm
  definition:
    username: awesome-user
    email: mail@example.com
    enabled: true
    firstName: Awesome
    lastName: User
  userSecret:
    secretName: example-keycloakuser
```

Rustcloak will create a user in Keycloak and set the password from the secret named `example-keycloakuser`. If it doesn't exist, it will create it with a random password:

```yaml
# kubectl get secret  example-keycloakuser -o yaml
apiVersion: v1
data:
  password: ZXMmPGk6TGpyKX0vOG0hdi95W15MPz83WVxPP3MpKig=
  username: YXdlc29tZS11c2Vy
kind: Secret
metadata:
  creationTimestamp: "2024-11-09T19:21:14Z"
  name: example-keycloakuser
  namespace: default
  ownerReferences:
  - apiVersion: rustcloak.k8s.eboland.de/v1
    kind: KeycloakUser
    name: example-keycloakuser
    uid: aeee2245-b790-48d0-b9f2-79af86cc1753
  resourceVersion: "1135"
  uid: e1d607fa-8dc6-4db9-bf6b-764d18fc0a57
type: Opaque
````
