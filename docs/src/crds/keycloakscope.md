# KeycloakScope

## v1

resource to define a Scope within a [KeyclaokClient](./keycloakclient.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clientRef](#specclientref)|string|✅|
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.displayName](#specdefinitiondisplayname)|string||
|[spec.definition.iconUri](#specdefinitioniconuri)|string||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.definition.policies[]](#specdefinitionpolicies)|object||
|[spec.definition.policies[].config](#specdefinitionpoliciesconfig)|object||
|[spec.definition.policies[].decisionStrategy](#specdefinitionpoliciesdecisionstrategy)|string||
|[spec.definition.policies[].description](#specdefinitionpoliciesdescription)|string||
|[spec.definition.policies[].id](#specdefinitionpoliciesid)|string||
|[spec.definition.policies[].logic](#specdefinitionpolicieslogic)|string||
|[spec.definition.policies[].name](#specdefinitionpoliciesname)|string||
|[spec.definition.policies[].owner](#specdefinitionpoliciesowner)|string||
|[spec.definition.policies[].policies[]](#specdefinitionpoliciespolicies)|string||
|[spec.definition.policies[].resourceType](#specdefinitionpoliciesresourcetype)|string||
|[spec.definition.policies[].resources[]](#specdefinitionpoliciesresources)|string||
|[spec.definition.policies[].resourcesData[]](#specdefinitionpoliciesresourcesdata)|object||
|[spec.definition.policies[].resourcesData[]._id](#specdefinitionpoliciesresourcesdataid)|string||
|[spec.definition.policies[].resourcesData[].attributes](#specdefinitionpoliciesresourcesdataattributes)|object||
|[spec.definition.policies[].resourcesData[].displayName](#specdefinitionpoliciesresourcesdatadisplayname)|string||
|[spec.definition.policies[].resourcesData[].icon_uri](#specdefinitionpoliciesresourcesdataiconuri)|string||
|[spec.definition.policies[].resourcesData[].name](#specdefinitionpoliciesresourcesdataname)|string||
|[spec.definition.policies[].resourcesData[].owner](#specdefinitionpoliciesresourcesdataowner)|object||
|[spec.definition.policies[].resourcesData[].owner.id](#specdefinitionpoliciesresourcesdataownerid)|string||
|[spec.definition.policies[].resourcesData[].owner.name](#specdefinitionpoliciesresourcesdataownername)|string||
|[spec.definition.policies[].resourcesData[].ownerManagedAccess](#specdefinitionpoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.policies[].resourcesData[].type](#specdefinitionpoliciesresourcesdatatype)|string||
|[spec.definition.policies[].resourcesData[].uri](#specdefinitionpoliciesresourcesdatauri)|string||
|[spec.definition.policies[].resourcesData[].uris[]](#specdefinitionpoliciesresourcesdatauris)|string||
|[spec.definition.policies[].scopes[]](#specdefinitionpoliciesscopes)|string||
|[spec.definition.policies[].type](#specdefinitionpoliciestype)|string||
|[spec.definition.resources[]](#specdefinitionresources)|object||
|[spec.definition.resources[]._id](#specdefinitionresourcesid)|string||
|[spec.definition.resources[].attributes](#specdefinitionresourcesattributes)|object||
|[spec.definition.resources[].displayName](#specdefinitionresourcesdisplayname)|string||
|[spec.definition.resources[].icon_uri](#specdefinitionresourcesiconuri)|string||
|[spec.definition.resources[].name](#specdefinitionresourcesname)|string||
|[spec.definition.resources[].owner](#specdefinitionresourcesowner)|object||
|[spec.definition.resources[].owner.id](#specdefinitionresourcesownerid)|string||
|[spec.definition.resources[].owner.name](#specdefinitionresourcesownername)|string||
|[spec.definition.resources[].ownerManagedAccess](#specdefinitionresourcesownermanagedaccess)|boolean||
|[spec.definition.resources[].type](#specdefinitionresourcestype)|string||
|[spec.definition.resources[].uri](#specdefinitionresourcesuri)|string||
|[spec.definition.resources[].uris[]](#specdefinitionresourcesuris)|string||
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

the KeycloakScope resource

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
|[displayName](#specdefinitiondisplayname)|string||
|[iconUri](#specdefinitioniconuri)|string||
|[id](#specdefinitionid)|string||
|[name](#specdefinitionname)|string||
|[policies[]](#specdefinitionpolicies)|object||
|[resources[]](#specdefinitionresources)|object||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } }, "additionalProperties": false } ``` </details>

---

### spec.definition.displayName

Type: string

*missing*

---

### spec.definition.iconUri

Type: string

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

### spec.definition.policies[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionpoliciesconfig)|object||
|[decisionStrategy](#specdefinitionpoliciesdecisionstrategy)|string||
|[description](#specdefinitionpoliciesdescription)|string||
|[id](#specdefinitionpoliciesid)|string||
|[logic](#specdefinitionpolicieslogic)|string||
|[name](#specdefinitionpoliciesname)|string||
|[owner](#specdefinitionpoliciesowner)|string||
|[policies[]](#specdefinitionpoliciespolicies)|string||
|[resourceType](#specdefinitionpoliciesresourcetype)|string||
|[resources[]](#specdefinitionpoliciesresources)|string||
|[resourcesData[]](#specdefinitionpoliciesresourcesdata)|object||
|[scopes[]](#specdefinitionpoliciesscopes)|string||
|[type](#specdefinitionpoliciestype)|string||

PolicyRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "decisionStrategy": { "$ref": "#/$defs/DecisionStrategy" }, "description": { "type": "string" }, "id": { "type": "string" }, "logic": { "$ref": "#/$defs/Logic" }, "name": { "type": "string" }, "owner": { "type": "string" }, "policies": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourceType": { "type": "string" }, "resources": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourcesData": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" }, "uniqueItems": true }, "scopes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "scopesData": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" } }, "additionalProperties": false } ``` </details>

---

### spec.definition.policies[].config

Type: object

*missing*

---

### spec.definition.policies[].decisionStrategy

Type: string

DecisionStrategy

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "AFFIRMATIVE", "UNANIMOUS", "CONSENSUS" ] } ``` </details>

---

### spec.definition.policies[].description

Type: string

*missing*

---

### spec.definition.policies[].id

Type: string

*missing*

---

### spec.definition.policies[].logic

Type: string

Logic

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "POSITIVE", "NEGATIVE" ] } ``` </details>

---

### spec.definition.policies[].name

Type: string

*missing*

---

### spec.definition.policies[].owner

Type: string

*missing*

---

### spec.definition.policies[].policies[]

Type: string

*missing*

---

### spec.definition.policies[].resourceType

Type: string

*missing*

---

### spec.definition.policies[].resources[]

Type: string

*missing*

---

### spec.definition.policies[].resourcesData[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[\_id](#specdefinitionpoliciesresourcesdataid)|string||
|[attributes](#specdefinitionpoliciesresourcesdataattributes)|object||
|[displayName](#specdefinitionpoliciesresourcesdatadisplayname)|string||
|[icon_uri](#specdefinitionpoliciesresourcesdataiconuri)|string||
|[name](#specdefinitionpoliciesresourcesdataname)|string||
|[owner](#specdefinitionpoliciesresourcesdataowner)|object||
|[ownerManagedAccess](#specdefinitionpoliciesresourcesdataownermanagedaccess)|boolean||
|[type](#specdefinitionpoliciesresourcesdatatype)|string||
|[uri](#specdefinitionpoliciesresourcesdatauri)|string||
|[uris[]](#specdefinitionpoliciesresourcesdatauris)|string||

ResourceRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "_id": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "displayName": { "type": "string" }, "icon_uri": { "type": "string" }, "name": { "type": "string" }, "owner": { "type": "object", "allOf": [ { "$ref": "#/$defs/ResourceOwnerRepresentation" } ] }, "ownerManagedAccess": { "type": "boolean" }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "scopesUma": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" }, "uri": { "type": "string" }, "uris": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } }, "additionalProperties": false } ``` </details>

---

### spec.definition.policies[].resourcesData[]._id

Type: string

*missing*

---

### spec.definition.policies[].resourcesData[].attributes

Type: object

*missing*

---

### spec.definition.policies[].resourcesData[].displayName

Type: string

*missing*

---

### spec.definition.policies[].resourcesData[].icon_uri

Type: string

*missing*

---

### spec.definition.policies[].resourcesData[].name

Type: string

*missing*

---

### spec.definition.policies[].resourcesData[].owner

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specdefinitionpoliciesresourcesdataownerid)|string||
|[name](#specdefinitionpoliciesresourcesdataownername)|string||

ResourceOwnerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "id": { "type": "string" }, "name": { "type": "string" } }, "additionalProperties": false } ``` </details>

---

### spec.definition.policies[].resourcesData[].owner.id

Type: string

*missing*

---

### spec.definition.policies[].resourcesData[].owner.name

Type: string

*missing*

---

### spec.definition.policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

---

### spec.definition.policies[].resourcesData[].type

Type: string

*missing*

---

### spec.definition.policies[].resourcesData[].uri

Type: string

*missing*

---

### spec.definition.policies[].resourcesData[].uris[]

Type: string

*missing*

---

### spec.definition.policies[].scopes[]

Type: string

*missing*

---

### spec.definition.policies[].type

Type: string

*missing*

---

### spec.definition.resources[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[\_id](#specdefinitionresourcesid)|string||
|[attributes](#specdefinitionresourcesattributes)|object||
|[displayName](#specdefinitionresourcesdisplayname)|string||
|[icon_uri](#specdefinitionresourcesiconuri)|string||
|[name](#specdefinitionresourcesname)|string||
|[owner](#specdefinitionresourcesowner)|object||
|[ownerManagedAccess](#specdefinitionresourcesownermanagedaccess)|boolean||
|[type](#specdefinitionresourcestype)|string||
|[uri](#specdefinitionresourcesuri)|string||
|[uris[]](#specdefinitionresourcesuris)|string||

ResourceRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "_id": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "displayName": { "type": "string" }, "icon_uri": { "type": "string" }, "name": { "type": "string" }, "owner": { "type": "object", "allOf": [ { "$ref": "#/$defs/ResourceOwnerRepresentation" } ] }, "ownerManagedAccess": { "type": "boolean" }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "scopesUma": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" }, "uri": { "type": "string" }, "uris": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } }, "additionalProperties": false } ``` </details>

---

### spec.definition.resources[]._id

Type: string

*missing*

---

### spec.definition.resources[].attributes

Type: object

*missing*

---

### spec.definition.resources[].displayName

Type: string

*missing*

---

### spec.definition.resources[].icon_uri

Type: string

*missing*

---

### spec.definition.resources[].name

Type: string

*missing*

---

### spec.definition.resources[].owner

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specdefinitionresourcesownerid)|string||
|[name](#specdefinitionresourcesownername)|string||

ResourceOwnerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "id": { "type": "string" }, "name": { "type": "string" } }, "additionalProperties": false } ``` </details>

---

### spec.definition.resources[].owner.id

Type: string

*missing*

---

### spec.definition.resources[].owner.name

Type: string

*missing*

---

### spec.definition.resources[].ownerManagedAccess

Type: boolean

*missing*

---

### spec.definition.resources[].type

Type: string

*missing*

---

### spec.definition.resources[].uri

Type: string

*missing*

---

### spec.definition.resources[].uris[]

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