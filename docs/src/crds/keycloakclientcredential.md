# KeycloakClientCredential

## v1beta1

represents credentials for a keycloak client

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clientSecret](#specclientsecret)|object|✅|
|[spec.clientSecret.clientIdKey](#specclientsecretclientidkey)|string||
|[spec.clientSecret.clientSecretKey](#specclientsecretclientsecretkey)|string||
|[spec.clientSecret.secretName](#specclientsecretsecretname)|string|✅|
|[spec.clusterInstanceRef](#specclusterinstanceref)|string||
|[spec.instanceRef](#specinstanceref)|string||
|[spec.resourcePath](#specresourcepath)|string|✅|
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
|[clientSecret](#specclientsecret)|object|✅|
|[clusterInstanceRef](#specclusterinstanceref)|string||
|[instanceRef](#specinstanceref)|string||
|[resourcePath](#specresourcepath)|string|✅|

*missing*

---

### spec.clientSecret

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clientIdKey](#specclientsecretclientidkey)|string||
|[clientSecretKey](#specclientsecretclientsecretkey)|string||
|[secretName](#specclientsecretsecretname)|string|✅|

*missing*

---

### spec.clientSecret.clientIdKey

Type: string

*missing*

---

### spec.clientSecret.clientSecretKey

Type: string

*missing*

---

### spec.clientSecret.secretName

Type: string

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