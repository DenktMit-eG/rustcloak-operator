# KeycloakIdentityProviderMapper

## v1

Auto-generated derived type for KeycloakIdentityProviderMapperSpec via `CustomResource`

|Name|Type|Required|
|:---|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.config](#specdefinitionconfig)|object||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.identityProviderAlias](#specdefinitionidentityprovideralias)|string||
|[spec.definition.identityProviderMapper](#specdefinitionidentityprovidermapper)|string||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.identityProviderRef](#specidentityproviderref)|string|✅|
|[spec.options](#specoptions)|object||
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

the KeycloakIdentityProviderMapper resource

### spec.definition

Type: object

#### Validations

|Rule|Error Message|
|:---|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

*missing*

### spec.definition.config

Type: object

*missing*

### spec.definition.id

Type: string

#### Validations

|Rule|Error Message|
|:---|:------------|
|self == oldSelf|Value is immutable|

*missing*

### spec.definition.identityProviderAlias

Type: string

*missing*

### spec.definition.identityProviderMapper

Type: string

*missing*

### spec.definition.name

Type: string

*missing*

### spec.identityProviderRef

Type: string

the name of the kubernetes object that created the identity provider.

### spec.options

Type: object

Options for the request to the Keycloak Admin API.

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