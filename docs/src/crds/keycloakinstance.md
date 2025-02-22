# KeycloakInstance

## v1

This resource makes a Keycloak instance known to the operator

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.baseUrl](#specbaseurl)|string|✅|
|[spec.client](#specclient)|object||
|[spec.client.id](#specclientid)|string|✅|
|[spec.client.secret](#specclientsecret)|string||
|[spec.credentials](#speccredentials)|object|✅|
|[spec.credentials.create](#speccredentialscreate)|boolean||
|[spec.credentials.passwordKey](#speccredentialspasswordkey)|string||
|[spec.credentials.secretName](#speccredentialssecretname)|string|✅|
|[spec.credentials.usernameKey](#speccredentialsusernamekey)|string||
|[spec.realm](#specrealm)|string||
|[spec.token](#spectoken)|object||
|[spec.token.expiresKey](#spectokenexpireskey)|string||
|[spec.token.secretName](#spectokensecretname)|string||
|[spec.token.tokenKey](#spectokentokenkey)|string||
|[status](#status)|object||
|[status.conditions[]](#statusconditions)|object||
|[status.conditions[].lastTransitionTime](#statusconditionslasttransitiontime)|string||
|[status.conditions[].lastUpdateTime](#statusconditionslastupdatetime)|string||
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
|[baseUrl](#specbaseurl)|string|✅|
|[client](#specclient)|object||
|[credentials](#speccredentials)|object|✅|
|[realm](#specrealm)|string||
|[token](#spectoken)|object||

*missing*

---

### spec.baseUrl

Type: string

*missing*

---

### spec.client

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specclientid)|string|✅|
|[secret](#specclientsecret)|string||

*missing*

---

### spec.client.id

Type: string

*missing*

---

### spec.client.secret

Type: string

*missing*

---

### spec.credentials

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[create](#speccredentialscreate)|boolean||
|[passwordKey](#speccredentialspasswordkey)|string||
|[secretName](#speccredentialssecretname)|string|✅|
|[usernameKey](#speccredentialsusernamekey)|string||

*missing*

---

### spec.credentials.create

Type: boolean

*missing*

---

### spec.credentials.passwordKey

Type: string

*missing*

---

### spec.credentials.secretName

Type: string

*missing*

---

### spec.credentials.usernameKey

Type: string

*missing*

---

### spec.realm

Type: string

*missing*

---

### spec.token

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[expiresKey](#spectokenexpireskey)|string||
|[secretName](#spectokensecretname)|string||
|[tokenKey](#spectokentokenkey)|string||

*missing*

---

### spec.token.expiresKey

Type: string

*missing*

---

### spec.token.secretName

Type: string

*missing*

---

### spec.token.tokenKey

Type: string

*missing*

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
|[lastUpdateTime](#statusconditionslastupdatetime)|string||
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

### status.conditions[].lastUpdateTime

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