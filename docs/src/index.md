<div align="center">
    <a href="/"><img src="https://github.com/DenktMit-eG/rustcloak-operator/raw/refs/heads/main/icon.svg"></a>
</div>

# About

The rustcloak operator is a Kubernetes operator that manages Keycloak instances 
through the [Keycloak Admin API][1]. The overall goal is to provide a cloud native
management interface for Keycloak instances.

## Goals

* Manage Keycloak instances solely through kubernetes resources.

* Provide a migration path for people that have been let down by Keycloak's own
  efforts to provide an operator.

## Non-Goals

* Manage the deployment of Keycloak instances.

* Support other IdM solutions than Keycloak.

[1]: https://www.keycloak.org/docs-api/latest/rest-api/

[2]: https://github.com/keycloak/keycloak-realm-operator
