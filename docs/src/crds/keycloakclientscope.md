# KeycloakClientScope

## v1beta1

resource to define a Scope within a [KeycloakClient](./keycloakclient.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clusterRealmRef](#specclusterrealmref)|string||
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.description](#specdefinitiondescription)|string||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.definition.protocol](#specdefinitionprotocol)|string||
|[spec.definition.protocolMappers[]](#specdefinitionprotocolmappers)|object||
|[spec.definition.protocolMappers[].config](#specdefinitionprotocolmappersconfig)|object||
|[spec.definition.protocolMappers[].consentRequired](#specdefinitionprotocolmappersconsentrequired)|boolean||
|[spec.definition.protocolMappers[].consentText](#specdefinitionprotocolmappersconsenttext)|string||
|[spec.definition.protocolMappers[].id](#specdefinitionprotocolmappersid)|string||
|[spec.definition.protocolMappers[].name](#specdefinitionprotocolmappersname)|string||
|[spec.definition.protocolMappers[].protocol](#specdefinitionprotocolmappersprotocol)|string||
|[spec.definition.protocolMappers[].protocolMapper](#specdefinitionprotocolmappersprotocolmapper)|string||
|[spec.isTemplate](#specistemplate)|boolean||
|[spec.patchFrom[]](#specpatchfrom)|object||
|[spec.patchFrom[].configMapKeyRef](#specpatchfromconfigmapkeyref)|object||
|[spec.patchFrom[].configMapKeyRef.key](#specpatchfromconfigmapkeyrefkey)|string|✅|
|[spec.patchFrom[].configMapKeyRef.name](#specpatchfromconfigmapkeyrefname)|string|✅|
|[spec.patchFrom[].configMapKeyRef.optional](#specpatchfromconfigmapkeyrefoptional)|boolean||
|[spec.patchFrom[].path](#specpatchfrompath)|string|✅|
|[spec.patchFrom[].secretKeyRef](#specpatchfromsecretkeyref)|object||
|[spec.patchFrom[].secretKeyRef.key](#specpatchfromsecretkeyrefkey)|string|✅|
|[spec.patchFrom[].secretKeyRef.name](#specpatchfromsecretkeyrefname)|string|✅|
|[spec.patchFrom[].secretKeyRef.optional](#specpatchfromsecretkeyrefoptional)|boolean||
|[spec.patchFrom[].value](#specpatchfromvalue)|string||
|[spec.patchFrom[].value_as](#specpatchfromvalueas)|string||
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
|[status.realm](#statusrealm)|object||
|[status.realm.clusterRealmRef](#statusrealmclusterrealmref)|string||
|[status.realm.realmRef](#statusrealmrealmref)|string||
|[status.resourcePath](#statusresourcepath)|string||
|[status.status](#statusstatus)|string||

---

### spec

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clusterRealmRef](#specclusterrealmref)|string||
|[definition](#specdefinition)|object|✅|
|[isTemplate](#specistemplate)|boolean||
|[patchFrom[]](#specpatchfrom)|object||
|[realmRef](#specrealmref)|string||

the KeycloakClientScope resource

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
|[attributes](#specdefinitionattributes)|object||
|[description](#specdefinitiondescription)|string||
|[id](#specdefinitionid)|string||
|[name](#specdefinitionname)|string||
|[protocol](#specdefinitionprotocol)|string||
|[protocolMappers[]](#specdefinitionprotocolmappers)|object||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

ClientScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "attributes": { "type": "object", "additionalProperties": { "type": "string" } }, "description": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "protocol": { "type": "string", "enum": [ "openid-connect", "saml" ] }, "protocolMappers": { "type": "array", "items": { "$ref": "#/$defs/ProtocolMapperRepresentation" } } } } ``` </details>

---

### spec.definition.attributes

Type: object

*missing*

---

### spec.definition.description

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

### spec.definition.protocol

Type: string

ClientScopeRepresentationProtocol

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "openid-connect", "saml" ] } ``` </details>

---

### spec.definition.protocolMappers[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionprotocolmappersconfig)|object||
|[consentRequired](#specdefinitionprotocolmappersconsentrequired)|boolean||
|[consentText](#specdefinitionprotocolmappersconsenttext)|string||
|[id](#specdefinitionprotocolmappersid)|string||
|[name](#specdefinitionprotocolmappersname)|string||
|[protocol](#specdefinitionprotocolmappersprotocol)|string||
|[protocolMapper](#specdefinitionprotocolmappersprotocolmapper)|string||

ProtocolMapperRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "consentRequired": { "type": "boolean" }, "consentText": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "protocol": { "type": "string", "enum": [ "openid-connect", "saml" ] }, "protocolMapper": { "type": "string" } } } ``` </details>

---

### spec.definition.protocolMappers[].config

Type: object

*missing*

---

### spec.definition.protocolMappers[].consentRequired

Type: boolean

*missing*

---

### spec.definition.protocolMappers[].consentText

Type: string

*missing*

---

### spec.definition.protocolMappers[].id

Type: string

*missing*

---

### spec.definition.protocolMappers[].name

Type: string

*missing*

---

### spec.definition.protocolMappers[].protocol

Type: string

ProtocolMapperRepresentationProtocol

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "openid-connect", "saml" ] } ``` </details>

---

### spec.definition.protocolMappers[].protocolMapper

Type: string

*missing*

---

### spec.isTemplate

Type: boolean

*missing*

---

### spec.patchFrom[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[configMapKeyRef](#specpatchfromconfigmapkeyref)|object||
|[path](#specpatchfrompath)|string|✅|
|[secretKeyRef](#specpatchfromsecretkeyref)|object||
|[value](#specpatchfromvalue)|string||
|[value_as](#specpatchfromvalueas)|string||

*missing*

---

### spec.patchFrom[].configMapKeyRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[key](#specpatchfromconfigmapkeyrefkey)|string|✅|
|[name](#specpatchfromconfigmapkeyrefname)|string|✅|
|[optional](#specpatchfromconfigmapkeyrefoptional)|boolean||

Selects a key from a ConfigMap.

---

### spec.patchFrom[].configMapKeyRef.key

Type: string

The key to select.

---

### spec.patchFrom[].configMapKeyRef.name

Type: string

Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names

---

### spec.patchFrom[].configMapKeyRef.optional

Type: boolean

Specify whether the ConfigMap or its key must be defined

---

### spec.patchFrom[].path

Type: string

*missing*

---

### spec.patchFrom[].secretKeyRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[key](#specpatchfromsecretkeyrefkey)|string|✅|
|[name](#specpatchfromsecretkeyrefname)|string|✅|
|[optional](#specpatchfromsecretkeyrefoptional)|boolean||

SecretKeySelector selects a key of a Secret.

---

### spec.patchFrom[].secretKeyRef.key

Type: string

The key of the secret to select from.  Must be a valid secret key.

---

### spec.patchFrom[].secretKeyRef.name

Type: string

Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names

---

### spec.patchFrom[].secretKeyRef.optional

Type: boolean

Specify whether the Secret or its key must be defined

---

### spec.patchFrom[].value

Type: string

*missing*

---

### spec.patchFrom[].value_as

Type: string

*missing*

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
|[realm](#statusrealm)|object||
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

### status.realm

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clusterRealmRef](#statusrealmclusterrealmref)|string||
|[realmRef](#statusrealmrealmref)|string||

Optional for backwards compatibility

---

### status.realm.clusterRealmRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the cluster realm to which this object belongs to

---

### status.realm.realmRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the realm to which this object belongs to

---

### status.resourcePath

Type: string

*missing*

---

### status.status

Type: string

*missing*