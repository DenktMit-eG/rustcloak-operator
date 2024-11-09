# KeycloakRole

## v1

Auto-generated derived type for KeycloakRoleSpec via `CustomResource`

|Name|Type|Required|
|:---|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clientRef](#specclientref)|string||
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.clientRole](#specdefinitionclientrole)|boolean||
|[spec.definition.composite](#specdefinitioncomposite)|boolean||
|[spec.definition.composites](#specdefinitioncomposites)|object||
|[spec.definition.composites.application](#specdefinitioncompositesapplication)|object||
|[spec.definition.composites.client](#specdefinitioncompositesclient)|object||
|[spec.definition.composites.realm](#specdefinitioncompositesrealm)|array||
|[spec.definition.composites.realm[]](#specdefinitioncompositesrealm)|string||
|[spec.definition.containerId](#specdefinitioncontainerid)|string||
|[spec.definition.description](#specdefinitiondescription)|string||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.definition.scopeParamRequired](#specdefinitionscopeparamrequired)|boolean||
|[spec.options](#specoptions)|object||
|[spec.realmRef](#specrealmref)|string||
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

the KeycloakRole resource

### spec.clientRef

Type: string

*missing*

### spec.definition

Type: object

#### Validations

|Rule|Error Message|
|:---|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

*missing*

### spec.definition.attributes

Type: object

*missing*

### spec.definition.clientRole

Type: boolean

*missing*

### spec.definition.composite

Type: boolean

*missing*

### spec.definition.composites

Type: object

*missing*

### spec.definition.composites.application

Type: object

*missing*

### spec.definition.composites.client

Type: object

*missing*

### spec.definition.composites.realm

Type: array

*missing*

### spec.definition.composites.realm[]

Type: string

*missing*

### spec.definition.containerId

Type: string

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

### spec.definition.name

Type: string

*missing*

### spec.definition.scopeParamRequired

Type: boolean

*missing*

### spec.options

Type: object

Options for the request to the Keycloak Admin API.

### spec.realmRef

Type: string

*missing*

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