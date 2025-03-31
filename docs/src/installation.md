# Installation

The prefered way to install the rustcloak operator is to use the provided helm chart. The helm chart is available through the [withlazers helm repository][1].

```shell
helm repo add withlazers https://charts.withlazers.dev
helm install rustcloak withlazers/rustcloak
```

For more information on how to configure the helm chart, see the [helm chart documentation][2] on Github.

[1]: https://charts.withlazers.dev
[2]: https://github.com/DenktMit-eG/rustcloak-operator/tree/main/charts/rustcloak-operator
