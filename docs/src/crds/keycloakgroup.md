# KeycloakGroup

## v1

resource to define a Group within a [KeycloakRealm](./keycloakrealm.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
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
|[spec.realmRef](#specrealmref)|string|✅|
|[status](#status)|object||
|[status.conditions[]](#statusconditions)|object||
|[status.conditions[].lastTransitionTime](#statusconditionslasttransitiontime)|string||
|[status.conditions[].lastUpdateTime](#statusconditionslastupdatetime)|string||
|[status.conditions[].message](#statusconditionsmessage)|string||
|[status.conditions[].reason](#statusconditionsreason)|string||
|[status.conditions[].status](#statusconditionsstatus)|string|✅|
|[status.conditions[].type](#statusconditionstype)|string|✅|
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
|[options](#specoptions)|object||
|[realmRef](#specrealmref)|string|✅|

the KeycloakGroup resource

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

*missing*

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

### spec.realmRef

Type: string

the name of the kubernetes object that created the realm.

---

### status

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[conditions[]](#statusconditions)|object||
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