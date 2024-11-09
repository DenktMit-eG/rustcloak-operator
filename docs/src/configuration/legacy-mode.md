# Legacy Mode

Rustcloak supports a mode called "legacy mode" in which it can control
the CRDs of the [keycloak-realms-operator][1].

Please note that Rustcloak cannot run alongside the
keycloak-realm-operator. It is a good idea to remove all objects
managed by the old controller before starting Rustcloak and redeploying
it for rustcloak to take over.

To enable legacy mode, enabled the `legacy: true` option in the rustcloak
helm chart.

[1]: https://github.com/keycloak/keycloak-realm-operator
