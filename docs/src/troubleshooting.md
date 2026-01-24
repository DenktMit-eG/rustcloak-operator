# Troubleshooting

This guide covers common issues when using the rustcloak-operator and how to resolve them.

## Checking Resource Status

### View resource status

```bash
# Check a specific resource
kubectl get keycloakrealm my-realm -o yaml

# List all rustcloak resources with their ready status
kubectl get keycloakinstance,keycloakrealm,keycloakclient -A

# Watch for changes
kubectl get keycloakrealm -w
```

### Check conditions

```bash
kubectl get keycloakrealm my-realm -o jsonpath='{.status.conditions[*].message}'
```

## Common Issues

### Instance Not Ready

**Symptoms:**
- `KeycloakInstance` shows `ready: false`
- Child resources report "parent not ready"

**Possible causes and solutions:**

1. **Connectivity issues**
   - Verify the Keycloak server is reachable from the cluster
   - Check if the `baseUrl` is correct and includes the protocol (`https://`)
   ```bash
   # Test connectivity from a pod
   kubectl run curl --rm -it --image=curlimages/curl -- curl -v https://keycloak.example.com
   ```

2. **Invalid credentials**
   - Verify the secret exists and contains correct credentials
   ```bash
   kubectl get secret keycloak-credentials -o yaml
   ```
   - Check that `usernameKey` and `passwordKey` match the secret's keys

3. **TLS certificate issues**
   - If using self-signed certificates, ensure the CA is trusted
   - Check operator logs for TLS-related errors

4. **Wrong realm**
   - The `realm` field defaults to "master" - verify this is correct for your setup
   - Service account authentication requires the client to exist in the specified realm

### Parent Not Ready Errors

**Symptoms:**
- Resource status shows "parent not ready" condition
- Resources stuck in pending state

**Solution:**

Resources must be created in order. Check the parent resource:

```bash
# For a KeycloakClient, check its parent realm
kubectl get keycloakrealm <realm-name> -o jsonpath='{.status.ready}'

# For a KeycloakRealm, check its parent instance
kubectl get keycloakinstance <instance-name> -o jsonpath='{.status.ready}'
```

Fix the parent resource issue first, then child resources will reconcile automatically.

### No Instance Found Errors

**Symptoms:**
- Error message: "no instance found"
- Resource not reconciling

**Solution:**

1. Verify the instance reference is correct:
   ```yaml
   spec:
     instanceRef: my-instance        # For namespaced instance
     # OR
     clusterInstanceRef: my-cluster-instance  # For cluster-scoped instance
   ```

2. For namespaced instances, verify the instance is in the same namespace

3. For cluster instances, verify the name matches exactly

### Secret Not Found

**Symptoms:**
- Instance not becoming ready
- Error about missing secret

**Solution:**

1. Create the secret if it doesn't exist:
   ```bash
   kubectl create secret generic keycloak-credentials \
     --from-literal=user=admin \
     --from-literal=password=admin-password
   ```

2. If using `create: true`, ensure the operator has RBAC permissions to create secrets

3. Verify the secret is in the same namespace as the KeycloakInstance

### Resource Stuck in Reconciling

**Symptoms:**
- Resource stays in a reconciling state
- Ready status doesn't change

**Possible causes and solutions:**

1. **API rate limiting**
   - Keycloak may be rate-limiting requests
   - Check operator logs for 429 errors

2. **Resource conflict**
   - Another process might be modifying the same Keycloak resource
   - Check Keycloak's admin console for manual changes

3. **Invalid definition**
   - The resource definition may have invalid fields
   - Check operator logs for validation errors
   ```bash
   kubectl logs -l app.kubernetes.io/name=rustcloak-operator -f
   ```

### Resource Not Deleted from Keycloak

**Symptoms:**
- Kubernetes resource deleted but Keycloak resource remains
- Finalizer not being removed

**Solution:**

1. Check operator logs for deletion errors
2. Verify the operator has connectivity to Keycloak
3. If the operator is unable to delete, you may need to manually remove the finalizer:
   ```bash
   kubectl patch keycloakrealm my-realm -p '{"metadata":{"finalizers":[]}}' --type=merge
   ```
   **Warning:** This will skip Keycloak cleanup - manually delete the resource from Keycloak.

## Operator Logs

### Viewing logs

```bash
# Basic logs
kubectl logs -l app.kubernetes.io/name=rustcloak-operator

# Follow logs
kubectl logs -l app.kubernetes.io/name=rustcloak-operator -f

# With timestamps
kubectl logs -l app.kubernetes.io/name=rustcloak-operator --timestamps
```

### Enable debug logging

Set the `RUST_LOG` environment variable in the operator deployment:

```yaml
env:
  - name: RUST_LOG
    value: "rustcloak_operator=debug"
```

For even more verbose output:

```yaml
env:
  - name: RUST_LOG
    value: "rustcloak_operator=trace,kube=debug"
```

### Common log messages

| Message | Meaning |
|---------|---------|
| `reconciling resource` | Normal - operator is processing a resource |
| `parent not ready` | Parent resource needs to be fixed first |
| `resource created` | Resource successfully created in Keycloak |
| `resource updated` | Resource successfully updated in Keycloak |
| `authentication failed` | Check credentials or connectivity |

## Resource Deletion Issues

### Stuck finalizers

If a resource won't delete due to a stuck finalizer:

1. Check why the finalizer is stuck (operator logs)
2. Fix the underlying issue if possible
3. As a last resort, remove the finalizer manually:
   ```bash
   kubectl patch <resource-type> <name> -p '{"metadata":{"finalizers":[]}}' --type=merge
   ```

### Orphaned Keycloak resources

If Keycloak resources remain after Kubernetes resources are deleted:

1. The operator may have been unavailable during deletion
2. Use the Keycloak admin console to manually clean up
3. Consider using Keycloak's built-in audit logs to track changes

## Debugging Tips

### Inspect the API object

View the full resource including status:

```bash
kubectl get keycloakrealm my-realm -o yaml
```

### Verify Keycloak state

Compare Kubernetes desired state with Keycloak actual state:

```bash
# Get the resource path from status
kubectl get keycloakrealm my-realm -o jsonpath='{.status.endpoint.resourcePath}'

# Use Keycloak API directly
curl -H "Authorization: Bearer $TOKEN" https://keycloak.example.com/admin/realms/my-realm
```

### Test with a minimal example

Start with a minimal configuration to isolate issues:

```yaml
apiVersion: rustcloak.k8s.withlazers.dev/v1
kind: KeycloakInstance
metadata:
  name: test-instance
spec:
  baseUrl: https://keycloak.example.com
  credentials:
    secretName: keycloak-credentials
---
apiVersion: rustcloak.k8s.withlazers.dev/v1
kind: KeycloakRealm
metadata:
  name: test-realm
spec:
  instanceRef: test-instance
  definition:
    realm: test
    enabled: true
```

### Check RBAC permissions

Ensure the operator has necessary permissions:

```bash
kubectl auth can-i --list --as=system:serviceaccount:rustcloak-system:rustcloak-operator
```

## Getting Help

If you're still experiencing issues:

1. Search existing issues: [GitHub Issues](https://github.com/withlazers/rustcloak-operator/issues)
2. Create a new issue with:
   - Operator version (`kubectl get deployment rustcloak-operator -o jsonpath='{.spec.template.spec.containers[0].image}'`)
   - Keycloak version
   - Resource YAML (sanitized)
   - Relevant operator logs
   - Steps to reproduce
