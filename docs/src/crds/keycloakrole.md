# KeycloakRole

## v1

resource to define a Protocol Mapper within either a [KeycloakRealm](./keycloakrealm.md) or a [KeycloakClient](./keycloakclient.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clientRef](#specclientref)|string||
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.clientRole](#specdefinitionclientrole)|boolean||
|[spec.definition.composite](#specdefinitioncomposite)|boolean||
|[spec.definition.composites](#specdefinitioncomposites)|object||
|[spec.definition.composites.application](#specdefinitioncompositesapplication)|object||
|[spec.definition.composites.client](#specdefinitioncompositesclient)|object||
|[spec.definition.composites.realm[]](#specdefinitioncompositesrealm)|string||
|[spec.definition.containerId](#specdefinitioncontainerid)|string||
|[spec.definition.description](#specdefinitiondescription)|string||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.definition.scopeParamRequired](#specdefinitionscopeparamrequired)|boolean||
|[spec.options](#specoptions)|object||
|[spec.patchFrom](#specpatchfrom)|object||
|[spec.realmRef](#specrealmref)|string||
|[status](#status)|object||
|[status.conditions[]](#statusconditions)|object||
|[status.conditions[].lastTransitionTime](#statusconditionslasttransitiontime)|string||
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
|[clientRef](#specclientref)|string||
|[definition](#specdefinition)|object|✅|
|[options](#specoptions)|object||
|[patchFrom](#specpatchfrom)|object||
|[realmRef](#specrealmref)|string||

the KeycloakRole resource

---

### spec.clientRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.definition

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[attributes](#specdefinitionattributes)|object||
|[clientRole](#specdefinitionclientrole)|boolean||
|[composite](#specdefinitioncomposite)|boolean||
|[composites](#specdefinitioncomposites)|object||
|[containerId](#specdefinitioncontainerid)|string||
|[description](#specdefinitiondescription)|string||
|[id](#specdefinitionid)|string||
|[name](#specdefinitionname)|string||
|[scopeParamRequired](#specdefinitionscopeparamrequired)|boolean||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.name) == has(oldSelf.name)|Value is immutable|

RoleRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "clientRole": { "type": "boolean" }, "composite": { "type": "boolean" }, "composites": { "$ref": "#/$defs/Composites" }, "containerId": { "type": "string" }, "description": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "scopeParamRequired": { "type": "boolean" } }, "additionalProperties": false } ``` </details>

---

### spec.definition.attributes

Type: object

*missing*

---

### spec.definition.clientRole

Type: boolean

*missing*

---

### spec.definition.composite

Type: boolean

*missing*

---

### spec.definition.composites

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[application](#specdefinitioncompositesapplication)|object||
|[client](#specdefinitioncompositesclient)|object||
|[realm[]](#specdefinitioncompositesrealm)|string||

Composites

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "application": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "client": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "realm": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } }, "additionalProperties": false } ``` </details>

---

### spec.definition.composites.application

Type: object

*missing*

---

### spec.definition.composites.client

Type: object

*missing*

---

### spec.definition.composites.realm[]

Type: string

*missing*

---

### spec.definition.containerId

Type: string

*missing*

---

### spec.definition.description

Type: string

*missing*

---

### spec.definition.id

Type: string

*missing*

---

### spec.definition.name

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.definition.scopeParamRequired

Type: boolean

*missing*

---

### spec.options

Type: object

Options for the request to the Keycloak Admin API.

---

### spec.patchFrom

Type: object

Defines additional values that can be loaded from secrets or configmaps. Field selectors are not supported. For more informations see [the patches documentation](../configuration/patches.md).

---

### spec.realmRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the realm to which this object belongs to

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

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the instance to which this object belongs to.

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