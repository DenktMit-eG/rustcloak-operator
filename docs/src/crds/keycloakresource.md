# KeycloakResource

## v1beta1

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
|[spec.patchFrom2[]](#specpatchfrom2)|object||
|[spec.patchFrom2[].configMapKeyRef](#specpatchfrom2configmapkeyref)|object||
|[spec.patchFrom2[].configMapKeyRef.key](#specpatchfrom2configmapkeyrefkey)|string|✅|
|[spec.patchFrom2[].configMapKeyRef.name](#specpatchfrom2configmapkeyrefname)|string|✅|
|[spec.patchFrom2[].configMapKeyRef.optional](#specpatchfrom2configmapkeyrefoptional)|boolean||
|[spec.patchFrom2[].fieldRef](#specpatchfrom2fieldref)|object||
|[spec.patchFrom2[].fieldRef.apiVersion](#specpatchfrom2fieldrefapiversion)|string||
|[spec.patchFrom2[].fieldRef.fieldPath](#specpatchfrom2fieldreffieldpath)|string|✅|
|[spec.patchFrom2[].path](#specpatchfrom2path)|string|✅|
|[spec.patchFrom2[].resourceFieldRef](#specpatchfrom2resourcefieldref)|object||
|[spec.patchFrom2[].resourceFieldRef.containerName](#specpatchfrom2resourcefieldrefcontainername)|string||
|[spec.patchFrom2[].resourceFieldRef.divisor](#specpatchfrom2resourcefieldrefdivisor)|string||
|[spec.patchFrom2[].resourceFieldRef.resource](#specpatchfrom2resourcefieldrefresource)|string|✅|
|[spec.patchFrom2[].secretKeyRef](#specpatchfrom2secretkeyref)|object||
|[spec.patchFrom2[].secretKeyRef.key](#specpatchfrom2secretkeyrefkey)|string|✅|
|[spec.patchFrom2[].secretKeyRef.name](#specpatchfrom2secretkeyrefname)|string|✅|
|[spec.patchFrom2[].secretKeyRef.optional](#specpatchfrom2secretkeyrefoptional)|boolean||
|[spec.patchFrom2[].valueAs](#specpatchfrom2valueas)|string||
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
|[clientRef](#specclientref)|string|✅|
|[definition](#specdefinition)|object|✅|
|[options](#specoptions)|object||
|[patchFrom](#specpatchfrom)|object||
|[patchFrom2[]](#specpatchfrom2)|object||

the KeycloakResource resource

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

ResourceRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "_id": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "displayName": { "type": "string" }, "icon_uri": { "type": "string" }, "name": { "type": "string" }, "owner": { "type": "object", "allOf": [ { "$ref": "#/$defs/ResourceOwnerRepresentation" } ] }, "ownerManagedAccess": { "type": "boolean" }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "scopesUma": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" }, "uri": { "type": "string" }, "uris": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } } } ``` </details>

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

ResourceOwnerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "id": { "type": "string" }, "name": { "type": "string" } } } ``` </details>

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

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } } } ``` </details>

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

PolicyRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "decisionStrategy": { "$ref": "#/$defs/DecisionStrategy" }, "description": { "type": "string" }, "id": { "type": "string" }, "logic": { "$ref": "#/$defs/Logic" }, "name": { "type": "string" }, "owner": { "type": "string" }, "policies": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourceType": { "type": "string" }, "resources": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourcesData": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" }, "uniqueItems": true }, "scopes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "scopesData": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" } } } ``` </details>

---

### spec.definition.scopes[].policies[].config

Type: object

*missing*

---

### spec.definition.scopes[].policies[].decisionStrategy

Type: string

DecisionStrategy

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "AFFIRMATIVE", "UNANIMOUS", "CONSENSUS" ] } ``` </details>

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

Logic

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "POSITIVE", "NEGATIVE" ] } ``` </details>

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

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } } } ``` </details>

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

PolicyRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "decisionStrategy": { "$ref": "#/$defs/DecisionStrategy" }, "description": { "type": "string" }, "id": { "type": "string" }, "logic": { "$ref": "#/$defs/Logic" }, "name": { "type": "string" }, "owner": { "type": "string" }, "policies": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourceType": { "type": "string" }, "resources": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourcesData": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" }, "uniqueItems": true }, "scopes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "scopesData": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" } } } ``` </details>

---

### spec.definition.scopesUma[].policies[].config

Type: object

*missing*

---

### spec.definition.scopesUma[].policies[].decisionStrategy

Type: string

DecisionStrategy

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "AFFIRMATIVE", "UNANIMOUS", "CONSENSUS" ] } ``` </details>

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

Logic

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "POSITIVE", "NEGATIVE" ] } ``` </details>

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

### spec.patchFrom2[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[configMapKeyRef](#specpatchfrom2configmapkeyref)|object||
|[fieldRef](#specpatchfrom2fieldref)|object||
|[path](#specpatchfrom2path)|string|✅|
|[resourceFieldRef](#specpatchfrom2resourcefieldref)|object||
|[secretKeyRef](#specpatchfrom2secretkeyref)|object||
|[valueAs](#specpatchfrom2valueas)|string||

EnvVarSource represents a source for the value of an EnvVar.

---

### spec.patchFrom2[].configMapKeyRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[key](#specpatchfrom2configmapkeyrefkey)|string|✅|
|[name](#specpatchfrom2configmapkeyrefname)|string|✅|
|[optional](#specpatchfrom2configmapkeyrefoptional)|boolean||

Selects a key of a ConfigMap.

---

### spec.patchFrom2[].configMapKeyRef.key

Type: string

The key to select.

---

### spec.patchFrom2[].configMapKeyRef.name

Type: string

Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names

---

### spec.patchFrom2[].configMapKeyRef.optional

Type: boolean

Specify whether the ConfigMap or its key must be defined

---

### spec.patchFrom2[].fieldRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[apiVersion](#specpatchfrom2fieldrefapiversion)|string||
|[fieldPath](#specpatchfrom2fieldreffieldpath)|string|✅|

Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.

---

### spec.patchFrom2[].fieldRef.apiVersion

Type: string

Version of the schema the FieldPath is written in terms of, defaults to "v1".

---

### spec.patchFrom2[].fieldRef.fieldPath

Type: string

Path of the field to select in the specified API version.

---

### spec.patchFrom2[].path

Type: string

*missing*

---

### spec.patchFrom2[].resourceFieldRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[containerName](#specpatchfrom2resourcefieldrefcontainername)|string||
|[divisor](#specpatchfrom2resourcefieldrefdivisor)|string||
|[resource](#specpatchfrom2resourcefieldrefresource)|string|✅|

Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.

---

### spec.patchFrom2[].resourceFieldRef.containerName

Type: string

Container name: required for volumes, optional for env vars

---

### spec.patchFrom2[].resourceFieldRef.divisor

Type: string

Specifies the output format of the exposed resources, defaults to "1"

---

### spec.patchFrom2[].resourceFieldRef.resource

Type: string

Required: resource to select

---

### spec.patchFrom2[].secretKeyRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[key](#specpatchfrom2secretkeyrefkey)|string|✅|
|[name](#specpatchfrom2secretkeyrefname)|string|✅|
|[optional](#specpatchfrom2secretkeyrefoptional)|boolean||

Selects a key of a secret in the pod's namespace

---

### spec.patchFrom2[].secretKeyRef.key

Type: string

The key of the secret to select from.  Must be a valid secret key.

---

### spec.patchFrom2[].secretKeyRef.name

Type: string

Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names

---

### spec.patchFrom2[].secretKeyRef.optional

Type: boolean

Specify whether the Secret or its key must be defined

---

### spec.patchFrom2[].valueAs

Type: string

*missing*

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