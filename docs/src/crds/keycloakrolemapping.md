# KeycloakRoleMapping

## v1beta1

represents a mapping between a user or group and a client

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.role](#specrole)|object||
|[spec.role.clientId](#specroleclientid)|string||
|[spec.role.clientRef](#specroleclientref)|string||
|[spec.role.name](#specrolename)|string|✅|
|[spec.roleRef](#specroleref)|string||
|[spec.subject](#specsubject)|object|✅|
|[spec.subject.groupRef](#specsubjectgroupref)|string||
|[spec.subject.userRef](#specsubjectuserref)|string||
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
|[role](#specrole)|object||
|[roleRef](#specroleref)|string||
|[subject](#specsubject)|object|✅|

*missing*

---

### spec.role

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clientId](#specroleclientid)|string||
|[clientRef](#specroleclientref)|string||
|[name](#specrolename)|string|✅|

The name of the role in keycloak. Mutual exclusive with roleRef

---

### spec.role.clientId

Type: string

The client id of the the client.

---

### spec.role.clientRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The kubernetes resources name of a KeycloakClient object.

---

### spec.role.name

Type: string

The name of the role in keycloak

---

### spec.roleRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The kubernetes resource name of a KeycloakRole object. Mutual exclusive with role

---

### spec.subject

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[groupRef](#specsubjectgroupref)|string||
|[userRef](#specsubjectuserref)|string||

The object that :the role mapping is for

---

### spec.subject.groupRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of a KeycloakGroup resource

---

### spec.subject.userRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of a KeycloakUser resource

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