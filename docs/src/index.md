<div align="center">
    <a href="/"><img src="https://github.com/withlazers/rustcloak-operator/raw/refs/heads/main/icon.svg"></a>
</div>

# About

The rustcloak operator is a Kubernetes operator that manages Keycloak instances 
through the [Keycloak Admin API][1].

## Features

* Supports nearly all Keycloak endpoints.

* Legacy mode to be used as a drop-in[^1] replacement for the [keycloak-realm-operator][2].

* Supports multiple Keycloak instances.

## Non-Features

While managing the lifecycle of Keycloak Objects is supported, we believe that the
deployment of Keycloak itself is too inflexible to be managed by an operator. Modern
deployment tools like Helm are better suited for this task.

[^1]: While rustcloak can manage the CRDs of the keycloak-realm-operator it is currently
      not able to manage instances that have been managed by the keycloak-realm-operator.
      You currently need to redeploy Keycloak, allowing rustcloak have control over the
      lifecycle of the Keycloak objects.

[1]: https://www.keycloak.org/docs-api/latest/rest-api/

[2]: https://github.com/keycloak/keycloak-realm-operator
