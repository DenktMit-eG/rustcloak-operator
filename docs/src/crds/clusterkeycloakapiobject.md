# ClusterKeycloakApiObject

## v1beta1

Custom Resource for Keycloak API requests. The user should not use this resource directly.

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.endpoint](#specendpoint)|object|✅|
|[spec.endpoint.clusterInstanceRef](#specendpointclusterinstanceref)|string||
|[spec.endpoint.initWorkflow](#specendpointinitworkflow)|string||
|[spec.endpoint.instanceRef](#specendpointinstanceref)|string||
|[spec.endpoint.parent](#specendpointparent)|object||
|[spec.endpoint.parent.clusterParentRef](#specendpointparentclusterparentref)|string||
|[spec.endpoint.parent.parentRef](#specendpointparentparentref)|string||
|[spec.endpoint.parent.sub_path](#specendpointparentsubpath)|string|✅|
|[spec.endpoint.path](#specendpointpath)|string||
|[spec.endpoint.realm](#specendpointrealm)|object||
|[spec.endpoint.realm.clusterRealmRef](#specendpointrealmclusterrealmref)|string||
|[spec.endpoint.realm.realmRef](#specendpointrealmrealmref)|string||
|[spec.immutablePayload](#specimmutablepayload)|string|✅|
|[spec.options](#specoptions)|object||
|[spec.options.patchFrom[]](#specoptionspatchfrom)|object||
|[spec.options.patchFrom[].configMapKeyRef](#specoptionspatchfromconfigmapkeyref)|object||
|[spec.options.patchFrom[].configMapKeyRef.key](#specoptionspatchfromconfigmapkeyrefkey)|string|✅|
|[spec.options.patchFrom[].configMapKeyRef.name](#specoptionspatchfromconfigmapkeyrefname)|string|✅|
|[spec.options.patchFrom[].configMapKeyRef.optional](#specoptionspatchfromconfigmapkeyrefoptional)|boolean||
|[spec.options.patchFrom[].path](#specoptionspatchfrompath)|string|✅|
|[spec.options.patchFrom[].secretKeyRef](#specoptionspatchfromsecretkeyref)|object||
|[spec.options.patchFrom[].secretKeyRef.key](#specoptionspatchfromsecretkeyrefkey)|string|✅|
|[spec.options.patchFrom[].secretKeyRef.name](#specoptionspatchfromsecretkeyrefname)|string|✅|
|[spec.options.patchFrom[].secretKeyRef.optional](#specoptionspatchfromsecretkeyrefoptional)|boolean||
|[spec.options.patchFrom[].value](#specoptionspatchfromvalue)|string||
|[spec.options.patchFrom[].value_as](#specoptionspatchfromvalueas)|string||
|[spec.payload](#specpayload)|string|✅|
|[status](#status)|object||
|[status.apiObjectRef](#statusapiobjectref)|string||
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
|[endpoint](#specendpoint)|object|✅|
|[immutablePayload](#specimmutablepayload)|string|✅|
|[options](#specoptions)|object||
|[payload](#specpayload)|string|✅|

defines an API request to the Keycloak Admin API.

---

### spec.endpoint

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clusterInstanceRef](#specendpointclusterinstanceref)|string||
|[initWorkflow](#specendpointinitworkflow)|string||
|[instanceRef](#specendpointinstanceref)|string||
|[parent](#specendpointparent)|object||
|[path](#specendpointpath)|string||
|[realm](#specendpointrealm)|object||

*missing*

---

### spec.endpoint.clusterInstanceRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the cluster instance to which this object belongs to.

---

### spec.endpoint.initWorkflow

Type: string

*missing*

---

### spec.endpoint.instanceRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the namespaced instance to which this object belongs to.

---

### spec.endpoint.parent

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clusterParentRef](#specendpointparentclusterparentref)|string||
|[parentRef](#specendpointparentparentref)|string||
|[sub_path](#specendpointparentsubpath)|string|✅|

*missing*

---

### spec.endpoint.parent.clusterParentRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the cluster API Object to which this object belongs to.

---

### spec.endpoint.parent.parentRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the API Object to which this object belongs to.

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

### spec.endpoint.realm

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clusterRealmRef](#specendpointrealmclusterrealmref)|string||
|[realmRef](#specendpointrealmrealmref)|string||

Optional for backwards compatibility

---

### spec.endpoint.realm.clusterRealmRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the cluster realm to which this object belongs to

---

### spec.endpoint.realm.realmRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the realm to which this object belongs to

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

|Property|Type|Required|
|:-------|:---|:------:|
|[patchFrom[]](#specoptionspatchfrom)|object||

*missing*

---

### spec.options.patchFrom[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[configMapKeyRef](#specoptionspatchfromconfigmapkeyref)|object||
|[path](#specoptionspatchfrompath)|string|✅|
|[secretKeyRef](#specoptionspatchfromsecretkeyref)|object||
|[value](#specoptionspatchfromvalue)|string||
|[value_as](#specoptionspatchfromvalueas)|string||

*missing*

---

### spec.options.patchFrom[].configMapKeyRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[key](#specoptionspatchfromconfigmapkeyrefkey)|string|✅|
|[name](#specoptionspatchfromconfigmapkeyrefname)|string|✅|
|[optional](#specoptionspatchfromconfigmapkeyrefoptional)|boolean||

Selects a key from a ConfigMap.

---

### spec.options.patchFrom[].configMapKeyRef.key

Type: string

The key to select.

---

### spec.options.patchFrom[].configMapKeyRef.name

Type: string

Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names

---

### spec.options.patchFrom[].configMapKeyRef.optional

Type: boolean

Specify whether the ConfigMap or its key must be defined

---

### spec.options.patchFrom[].path

Type: string

*missing*

---

### spec.options.patchFrom[].secretKeyRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[key](#specoptionspatchfromsecretkeyrefkey)|string|✅|
|[name](#specoptionspatchfromsecretkeyrefname)|string|✅|
|[optional](#specoptionspatchfromsecretkeyrefoptional)|boolean||

SecretKeySelector selects a key of a Secret.

---

### spec.options.patchFrom[].secretKeyRef.key

Type: string

The key of the secret to select from.  Must be a valid secret key.

---

### spec.options.patchFrom[].secretKeyRef.name

Type: string

Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names

---

### spec.options.patchFrom[].secretKeyRef.optional

Type: boolean

Specify whether the Secret or its key must be defined

---

### spec.options.patchFrom[].value

Type: string

*missing*

---

### spec.options.patchFrom[].value_as

Type: string

*missing*

---

### spec.payload

Type: string

*missing*

---

### status

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[apiObjectRef](#statusapiobjectref)|string||
|[conditions[]](#statusconditions)|object||
|[instance](#statusinstance)|object||
|[message](#statusmessage)|string||
|[ready](#statusready)|boolean|✅|
|[realm](#statusrealm)|object||
|[resourcePath](#statusresourcepath)|string||
|[status](#statusstatus)|string||

*missing*

---

### status.apiObjectRef

Type: string

Reference to the API object name (KeycloakApiObject or ClusterKeycloakApiObject)

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