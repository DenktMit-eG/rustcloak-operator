# KeycloakResource

## v1

resource to define a Resource within a [KeyclaokClient](./keycloakclient.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clientRef](#specclientref)|string|✅|
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition._id](#specdefinitionid)|string||
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.displayName](#specdefinitiondisplayname)|string||
|[spec.definition.icon_uri](#specdefinitioniconuri)|string||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.definition.owner](#specdefinitionowner)|object||
|[spec.definition.owner.id](#specdefinitionownerid)|string||
|[spec.definition.owner.name](#specdefinitionownername)|string||
|[spec.definition.ownerManagedAccess](#specdefinitionownermanagedaccess)|boolean||
|[spec.definition.scopes[]](#specdefinitionscopes)|object||
|[spec.definition.scopes[].displayName](#specdefinitionscopesdisplayname)|string||
|[spec.definition.scopes[].iconUri](#specdefinitionscopesiconuri)|string||
|[spec.definition.scopes[].id](#specdefinitionscopesid)|string||
|[spec.definition.scopes[].name](#specdefinitionscopesname)|string||
|[spec.definition.scopes[].policies[]](#specdefinitionscopespolicies)|object||
|[spec.definition.scopes[].policies[].config](#specdefinitionscopespoliciesconfig)|object||
|[spec.definition.scopes[].policies[].decisionStrategy](#specdefinitionscopespoliciesdecisionstrategy)|string||
|[spec.definition.scopes[].policies[].description](#specdefinitionscopespoliciesdescription)|string||
|[spec.definition.scopes[].policies[].id](#specdefinitionscopespoliciesid)|string||
|[spec.definition.scopes[].policies[].logic](#specdefinitionscopespolicieslogic)|string||
|[spec.definition.scopes[].policies[].name](#specdefinitionscopespoliciesname)|string||
|[spec.definition.scopes[].policies[].owner](#specdefinitionscopespoliciesowner)|string||
|[spec.definition.scopes[].policies[].policies[]](#specdefinitionscopespoliciespolicies)|string||
|[spec.definition.scopes[].policies[].resourceType](#specdefinitionscopespoliciesresourcetype)|string||
|[spec.definition.scopes[].policies[].resources[]](#specdefinitionscopespoliciesresources)|string||
|[spec.definition.scopes[].policies[].scopes[]](#specdefinitionscopespoliciesscopes)|string||
|[spec.definition.scopes[].policies[].type](#specdefinitionscopespoliciestype)|string||
|[spec.definition.scopesUma[]](#specdefinitionscopesuma)|object||
|[spec.definition.scopesUma[].displayName](#specdefinitionscopesumadisplayname)|string||
|[spec.definition.scopesUma[].iconUri](#specdefinitionscopesumaiconuri)|string||
|[spec.definition.scopesUma[].id](#specdefinitionscopesumaid)|string||
|[spec.definition.scopesUma[].name](#specdefinitionscopesumaname)|string||
|[spec.definition.scopesUma[].policies[]](#specdefinitionscopesumapolicies)|object||
|[spec.definition.scopesUma[].policies[].config](#specdefinitionscopesumapoliciesconfig)|object||
|[spec.definition.scopesUma[].policies[].decisionStrategy](#specdefinitionscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.scopesUma[].policies[].description](#specdefinitionscopesumapoliciesdescription)|string||
|[spec.definition.scopesUma[].policies[].id](#specdefinitionscopesumapoliciesid)|string||
|[spec.definition.scopesUma[].policies[].logic](#specdefinitionscopesumapolicieslogic)|string||
|[spec.definition.scopesUma[].policies[].name](#specdefinitionscopesumapoliciesname)|string||
|[spec.definition.scopesUma[].policies[].owner](#specdefinitionscopesumapoliciesowner)|string||
|[spec.definition.scopesUma[].policies[].policies[]](#specdefinitionscopesumapoliciespolicies)|string||
|[spec.definition.scopesUma[].policies[].resourceType](#specdefinitionscopesumapoliciesresourcetype)|string||
|[spec.definition.scopesUma[].policies[].resources[]](#specdefinitionscopesumapoliciesresources)|string||
|[spec.definition.scopesUma[].policies[].scopes[]](#specdefinitionscopesumapoliciesscopes)|string||
|[spec.definition.scopesUma[].policies[].type](#specdefinitionscopesumapoliciestype)|string||
|[spec.definition.type](#specdefinitiontype)|string||
|[spec.definition.uri](#specdefinitionuri)|string||
|[spec.definition.uris[]](#specdefinitionuris)|string||
|[spec.options](#specoptions)|object||
|[spec.patchFrom](#specpatchfrom)|object||
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
|[clientRef](#specclientref)|string|✅|
|[definition](#specdefinition)|object|✅|
|[options](#specoptions)|object||
|[patchFrom](#specpatchfrom)|object||

the KeycloakResource resource

---

### spec.clientRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

the name of the kubernetes object that created the client.

---

### spec.definition

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[\_id](#specdefinitionid)|string||
|[attributes](#specdefinitionattributes)|object||
|[displayName](#specdefinitiondisplayname)|string||
|[icon_uri](#specdefinitioniconuri)|string||
|[name](#specdefinitionname)|string||
|[owner](#specdefinitionowner)|object||
|[ownerManagedAccess](#specdefinitionownermanagedaccess)|boolean||
|[scopes[]](#specdefinitionscopes)|object||
|[scopesUma[]](#specdefinitionscopesuma)|object||
|[type](#specdefinitiontype)|string||
|[uri](#specdefinitionuri)|string||
|[uris[]](#specdefinitionuris)|string||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self._id) == has(oldSelf._id)|Value is immutable|

*missing*

---

### spec.definition._id

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.definition.attributes

Type: object

*missing*

---

### spec.definition.displayName

Type: string

*missing*

---

### spec.definition.icon_uri

Type: string

*missing*

---

### spec.definition.name

Type: string

*missing*

---

### spec.definition.owner

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specdefinitionownerid)|string||
|[name](#specdefinitionownername)|string||

*missing*

---

### spec.definition.owner.id

Type: string

*missing*

---

### spec.definition.owner.name

Type: string

*missing*

---

### spec.definition.ownerManagedAccess

Type: boolean

*missing*

---

### spec.definition.scopes[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[displayName](#specdefinitionscopesdisplayname)|string||
|[iconUri](#specdefinitionscopesiconuri)|string||
|[id](#specdefinitionscopesid)|string||
|[name](#specdefinitionscopesname)|string||
|[policies[]](#specdefinitionscopespolicies)|object||

*missing*

---

### spec.definition.scopes[].displayName

Type: string

*missing*

---

### spec.definition.scopes[].iconUri

Type: string

*missing*

---

### spec.definition.scopes[].id

Type: string

*missing*

---

### spec.definition.scopes[].name

Type: string

*missing*

---

### spec.definition.scopes[].policies[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionscopespoliciesconfig)|object||
|[decisionStrategy](#specdefinitionscopespoliciesdecisionstrategy)|string||
|[description](#specdefinitionscopespoliciesdescription)|string||
|[id](#specdefinitionscopespoliciesid)|string||
|[logic](#specdefinitionscopespolicieslogic)|string||
|[name](#specdefinitionscopespoliciesname)|string||
|[owner](#specdefinitionscopespoliciesowner)|string||
|[policies[]](#specdefinitionscopespoliciespolicies)|string||
|[resourceType](#specdefinitionscopespoliciesresourcetype)|string||
|[resources[]](#specdefinitionscopespoliciesresources)|string||
|[scopes[]](#specdefinitionscopespoliciesscopes)|string||
|[type](#specdefinitionscopespoliciestype)|string||

*missing*

---

### spec.definition.scopes[].policies[].config

Type: object

*missing*

---

### spec.definition.scopes[].policies[].decisionStrategy

Type: string

*missing*

---

### spec.definition.scopes[].policies[].description

Type: string

*missing*

---

### spec.definition.scopes[].policies[].id

Type: string

*missing*

---

### spec.definition.scopes[].policies[].logic

Type: string

*missing*

---

### spec.definition.scopes[].policies[].name

Type: string

*missing*

---

### spec.definition.scopes[].policies[].owner

Type: string

*missing*

---

### spec.definition.scopes[].policies[].policies[]

Type: string

*missing*

---

### spec.definition.scopes[].policies[].resourceType

Type: string

*missing*

---

### spec.definition.scopes[].policies[].resources[]

Type: string

*missing*

---

### spec.definition.scopes[].policies[].scopes[]

Type: string

*missing*

---

### spec.definition.scopes[].policies[].type

Type: string

*missing*

---

### spec.definition.scopesUma[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[displayName](#specdefinitionscopesumadisplayname)|string||
|[iconUri](#specdefinitionscopesumaiconuri)|string||
|[id](#specdefinitionscopesumaid)|string||
|[name](#specdefinitionscopesumaname)|string||
|[policies[]](#specdefinitionscopesumapolicies)|object||

*missing*

---

### spec.definition.scopesUma[].displayName

Type: string

*missing*

---

### spec.definition.scopesUma[].iconUri

Type: string

*missing*

---

### spec.definition.scopesUma[].id

Type: string

*missing*

---

### spec.definition.scopesUma[].name

Type: string

*missing*

---

### spec.definition.scopesUma[].policies[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionscopesumapoliciesconfig)|object||
|[decisionStrategy](#specdefinitionscopesumapoliciesdecisionstrategy)|string||
|[description](#specdefinitionscopesumapoliciesdescription)|string||
|[id](#specdefinitionscopesumapoliciesid)|string||
|[logic](#specdefinitionscopesumapolicieslogic)|string||
|[name](#specdefinitionscopesumapoliciesname)|string||
|[owner](#specdefinitionscopesumapoliciesowner)|string||
|[policies[]](#specdefinitionscopesumapoliciespolicies)|string||
|[resourceType](#specdefinitionscopesumapoliciesresourcetype)|string||
|[resources[]](#specdefinitionscopesumapoliciesresources)|string||
|[scopes[]](#specdefinitionscopesumapoliciesscopes)|string||
|[type](#specdefinitionscopesumapoliciestype)|string||

*missing*

---

### spec.definition.scopesUma[].policies[].config

Type: object

*missing*

---

### spec.definition.scopesUma[].policies[].decisionStrategy

Type: string

*missing*

---

### spec.definition.scopesUma[].policies[].description

Type: string

*missing*

---

### spec.definition.scopesUma[].policies[].id

Type: string

*missing*

---

### spec.definition.scopesUma[].policies[].logic

Type: string

*missing*

---

### spec.definition.scopesUma[].policies[].name

Type: string

*missing*

---

### spec.definition.scopesUma[].policies[].owner

Type: string

*missing*

---

### spec.definition.scopesUma[].policies[].policies[]

Type: string

*missing*

---

### spec.definition.scopesUma[].policies[].resourceType

Type: string

*missing*

---

### spec.definition.scopesUma[].policies[].resources[]

Type: string

*missing*

---

### spec.definition.scopesUma[].policies[].scopes[]

Type: string

*missing*

---

### spec.definition.scopesUma[].policies[].type

Type: string

*missing*

---

### spec.definition.type

Type: string

*missing*

---

### spec.definition.uri

Type: string

*missing*

---

### spec.definition.uris[]

Type: string

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