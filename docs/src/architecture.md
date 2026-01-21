# Architecture

![Architecture](./architecture/arch.svg)

## Overview

The rustcloak-operator is a Kubernetes operator that bridges Kubernetes Custom
Resource Definitions (CRDs) with the Keycloak Admin REST API. It enables
declarative management of Keycloak resources using GitOps workflows, allowing
you to define Keycloak configurations as Kubernetes manifests.

The operator watches for changes to rustcloak CRDs and reconciles them with the
corresponding Keycloak server, ensuring the desired state defined in Kubernetes
is reflected in Keycloak.

## Components

### KeycloakInstance / ClusterKeycloakInstance

The root resource that defines a connection to a Keycloak server. It contains:
- The base URL of the Keycloak server
- Authentication credentials (admin username/password)
- Optional token caching configuration

**KeycloakInstance** is namespace-scoped, while **ClusterKeycloakInstance** is
cluster-scoped and can be referenced by resources in any namespace.

### KeycloakRealm / ClusterKeycloakRealm

Defines a Keycloak realm and references a parent KeycloakInstance. Realms are
the top-level organizational unit in Keycloak containing users, clients, roles,
and other configuration.

### Child Resources

All other resources (clients, users, groups, etc.) are children of a realm:

| Resource                       | Description                                    |
| ------------------------------ | ---------------------------------------------- |
| **KeycloakClient**             | OAuth2/OIDC clients                            |
| **KeycloakUser**               | User accounts                                  |
| **KeycloakGroup**              | User groups                                    |
| **KeycloakRole**               | Realm and client roles                         |
| **KeycloakClientScope**        | Client scopes for token claims                 |
| **KeycloakIdentityProvider**   | External identity providers (SAML, OIDC, etc.) |
| **KeycloakProtocolMapper**     | Protocol mappers for token customization       |
| **KeycloakAuthenticationFlow** | Custom authentication flows                    |
| **KeycloakComponent**          | Server components (user federation, etc.)      |
| **KeycloakOrganization**       | Organizations (Keycloak 24+)                   |

### Utility Resources

| Resource                     | Description                                    | Purpose     |
| ---------------------------- | ---------------------------------------------- | ----------- |
| **KeycloakRoleMapping**      | Assign roles to users/groups                   | user facing |
| **KeycloakApiObject**        | Generic resource for unsupported API endpoints | internal    |
| **KeycloakClientCredential** | Sync existing client credentials to a Secret   | internal    |
| **KeycloakUserCredential**   | Sync existing user credentials to a Secret     | internal    |

## Data Flow

```
+---------------------------------------------------------------------+
|                         Kubernetes Cluster                          |
|                                                                     |
|  +-------------+    +--------------------+    +-----------------+   |
|  |   CRD       |---►| rustcloak-operator |---►| Keycloak Server |   |
|  | (desired    |    |                    |    |                 |   |
|  |  state)     |◄---|  - Watch CRDs      |◄---| (actual state)  |   |
|  +-------------+    |  - Reconcile       |    +-----------------+   |
|         |           |  - Update status   |                          |
|         ▼           +--------------------+                          |
|  +-------------+                                                    |
|  |   Status    |                                                    |
|  | (endpoint,  |                                                    |
|  |  ready)     |                                                    |
|  +-------------+                                                    |
+---------------------------------------------------------------------+
```

### Reconciliation Process

1. **Watch**: The operator watches for create, update, and delete events on
rustcloak CRDs
2. **Validate**: Parent resources must be in a ready state before child
resources can be reconciled
3. **Diff**: The operator compares the desired state (CRD spec) with the actual
state in Keycloak
4. **Apply**: Changes are applied to Keycloak via the Admin REST API
5. **Status Update**: The CRD status is updated with the resource endpoint and
ready state

## Resource Hierarchy

Resources form a tree structure with parent-child relationships:

