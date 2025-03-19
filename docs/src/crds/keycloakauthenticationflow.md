# KeycloakAuthenticationFlow

## v1beta1

resource to define an Authentication Flow within a [KeycloakRealm](./keycloakrealm.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clusterRealmRef](#specclusterrealmref)|string||
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.alias](#specdefinitionalias)|string||
|[spec.definition.authenticationExecutions[]](#specdefinitionauthenticationexecutions)|object||
|[spec.definition.authenticationExecutions[].authenticator](#specdefinitionauthenticationexecutionsauthenticator)|string||
|[spec.definition.authenticationExecutions[].authenticatorConfig](#specdefinitionauthenticationexecutionsauthenticatorconfig)|string||
|[spec.definition.authenticationExecutions[].authenticatorFlow](#specdefinitionauthenticationexecutionsauthenticatorflow)|boolean||
|[spec.definition.authenticationExecutions[].autheticatorFlow](#specdefinitionauthenticationexecutionsautheticatorflow)|boolean||
|[spec.definition.authenticationExecutions[].flowAlias](#specdefinitionauthenticationexecutionsflowalias)|string||
|[spec.definition.authenticationExecutions[].priority](#specdefinitionauthenticationexecutionspriority)|integer||
|[spec.definition.authenticationExecutions[].requirement](#specdefinitionauthenticationexecutionsrequirement)|string||
|[spec.definition.authenticationExecutions[].userSetupAllowed](#specdefinitionauthenticationexecutionsusersetupallowed)|boolean||
|[spec.definition.builtIn](#specdefinitionbuiltin)|boolean||
|[spec.definition.description](#specdefinitiondescription)|string||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.providerId](#specdefinitionproviderid)|string||
|[spec.definition.topLevel](#specdefinitiontoplevel)|boolean||
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
|[status.resourcePath](#statusresourcepath)|string||
|[status.status](#statusstatus)|string||

---

### spec

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clusterRealmRef](#specclusterrealmref)|string||
|[definition](#specdefinition)|object|✅|
|[patchFrom[]](#specpatchfrom)|object||
|[realmRef](#specrealmref)|string||

the KeycloakAuthenticationFlow resource

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
|[alias](#specdefinitionalias)|string||
|[authenticationExecutions[]](#specdefinitionauthenticationexecutions)|object||
|[builtIn](#specdefinitionbuiltin)|boolean||
|[description](#specdefinitiondescription)|string||
|[id](#specdefinitionid)|string||
|[providerId](#specdefinitionproviderid)|string||
|[topLevel](#specdefinitiontoplevel)|boolean||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

AuthenticationFlowRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "alias": { "type": "string" }, "authenticationExecutions": { "type": "array", "items": { "$ref": "#/$defs/AuthenticationExecutionExportRepresentation" } }, "builtIn": { "type": "boolean" }, "description": { "type": "string" }, "id": { "type": "string" }, "providerId": { "type": "string" }, "topLevel": { "type": "boolean" } } } ``` </details>

---

### spec.definition.alias

Type: string

*missing*

---

### spec.definition.authenticationExecutions[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[authenticator](#specdefinitionauthenticationexecutionsauthenticator)|string||
|[authenticatorConfig](#specdefinitionauthenticationexecutionsauthenticatorconfig)|string||
|[authenticatorFlow](#specdefinitionauthenticationexecutionsauthenticatorflow)|boolean||
|[autheticatorFlow](#specdefinitionauthenticationexecutionsautheticatorflow)|boolean||
|[flowAlias](#specdefinitionauthenticationexecutionsflowalias)|string||
|[priority](#specdefinitionauthenticationexecutionspriority)|integer||
|[requirement](#specdefinitionauthenticationexecutionsrequirement)|string||
|[userSetupAllowed](#specdefinitionauthenticationexecutionsusersetupallowed)|boolean||

AuthenticationExecutionExportRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "authenticator": { "type": "string" }, "authenticatorConfig": { "type": "string" }, "authenticatorFlow": { "type": "boolean" }, "autheticatorFlow": { "type": "boolean" }, "flowAlias": { "type": "string" }, "priority": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "requirement": { "type": "string", "enum": [ "ALTERNATIVE", "CONDITIONAL", "DISABLED", "REQUIRED" ] }, "userSetupAllowed": { "type": "boolean" } } } ``` </details>

---

### spec.definition.authenticationExecutions[].authenticator

Type: string

*missing*

---

### spec.definition.authenticationExecutions[].authenticatorConfig

Type: string

*missing*

---

### spec.definition.authenticationExecutions[].authenticatorFlow

Type: boolean

*missing*

---

### spec.definition.authenticationExecutions[].autheticatorFlow

Type: boolean

*missing*

---

### spec.definition.authenticationExecutions[].flowAlias

Type: string

*missing*

---

### spec.definition.authenticationExecutions[].priority

Type: integer

*missing*

---

### spec.definition.authenticationExecutions[].requirement

Type: string

AuthenticationExecutionExportRepresentationRequirement

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "ALTERNATIVE", "CONDITIONAL", "DISABLED", "REQUIRED" ] } ``` </details>

---

### spec.definition.authenticationExecutions[].userSetupAllowed

Type: boolean

*missing*

---

### spec.definition.builtIn

Type: boolean

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

### spec.definition.providerId

Type: string

*missing*

---

### spec.definition.topLevel

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