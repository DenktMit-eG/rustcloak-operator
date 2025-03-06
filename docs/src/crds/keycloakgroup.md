# KeycloakGroup

## v1beta1

resource to define a Group within a [KeycloakRealm](./keycloakrealm.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clusterRealmRef](#specclusterrealmref)|string||
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.access](#specdefinitionaccess)|object||
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.clientRoles](#specdefinitionclientroles)|object||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.definition.parentId](#specdefinitionparentid)|string||
|[spec.definition.path](#specdefinitionpath)|string||
|[spec.definition.realmRoles[]](#specdefinitionrealmroles)|string||
|[spec.definition.subGroupCount](#specdefinitionsubgroupcount)|integer||
|[spec.options](#specoptions)|object||
|[spec.parentGroupRef](#specparentgroupref)|string||
|[spec.patchFrom](#specpatchfrom)|object||
|[spec.realmRef](#specrealmref)|string||
|[status](#status)|object||
|[status.conditions[]](#statusconditions)|object||
|[status.conditions[].lastTransitionTime](#statusconditionslasttransitiontime)|string||
|[status.conditions[].message](#statusconditionsmessage)|string||
|[status.conditions[].reason](#statusconditionsreason)|string||
|[status.conditions[].status](#statusconditionsstatus)|string|✅|
|[status.conditions[].type](#statusconditionstype)|string|✅|
|[status.instance](#statusinstance)|object||
|[status.instance.clusterInstanceRef](#statusinstanceclusterinstanceref)|string||
|[status.instance.instanceRef](#statusinstanceinstanceref)|string||
|[status.message](#statusmessage)|string||
|[status.ready](#statusready)|boolean|✅|
|[status.resourcePath](#statusresourcepath)|string||
|[status.status](#statusstatus)|string||

---

### spec

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clusterRealmRef](#specclusterrealmref)|string||
|[definition](#specdefinition)|object|✅|
|[options](#specoptions)|object||
|[parentGroupRef](#specparentgroupref)|string||
|[patchFrom](#specpatchfrom)|object||
|[realmRef](#specrealmref)|string||

the KeycloakGroup resource

---

### spec.clusterRealmRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the cluster realm to which this object belongs to

---

### spec.definition

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[access](#specdefinitionaccess)|object||
|[attributes](#specdefinitionattributes)|object||
|[clientRoles](#specdefinitionclientroles)|object||
|[id](#specdefinitionid)|string||
|[name](#specdefinitionname)|string||
|[parentId](#specdefinitionparentid)|string||
|[path](#specdefinitionpath)|string||
|[realmRoles[]](#specdefinitionrealmroles)|string||
|[subGroupCount](#specdefinitionsubgroupcount)|integer||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

GroupRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "access": { "type": "object", "additionalProperties": { "type": "boolean" } }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "clientRoles": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "id": { "type": "string" }, "name": { "type": "string" }, "parentId": { "type": "string" }, "path": { "type": "string" }, "realmRoles": { "type": "array", "items": { "type": "string" } }, "subGroupCount": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "subGroups": { "type": "array", "items": { "$ref": "#/$defs/GroupRepresentation" } } }, "additionalProperties": false } ``` </details>

---

### spec.definition.access

Type: object

*missing*

---

### spec.definition.attributes

Type: object

*missing*

---

### spec.definition.clientRoles

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

### spec.definition.name

Type: string

*missing*

---

### spec.definition.parentId

Type: string

*missing*

---

### spec.definition.path

Type: string

*missing*

---

### spec.definition.realmRoles[]

Type: string

*missing*

---

### spec.definition.subGroupCount

Type: integer

*missing*

---

### spec.options

Type: object

Options for the request to the Keycloak Admin API.

---

### spec.parentGroupRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

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
|[instance](#statusinstance)|object||
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

### status.instance

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clusterInstanceRef](#statusinstanceclusterinstanceref)|string||
|[instanceRef](#statusinstanceinstanceref)|string||

*missing*

---

### status.instance.clusterInstanceRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the cluster instance to which this object belongs to.

---

### status.instance.instanceRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the namespaced instance to which this object belongs to.

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