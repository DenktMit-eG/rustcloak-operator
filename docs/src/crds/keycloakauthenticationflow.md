# KeycloakAuthenticationFlow

## v1

Auto-generated derived type for KeycloakAuthenticationFlowSpec via `CustomResource`

|Name|Type|Required|
|:---|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.alias](#specdefinitionalias)|string||
|[spec.definition.authenticationExecutions](#specdefinitionauthenticationexecutions)|array||
|[spec.definition.authenticationExecutions[]](#specdefinitionauthenticationexecutions)|object||
|[spec.definition.authenticationExecutions[].authenticator](#specdefinitionauthenticationexecutionsauthenticator)|string||
|[spec.definition.authenticationExecutions[].authenticatorConfig](#specdefinitionauthenticationexecutionsauthenticatorconfig)|string||
|[spec.definition.authenticationExecutions[].authenticatorFlow](#specdefinitionauthenticationexecutionsauthenticatorflow)|boolean||
|[spec.definition.authenticationExecutions[].autheticatorFlow](#specdefinitionauthenticationexecutionsautheticatorflow)|boolean||
|[spec.definition.authenticationExecutions[].flowAlias](#specdefinitionauthenticationexecutionsflowalias)|string||
|[spec.definition.authenticationExecutions[].priority](#specdefinitionauthenticationexecutionspriority)|integer||
|[spec.definition.authenticationExecutions[].requirement](#specdefinitionauthenticationexecutionsrequirement)|string||
|[spec.definition.authenticationExecutions[].userSetupAllowed](#specdefinitionauthenticationexecutionsusersetupallowed)|boolean||
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

the KeycloakAuthenticationFlow resource

### spec.definition

Type: object

#### Validations

|Rule|Error Message|
|:---|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

*missing*

### spec.definition.alias

Type: string

*missing*

### spec.definition.authenticationExecutions

Type: array

*missing*

### spec.definition.authenticationExecutions[]

Type: object

*missing*

### spec.definition.authenticationExecutions[].authenticator

Type: string

*missing*

### spec.definition.authenticationExecutions[].authenticatorConfig

Type: string

*missing*

### spec.definition.authenticationExecutions[].authenticatorFlow

Type: boolean

*missing*

### spec.definition.authenticationExecutions[].autheticatorFlow

Type: boolean

*missing*

### spec.definition.authenticationExecutions[].flowAlias

Type: string

*missing*

### spec.definition.authenticationExecutions[].priority

Type: integer

*missing*

### spec.definition.authenticationExecutions[].requirement

Type: string

*missing*

### spec.definition.authenticationExecutions[].userSetupAllowed

Type: boolean

*missing*

### spec.definition.authenticationExecutions[].authenticator

Type: string

*missing*

### spec.definition.authenticationExecutions[].authenticatorConfig

Type: string

*missing*

### spec.definition.authenticationExecutions[].authenticatorFlow

Type: boolean

*missing*

### spec.definition.authenticationExecutions[].autheticatorFlow

Type: boolean

*missing*

### spec.definition.authenticationExecutions[].flowAlias

Type: string

*missing*

### spec.definition.authenticationExecutions[].priority

Type: integer

*missing*

### spec.definition.authenticationExecutions[].requirement

Type: string

*missing*

### spec.definition.authenticationExecutions[].userSetupAllowed

Type: boolean

*missing*

### spec.definition.builtIn

Type: boolean

*missing*

### spec.definition.description

Type: string

*missing*

### spec.definition.id

Type: string

#### Validations

|Rule|Error Message|
|:---|:------------|
|self == oldSelf|Value is immutable|

*missing*

### spec.definition.providerId

Type: string

*missing*

### spec.definition.topLevel

Type: boolean

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