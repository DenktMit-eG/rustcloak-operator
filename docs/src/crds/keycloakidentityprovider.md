# KeycloakIdentityProvider

## v1

Auto-generated derived type for KeycloakIdentityProviderSpec via `CustomResource`

|Name|Type|Required|
|:---|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.addReadTokenRoleOnCreate](#specdefinitionaddreadtokenroleoncreate)|boolean||
|[spec.definition.alias](#specdefinitionalias)|string||
|[spec.definition.authenticateByDefault](#specdefinitionauthenticatebydefault)|boolean||
|[spec.definition.config](#specdefinitionconfig)|object||
|[spec.definition.displayName](#specdefinitiondisplayname)|string||
|[spec.definition.enabled](#specdefinitionenabled)|boolean||
|[spec.definition.firstBrokerLoginFlowAlias](#specdefinitionfirstbrokerloginflowalias)|string||
|[spec.definition.hideOnLogin](#specdefinitionhideonlogin)|boolean||
|[spec.definition.internalId](#specdefinitioninternalid)|string||
|[spec.definition.linkOnly](#specdefinitionlinkonly)|boolean||
|[spec.definition.organizationId](#specdefinitionorganizationid)|string||
|[spec.definition.postBrokerLoginFlowAlias](#specdefinitionpostbrokerloginflowalias)|string||
|[spec.definition.providerId](#specdefinitionproviderid)|string||
|[spec.definition.storeToken](#specdefinitionstoretoken)|boolean||
|[spec.definition.trustEmail](#specdefinitiontrustemail)|boolean||
|[spec.definition.updateProfileFirstLogin](#specdefinitionupdateprofilefirstlogin)|boolean||
|[spec.definition.updateProfileFirstLoginMode](#specdefinitionupdateprofilefirstloginmode)|string||
|[spec.options](#specoptions)|object||
|[spec.realmRef](#specrealmref)|string|✅|
|[status](#status)|object||
|[status.conditions](#statusconditions)|array||
|[status.conditions[]](#statusconditions)|object||
|[status.conditions[].lastTransitionTime](#statusconditionslasttransitiontime)|string||
|[status.conditions[].lastUpdateTime](#statusconditionslastupdatetime)|string||
|[status.conditions[].message](#statusconditionsmessage)|string||
|[status.conditions[].reason](#statusconditionsreason)|string||
|[status.conditions[].status](#statusconditionsstatus)|string|✅|
|[status.conditions[].type](#statusconditionstype)|string|✅|
|[status.conditions[].lastTransitionTime](#statusconditionslasttransitiontime)|string||
|[status.conditions[].lastUpdateTime](#statusconditionslastupdatetime)|string||
|[status.conditions[].message](#statusconditionsmessage)|string||
|[status.conditions[].reason](#statusconditionsreason)|string||
|[status.conditions[].status](#statusconditionsstatus)|string|✅|
|[status.conditions[].type](#statusconditionstype)|string|✅|
|[status.message](#statusmessage)|string||
|[status.ready](#statusready)|boolean|✅|
|[status.resourcePath](#statusresourcepath)|string||
|[status.status](#statusstatus)|string||

### spec

Type: object

the KeycloakIdentityProvider resource

### spec.definition

Type: object

#### Validations

|Rule|Error Message|
|:---|:------------|
|has(self.alias) == has(oldSelf.alias)|Value is immutable|

*missing*

### spec.definition.addReadTokenRoleOnCreate

Type: boolean

*missing*

### spec.definition.alias

Type: string

#### Validations

|Rule|Error Message|
|:---|:------------|
|self == oldSelf|Value is immutable|

*missing*

### spec.definition.authenticateByDefault

Type: boolean

*missing*

### spec.definition.config

Type: object

*missing*

### spec.definition.displayName

Type: string

*missing*

### spec.definition.enabled

Type: boolean

*missing*

### spec.definition.firstBrokerLoginFlowAlias

Type: string

*missing*

### spec.definition.hideOnLogin

Type: boolean

*missing*

### spec.definition.internalId

Type: string

*missing*

### spec.definition.linkOnly

Type: boolean

*missing*

### spec.definition.organizationId

Type: string

*missing*

### spec.definition.postBrokerLoginFlowAlias

Type: string

*missing*

### spec.definition.providerId

Type: string

*missing*

### spec.definition.storeToken

Type: boolean

*missing*

### spec.definition.trustEmail

Type: boolean

*missing*

### spec.definition.updateProfileFirstLogin

Type: boolean

*missing*

### spec.definition.updateProfileFirstLoginMode

Type: string

*missing*

### spec.options

Type: object

Options for the request to the Keycloak Admin API.

### spec.realmRef

Type: string

the name of the kubernetes object that created the realm.

### status

Type: object

*missing*

### status.conditions

Type: array

*missing*

### status.conditions[]

Type: object

*missing*

### status.conditions[].lastTransitionTime

Type: string

Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.

### status.conditions[].lastUpdateTime

Type: string

Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.

### status.conditions[].message

Type: string

*missing*

### status.conditions[].reason

Type: string

*missing*

### status.conditions[].status

Type: string

*missing*

### status.conditions[].type

Type: string

*missing*

### status.conditions[].lastTransitionTime

Type: string

Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.

### status.conditions[].lastUpdateTime

Type: string

Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.

### status.conditions[].message

Type: string

*missing*

### status.conditions[].reason

Type: string

*missing*

### status.conditions[].status

Type: string

*missing*

### status.conditions[].type

Type: string

*missing*

### status.message

Type: string

*missing*

### status.ready

Type: boolean

*missing*

### status.resourcePath

Type: string

*missing*

### status.status

Type: string

*missing*