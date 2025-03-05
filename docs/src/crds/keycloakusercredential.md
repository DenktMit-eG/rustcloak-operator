# KeycloakUserCredential

## v1beta1

represents credentials for a keycloak user

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clusterInstanceRef](#specclusterinstanceref)|string||
|[spec.instanceRef](#specinstanceref)|string||
|[spec.resourcePath](#specresourcepath)|string|✅|
|[spec.userSecret](#specusersecret)|object|✅|
|[spec.userSecret.emailKey](#specusersecretemailkey)|string||
|[spec.userSecret.passwordKey](#specusersecretpasswordkey)|string||
|[spec.userSecret.secretName](#specusersecretsecretname)|string|✅|
|[spec.userSecret.usernameKey](#specusersecretusernamekey)|string||
|[status](#status)|object||
|[status.clusterInstanceRef](#statusclusterinstanceref)|string||
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
|[clusterInstanceRef](#specclusterinstanceref)|string||
|[instanceRef](#specinstanceref)|string||
|[resourcePath](#specresourcepath)|string|✅|
|[userSecret](#specusersecret)|object|✅|

*missing*

---

### spec.clusterInstanceRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the cluster instance to which this object belongs to.

---

### spec.instanceRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.resourcePath

Type: string

*missing*

---

### spec.userSecret

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[emailKey](#specusersecretemailkey)|string||
|[passwordKey](#specusersecretpasswordkey)|string||
|[secretName](#specusersecretsecretname)|string|✅|
|[usernameKey](#specusersecretusernamekey)|string||

*missing*

---

### spec.userSecret.emailKey

Type: string

*missing*

---

### spec.userSecret.passwordKey

Type: string

*missing*

---

### spec.userSecret.secretName

Type: string

*missing*

---

### spec.userSecret.usernameKey

Type: string

*missing*

---

### status

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clusterInstanceRef](#statusclusterinstanceref)|string||
|[conditions[]](#statusconditions)|object||
|[instanceRef](#statusinstanceref)|string||
|[message](#statusmessage)|string||
|[ready](#statusready)|boolean|✅|
|[resourcePath](#statusresourcepath)|string||
|[status](#statusstatus)|string||

*missing*

---

### status.clusterInstanceRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the cluster instance to which this object belongs to.

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