```
KeycloakInstance (or ClusterKeycloakInstance)
+-- KeycloakRealm (or ClusterKeycloakRealm)
    +-- KeycloakClient
    |   +-- KeycloakProtocolMapper
    |   +-- KeycloakRole (client role)
    |   +-- KeycloakResource (authorization)
    |   +-- KeycloakScope (authorization)
    |   +-- KeycloakUser (service account)
    +-- KeycloakUser
    +-- KeycloakGroup
    +-- KeycloakRole (realm role)
    +-- KeycloakClientScope
    |   +-- KeycloakProtocolMapper
    +-- KeycloakIdentityProvider
    |   +-- KeycloakIdentityProviderMapper
    +-- KeycloakAuthenticationFlow
    |   +-- KeycloakAuthenticatorConfig
    +-- KeycloakComponent
    +-- KeycloakOrganization
    +-- KeycloakRequiredActionProvider
```

A child resource cannot be reconciled until its parent is ready. This ensures
resources are created in the correct order.

## Secret Management

The operator integrates with Kubernetes Secrets for credential management:

### Instance Credentials

```yaml
apiVersion: rustcloak.k8s.withlazers.dev/v1
kind: KeycloakInstance
spec:
  credentials:
    secretName: keycloak-admin-credentials
    usernameKey: user      # default: "user"
    passwordKey: password  # default: "password"
    create: true           # auto-create Secret with random password
```

### Client Secret Synchronization

When a KeycloakClient is created with `clientSecret` specified, the operator
will:
1. Retrieve the generated client secret from Keycloak
2. Store it in the specified Kubernetes Secret

```yaml
apiVersion: rustcloak.k8s.withlazers.dev/v1
kind: KeycloakClient
spec:
  clientSecret:
    secretName: my-client-credentials
    clientIdKey: my_client_id          # default: "client_id"
    clientSecretKey: my_client_secret  # default: "client_secret"
```

### User Credential Synchronization

Similar to clients, user credentials can be synchronized to Secrets:

```yaml
apiVersion: rustcloak.k8s.withlazers.dev/v1
kind: KeycloakUser
spec:
  userSecret:
    secretName: my-user-credentials
    usernameKey: username  # default: "username"
    passwordKey: password  # default: "password"
    emailKey: email        # default: "email"
    create: true           # auto-create Secret with random password
```

### Token Caching

The operator caches authentication tokens to reduce API calls.

```yaml
apiVersion: rustcloak.k8s.withlazers.dev/v1
kind: KeycloakInstance
spec:
  token:
    secretName: keycloak-token  # default: "{instance}-api-token"
    tokenKey: token             # default: "token"
    expiresKey: expires         # default: "expires"
```

## Status Reporting

Each resource includes a `.status` field that reports:

| Field | Description |
|-------|-------------|
| `ready` | Boolean indicating if the resource is successfully reconciled |
| `endpoint.resourcePath` | The Keycloak API path for this resource |
| `endpoint.realmPath` | The realm path (for realm-scoped resources) |
| `conditions` | Standard Kubernetes conditions with detailed status |

Example status:

```yaml
status:
  ready: true
  endpoint:
    resourcePath: admin/realms/my-realm/clients/abc-123
    realmPath: admin/realms/my-realm
  conditions:
    - type: Ready
      status: "True"
      lastTransitionTime: "2024-01-15T10:30:00Z"
```

### Common Status Conditions

- **Ready**: Resource is successfully reconciled
- **ParentNotReady**: Waiting for parent resource to become ready
- **Error**: Reconciliation failed (check message for details)

## API Options

Resources support an `options` field for advanced configuration:

```yaml
apiVersion: rustcloak.k8s.withlazers.dev/v1
kind: KeycloakClient
spec:
  options:
    patchFrom:
      - secretKeyRef:
          name: client-patches
          key: patch.json
```

See [Patches](./configuration/patches.md) for more details.
