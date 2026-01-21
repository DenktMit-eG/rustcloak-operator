# ClusterKeycloakInstance

## v1beta1

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
|[baseUrl](#specbaseurl)|string|✅|
|[client](#specclient)|object||
|[credentials](#speccredentials)|object|✅|
|[realm](#specrealm)|string||
|[token](#spectoken)|object||

*missing*

---

### spec.baseUrl

Type: string

Base URL of the Keycloak server (e.g., "https://keycloak.example.com").

---

### spec.client

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specclientid)|string|✅|
|[secret](#specclientsecret)|string||

Optional client credentials for service account authentication instead of username/password.

---

### spec.client.id

Type: string

Client ID for service account authentication.

---

### spec.client.secret

Type: string

Client secret for service account authentication. Optional if the client is public.

---

### spec.credentials

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[create](#speccredentialscreate)|boolean||
|[passwordKey](#speccredentialspasswordkey)|string||
|[secretName](#speccredentialssecretname)|string|✅|
|[usernameKey](#speccredentialsusernamekey)|string||

Reference to the Kubernetes Secret containing admin credentials.

---

### spec.credentials.create

Type: boolean

If true, the operator will create the secret with a randomly generated password if it doesn't exist.

---

### spec.credentials.passwordKey

Type: string

Key in the secret containing the password. Defaults to "password".

---

### spec.credentials.secretName

Type: string

Name of the Kubernetes Secret containing the credentials.

---

### spec.credentials.usernameKey

Type: string

Key in the secret containing the username. Defaults to "user".

---

### spec.realm

Type: string

Realm used for authentication. Defaults to "master".

---

### spec.token

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[expiresKey](#spectokenexpireskey)|string||
|[secretName](#spectokensecretname)|string||
|[tokenKey](#spectokentokenkey)|string||

Optional configuration for caching the authentication token in a Secret.

---

### spec.token.expiresKey

Type: string

Key in the secret for storing the token expiration timestamp. Defaults to "expires".

---

### spec.token.secretName

Type: string

Name of the Kubernetes Secret for caching the authentication token. If not specified, defaults to "{instance-name}-api-token".

---

### spec.token.tokenKey

Type: string

Key in the secret for storing the token. Defaults to "token".

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