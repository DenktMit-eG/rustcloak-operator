# ClusterKeycloakApiObject

## v1

Custom Resource for Keycloak API requests. The user should not use this resource directly.

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.endpoint](#specendpoint)|object|✅|
|[spec.endpoint.instanceRef](#specendpointinstanceref)|string|✅|
|[spec.endpoint.parent](#specendpointparent)|object||
|[spec.endpoint.parent.parent_ref](#specendpointparentparentref)|string|✅|
|[spec.endpoint.parent.sub_path](#specendpointparentsubpath)|string|✅|
|[spec.endpoint.path](#specendpointpath)|string||
|[spec.immutablePayload](#specimmutablepayload)|string|✅|
|[spec.options](#specoptions)|object||
|[spec.payload](#specpayload)|string|✅|
|[spec.vars[]](#specvars)|object||
|[spec.vars[].name](#specvarsname)|string|✅|
|[spec.vars[].value](#specvarsvalue)|string||
|[spec.vars[].valueFrom](#specvarsvaluefrom)|object||
|[spec.vars[].valueFrom.configMapKeyRef](#specvarsvaluefromconfigmapkeyref)|object||
|[spec.vars[].valueFrom.configMapKeyRef.key](#specvarsvaluefromconfigmapkeyrefkey)|string|✅|
|[spec.vars[].valueFrom.configMapKeyRef.name](#specvarsvaluefromconfigmapkeyrefname)|string|✅|
|[spec.vars[].valueFrom.configMapKeyRef.optional](#specvarsvaluefromconfigmapkeyrefoptional)|boolean||
|[spec.vars[].valueFrom.fieldRef](#specvarsvaluefromfieldref)|object||
|[spec.vars[].valueFrom.fieldRef.apiVersion](#specvarsvaluefromfieldrefapiversion)|string||
|[spec.vars[].valueFrom.fieldRef.fieldPath](#specvarsvaluefromfieldreffieldpath)|string|✅|
|[spec.vars[].valueFrom.resourceFieldRef](#specvarsvaluefromresourcefieldref)|object||
|[spec.vars[].valueFrom.resourceFieldRef.containerName](#specvarsvaluefromresourcefieldrefcontainername)|string||
|[spec.vars[].valueFrom.resourceFieldRef.divisor](#specvarsvaluefromresourcefieldrefdivisor)|string||
|[spec.vars[].valueFrom.resourceFieldRef.resource](#specvarsvaluefromresourcefieldrefresource)|string|✅|
|[spec.vars[].valueFrom.secretKeyRef](#specvarsvaluefromsecretkeyref)|object||
|[spec.vars[].valueFrom.secretKeyRef.key](#specvarsvaluefromsecretkeyrefkey)|string|✅|
|[spec.vars[].valueFrom.secretKeyRef.name](#specvarsvaluefromsecretkeyrefname)|string|✅|
|[spec.vars[].valueFrom.secretKeyRef.optional](#specvarsvaluefromsecretkeyrefoptional)|boolean||
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
|[endpoint](#specendpoint)|object|✅|
|[immutablePayload](#specimmutablepayload)|string|✅|
|[options](#specoptions)|object||
|[payload](#specpayload)|string|✅|
|[vars[]](#specvars)|object||

defines an API request to the Keycloak Admin API.

---

### spec.endpoint

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[instanceRef](#specendpointinstanceref)|string|✅|
|[parent](#specendpointparent)|object||
|[path](#specendpointpath)|string||

*missing*

---

### spec.endpoint.instanceRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the instance to which this object belongs to.

---

### spec.endpoint.parent

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[parent_ref](#specendpointparentparentref)|string|✅|
|[sub_path](#specendpointparentsubpath)|string|✅|

*missing*

---

### spec.endpoint.parent.parent_ref

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.endpoint.parent.sub_path

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.endpoint.path

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.immutablePayload

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.options

Type: object

Options for the request to the Keycloak Admin API.

---

### spec.payload

Type: string

*missing*

---

### spec.vars[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[name](#specvarsname)|string|✅|
|[value](#specvarsvalue)|string||
|[valueFrom](#specvarsvaluefrom)|object||

EnvVar represents an environment variable present in a Container.

---

### spec.vars[].name

Type: string

Name of the environment variable. Must be a C_IDENTIFIER.

---

### spec.vars[].value

Type: string

Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".

---

### spec.vars[].valueFrom

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[configMapKeyRef](#specvarsvaluefromconfigmapkeyref)|object||
|[fieldRef](#specvarsvaluefromfieldref)|object||
|[resourceFieldRef](#specvarsvaluefromresourcefieldref)|object||
|[secretKeyRef](#specvarsvaluefromsecretkeyref)|object||

Source for the environment variable's value. Cannot be used if value is not empty.

---

### spec.vars[].valueFrom.configMapKeyRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[key](#specvarsvaluefromconfigmapkeyrefkey)|string|✅|
|[name](#specvarsvaluefromconfigmapkeyrefname)|string|✅|
|[optional](#specvarsvaluefromconfigmapkeyrefoptional)|boolean||

Selects a key of a ConfigMap.

---

### spec.vars[].valueFrom.configMapKeyRef.key

Type: string

The key to select.

---

### spec.vars[].valueFrom.configMapKeyRef.name

Type: string

Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names

---

### spec.vars[].valueFrom.configMapKeyRef.optional

Type: boolean

Specify whether the ConfigMap or its key must be defined

---

### spec.vars[].valueFrom.fieldRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[apiVersion](#specvarsvaluefromfieldrefapiversion)|string||
|[fieldPath](#specvarsvaluefromfieldreffieldpath)|string|✅|

Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.

---

### spec.vars[].valueFrom.fieldRef.apiVersion

Type: string

Version of the schema the FieldPath is written in terms of, defaults to "v1".

---

### spec.vars[].valueFrom.fieldRef.fieldPath

Type: string

Path of the field to select in the specified API version.

---

### spec.vars[].valueFrom.resourceFieldRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[containerName](#specvarsvaluefromresourcefieldrefcontainername)|string||
|[divisor](#specvarsvaluefromresourcefieldrefdivisor)|string||
|[resource](#specvarsvaluefromresourcefieldrefresource)|string|✅|

Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.

---

### spec.vars[].valueFrom.resourceFieldRef.containerName

Type: string

Container name: required for volumes, optional for env vars

---

### spec.vars[].valueFrom.resourceFieldRef.divisor

Type: string

Specifies the output format of the exposed resources, defaults to "1"

---

### spec.vars[].valueFrom.resourceFieldRef.resource

Type: string

Required: resource to select

---

### spec.vars[].valueFrom.secretKeyRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[key](#specvarsvaluefromsecretkeyrefkey)|string|✅|
|[name](#specvarsvaluefromsecretkeyrefname)|string|✅|
|[optional](#specvarsvaluefromsecretkeyrefoptional)|boolean||

Selects a key of a secret in the pod's namespace

---

### spec.vars[].valueFrom.secretKeyRef.key

Type: string

The key of the secret to select from.  Must be a valid secret key.

---

### spec.vars[].valueFrom.secretKeyRef.name

Type: string

Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names

---

### spec.vars[].valueFrom.secretKeyRef.optional

Type: boolean

Specify whether the Secret or its key must be defined

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