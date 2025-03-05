# KeycloakIdentityProviderMapper

## v1beta1

resource to define a identity provider mapper within a [KeyclaokIdentityProvider](./keycloakidentityprovider.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.config](#specdefinitionconfig)|object||
|[spec.definition.config.attribute.friendly.name](#specdefinitionconfigattributefriendlyname)|string||
|[spec.definition.config.attribute.name.format](#specdefinitionconfigattributenameformat)|string||
|[spec.definition.config.syncMode](#specdefinitionconfigsyncmode)|string||
|[spec.definition.config.user.attribute](#specdefinitionconfiguserattribute)|string||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.identityProviderAlias](#specdefinitionidentityprovideralias)|string||
|[spec.definition.identityProviderMapper](#specdefinitionidentityprovidermapper)|string||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.identityProviderRef](#specidentityproviderref)|string|✅|
|[spec.options](#specoptions)|object||
|[spec.patchFrom](#specpatchfrom)|object||
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
|[definition](#specdefinition)|object|✅|
|[identityProviderRef](#specidentityproviderref)|string|✅|
|[options](#specoptions)|object||
|[patchFrom](#specpatchfrom)|object||

the KeycloakIdentityProviderMapper resource

---

### spec.definition

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionconfig)|object||
|[id](#specdefinitionid)|string||
|[identityProviderAlias](#specdefinitionidentityprovideralias)|string||
|[identityProviderMapper](#specdefinitionidentityprovidermapper)|string||
|[name](#specdefinitionname)|string||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

IdentityProviderMapperRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "properties": { "attribute.friendly.name": { "title": "Friendly name", "description": "Friendly name of attribute to search for in assertion. You can leave this blank and specify a name instead.", "type": "string" }, "attribute.name.format": { "type": "string", "enum": [ "ATTRIBUTE_FORMAT_BASIC", "ATTRIBUTE_FORMAT_URI", "ATTRIBUTE_FORMAT_UNSPECIFIED" ] }, "syncMode": { "title": "Sync mode override", "description": "Overrides the default sync mode of the IDP for this mapper. Values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider and 'inherit' to use the sync mode defined in the identity provider for this mapper.", "type": "string", "enum": [ "INHERIT", "IMPORT", "LEGACY", "FORCE" ] }, "user.attribute": { "title": "User Attribute Name", "description": "Name of user attribute you want to hardcode", "type": "string" } }, "additionalProperties": { "type": "string" } }, "id": { "type": "string" }, "identityProviderAlias": { "type": "string" }, "identityProviderMapper": { "type": "string" }, "name": { "title": "Name", "description": "Name of the mapper.", "type": "string" } }, "additionalProperties": false } ``` </details>

---

### spec.definition.config

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[attribute.friendly.name](#specdefinitionconfigattributefriendlyname)|string||
|[attribute.name.format](#specdefinitionconfigattributenameformat)|string||
|[syncMode](#specdefinitionconfigsyncmode)|string||
|[user.attribute](#specdefinitionconfiguserattribute)|string||

IdentityProviderMapperRepresentationConfig

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "attribute.friendly.name": { "title": "Friendly name", "description": "Friendly name of attribute to search for in assertion. You can leave this blank and specify a name instead.", "type": "string" }, "attribute.name.format": { "type": "string", "enum": [ "ATTRIBUTE_FORMAT_BASIC", "ATTRIBUTE_FORMAT_URI", "ATTRIBUTE_FORMAT_UNSPECIFIED" ] }, "syncMode": { "title": "Sync mode override", "description": "Overrides the default sync mode of the IDP for this mapper. Values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider and 'inherit' to use the sync mode defined in the identity provider for this mapper.", "type": "string", "enum": [ "INHERIT", "IMPORT", "LEGACY", "FORCE" ] }, "user.attribute": { "title": "User Attribute Name", "description": "Name of user attribute you want to hardcode", "type": "string" } }, "additionalProperties": { "type": "string" } } ``` </details>

---

### spec.definition.config.attribute.friendly.name

Type: string

Friendly name of attribute to search for in assertion. You can leave this blank and specify a name instead.

---

### spec.definition.config.attribute.name.format

Type: string

IdentityProviderMapperRepresentationConfigAttributeNameFormat

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "ATTRIBUTE_FORMAT_BASIC", "ATTRIBUTE_FORMAT_URI", "ATTRIBUTE_FORMAT_UNSPECIFIED" ] } ``` </details>

---

### spec.definition.config.syncMode

Type: string

Overrides the default sync mode of the IDP for this mapper. Values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider and 'inherit' to use the sync mode defined in the identity provider for this mapper.

---

### spec.definition.config.user.attribute

Type: string

Name of user attribute you want to hardcode

---

### spec.definition.id

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.definition.identityProviderAlias

Type: string

*missing*

---

### spec.definition.identityProviderMapper

Type: string

*missing*

---

### spec.definition.name

Type: string

Name of the mapper.

---

### spec.identityProviderRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

the name of the kubernetes object that created the identity provider.

---

### spec.options

Type: object

Options for the request to the Keycloak Admin API.

---

### spec.patchFrom

Type: object

Defines additional values that can be loaded from secrets or configmaps. Field selectors are not supported. For more informations see [the patches documentation](../configuration/patches.md).

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