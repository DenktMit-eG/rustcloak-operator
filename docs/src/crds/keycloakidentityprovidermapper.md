# KeycloakIdentityProviderMapper

## v1

resource to define a identity provider mapper within a [KeyclaokIdentityProvider](./keycloakidentityprovider.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.config](#specdefinitionconfig)|object||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.identityProviderAlias](#specdefinitionidentityprovideralias)|string||
|[spec.definition.identityProviderMapper](#specdefinitionidentityprovidermapper)|string||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.identityProviderRef](#specidentityproviderref)|string|✅|
|[spec.options](#specoptions)|object||
|[spec.patchFrom](#specpatchfrom)|object||
|[status](#status)|object||
|[status.conditions[]](#statusconditions)|object||
|[status.conditions[].lastTransitionTime](#statusconditionslasttransitiontime)|string||
|[status.conditions[].lastUpdateTime](#statusconditionslastupdatetime)|string||
|[status.conditions[].message](#statusconditionsmessage)|string||
|[status.conditions[].reason](#statusconditionsreason)|string||
|[status.conditions[].status](#statusconditionsstatus)|string|✅|
|[status.conditions[].type](#statusconditionstype)|string|✅|
|[status.instanceRef](#statusinstanceref)|string||
|[status.message](#statusmessage)|string||
|[status.ready](#statusready)|boolean|✅|
|[status.resourcePath](#statusresourcepath)|string||
|[status.status](#statusstatus)|string||

---

### spec

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[definition](#specdefinition)|object|✅|
|[identityProviderRef](#specidentityproviderref)|string|✅|
|[options](#specoptions)|object||
|[patchFrom](#specpatchfrom)|object||

the KeycloakIdentityProviderMapper resource

---

### spec.definition

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionconfig)|object||
|[id](#specdefinitionid)|string||
|[identityProviderAlias](#specdefinitionidentityprovideralias)|string||
|[identityProviderMapper](#specdefinitionidentityprovidermapper)|string||
|[name](#specdefinitionname)|string||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

*missing*

---

### spec.definition.config

Type: object

*missing*

---

### spec.definition.id

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.definition.identityProviderAlias

Type: string

*missing*

---

### spec.definition.identityProviderMapper

Type: string

*missing*

---

### spec.definition.name

Type: string

*missing*

---

### spec.identityProviderRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

the name of the kubernetes object that created the identity provider.

---

### spec.options

Type: object

Options for the request to the Keycloak Admin API.

---

### spec.patchFrom

Type: object

Defines additional values that can be loaded from secrets or configmaps. Field selectors are not supported. For more informations see [the patches documentation](../configuration/patches.md).

---

### status

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[conditions[]](#statusconditions)|object||
|[instanceRef](#statusinstanceref)|string||
|[message](#statusmessage)|string||
|[ready](#statusready)|boolean|✅|
|[resourcePath](#statusresourcepath)|string||
|[status](#statusstatus)|string||

*missing*

---

### status.conditions[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[lastTransitionTime](#statusconditionslasttransitiontime)|string||
|[lastUpdateTime](#statusconditionslastupdatetime)|string||
|[message](#statusconditionsmessage)|string||
|[reason](#statusconditionsreason)|string||
|[status](#statusconditionsstatus)|string|✅|
|[type](#statusconditionstype)|string|✅|

*missing*

---

### status.conditions[].lastTransitionTime

Type: string

Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.

---

### status.conditions[].lastUpdateTime

Type: string

Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.

---

### status.conditions[].message

Type: string

*missing*

---

### status.conditions[].reason

Type: string

*missing*

---

### status.conditions[].status

Type: string

*missing*

---

### status.conditions[].type

Type: string

*missing*

---

### status.instanceRef

Type: string

*missing*

---

### status.message

Type: string

*missing*

---

### status.ready

Type: boolean

*missing*

---

### status.resourcePath

Type: string

*missing*

---

### status.status

Type: string

*missing*