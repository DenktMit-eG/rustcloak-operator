# KeycloakUser

## v1beta1

resource to define a User within a [KeyclaokRealm](./keycloakrealm.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clientRef](#specclientref)|string||
|[spec.clusterRealmRef](#specclusterrealmref)|string||
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.access](#specdefinitionaccess)|object||
|[spec.definition.applicationRoles](#specdefinitionapplicationroles)|object||
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.clientConsents[]](#specdefinitionclientconsents)|object||
|[spec.definition.clientConsents[].clientId](#specdefinitionclientconsentsclientid)|string||
|[spec.definition.clientConsents[].createdDate](#specdefinitionclientconsentscreateddate)|integer||
|[spec.definition.clientConsents[].grantedClientScopes[]](#specdefinitionclientconsentsgrantedclientscopes)|string||
|[spec.definition.clientConsents[].grantedRealmRoles[]](#specdefinitionclientconsentsgrantedrealmroles)|string||
|[spec.definition.clientConsents[].lastUpdatedDate](#specdefinitionclientconsentslastupdateddate)|integer||
|[spec.definition.clientRoles](#specdefinitionclientroles)|object||
|[spec.definition.createdTimestamp](#specdefinitioncreatedtimestamp)|integer||
|[spec.definition.credentials[]](#specdefinitioncredentials)|object||
|[spec.definition.credentials[].algorithm](#specdefinitioncredentialsalgorithm)|string||
|[spec.definition.credentials[].config](#specdefinitioncredentialsconfig)|object||
|[spec.definition.credentials[].counter](#specdefinitioncredentialscounter)|integer||
|[spec.definition.credentials[].createdDate](#specdefinitioncredentialscreateddate)|integer||
|[spec.definition.credentials[].credentialData](#specdefinitioncredentialscredentialdata)|string||
|[spec.definition.credentials[].device](#specdefinitioncredentialsdevice)|string||
|[spec.definition.credentials[].digits](#specdefinitioncredentialsdigits)|integer||
|[spec.definition.credentials[].hashIterations](#specdefinitioncredentialshashiterations)|integer||
|[spec.definition.credentials[].hashedSaltedValue](#specdefinitioncredentialshashedsaltedvalue)|string||
|[spec.definition.credentials[].id](#specdefinitioncredentialsid)|string||
|[spec.definition.credentials[].period](#specdefinitioncredentialsperiod)|integer||
|[spec.definition.credentials[].priority](#specdefinitioncredentialspriority)|integer||
|[spec.definition.credentials[].salt](#specdefinitioncredentialssalt)|string||
|[spec.definition.credentials[].secretData](#specdefinitioncredentialssecretdata)|string||
|[spec.definition.credentials[].temporary](#specdefinitioncredentialstemporary)|boolean||
|[spec.definition.credentials[].type](#specdefinitioncredentialstype)|string||
|[spec.definition.credentials[].userLabel](#specdefinitioncredentialsuserlabel)|string||
|[spec.definition.credentials[].value](#specdefinitioncredentialsvalue)|string||
|[spec.definition.disableableCredentialTypes[]](#specdefinitiondisableablecredentialtypes)|string||
|[spec.definition.email](#specdefinitionemail)|string||
|[spec.definition.emailVerified](#specdefinitionemailverified)|boolean||
|[spec.definition.enabled](#specdefinitionenabled)|boolean||
|[spec.definition.federatedIdentities[]](#specdefinitionfederatedidentities)|object||
|[spec.definition.federatedIdentities[].identityProvider](#specdefinitionfederatedidentitiesidentityprovider)|string||
|[spec.definition.federatedIdentities[].userId](#specdefinitionfederatedidentitiesuserid)|string||
|[spec.definition.federatedIdentities[].userName](#specdefinitionfederatedidentitiesusername)|string||
|[spec.definition.federationLink](#specdefinitionfederationlink)|string||
|[spec.definition.firstName](#specdefinitionfirstname)|string||
|[spec.definition.groups[]](#specdefinitiongroups)|string||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.lastName](#specdefinitionlastname)|string||
|[spec.definition.notBefore](#specdefinitionnotbefore)|integer||
|[spec.definition.origin](#specdefinitionorigin)|string||
|[spec.definition.realmRoles[]](#specdefinitionrealmroles)|string||
|[spec.definition.requiredActions[]](#specdefinitionrequiredactions)|string||
|[spec.definition.self](#specdefinitionself)|string||
|[spec.definition.serviceAccountClientId](#specdefinitionserviceaccountclientid)|string||
|[spec.definition.socialLinks[]](#specdefinitionsociallinks)|object||
|[spec.definition.socialLinks[].socialProvider](#specdefinitionsociallinkssocialprovider)|string||
|[spec.definition.socialLinks[].socialUserId](#specdefinitionsociallinkssocialuserid)|string||
|[spec.definition.socialLinks[].socialUsername](#specdefinitionsociallinkssocialusername)|string||
|[spec.definition.totp](#specdefinitiontotp)|boolean||
|[spec.definition.userProfileMetadata](#specdefinitionuserprofilemetadata)|object||
|[spec.definition.userProfileMetadata.attributes[]](#specdefinitionuserprofilemetadataattributes)|object||
|[spec.definition.userProfileMetadata.attributes[].annotations](#specdefinitionuserprofilemetadataattributesannotations)|object||
|[spec.definition.userProfileMetadata.attributes[].displayName](#specdefinitionuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.userProfileMetadata.attributes[].group](#specdefinitionuserprofilemetadataattributesgroup)|string||
|[spec.definition.userProfileMetadata.attributes[].multivalued](#specdefinitionuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.userProfileMetadata.attributes[].name](#specdefinitionuserprofilemetadataattributesname)|string||
|[spec.definition.userProfileMetadata.attributes[].readOnly](#specdefinitionuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.userProfileMetadata.attributes[].required](#specdefinitionuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.userProfileMetadata.attributes[].validators](#specdefinitionuserprofilemetadataattributesvalidators)|object||
|[spec.definition.userProfileMetadata.groups[]](#specdefinitionuserprofilemetadatagroups)|object||
|[spec.definition.userProfileMetadata.groups[].annotations](#specdefinitionuserprofilemetadatagroupsannotations)|object||
|[spec.definition.userProfileMetadata.groups[].displayDescription](#specdefinitionuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.userProfileMetadata.groups[].displayHeader](#specdefinitionuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.userProfileMetadata.groups[].name](#specdefinitionuserprofilemetadatagroupsname)|string||
|[spec.definition.username](#specdefinitionusername)|string||
|[spec.options](#specoptions)|object||
|[spec.patchFrom](#specpatchfrom)|object||
|[spec.patchFrom2[]](#specpatchfrom2)|object||
|[spec.patchFrom2[].configMapKeyRef](#specpatchfrom2configmapkeyref)|object||
|[spec.patchFrom2[].configMapKeyRef.key](#specpatchfrom2configmapkeyrefkey)|string|✅|
|[spec.patchFrom2[].configMapKeyRef.name](#specpatchfrom2configmapkeyrefname)|string|✅|
|[spec.patchFrom2[].configMapKeyRef.optional](#specpatchfrom2configmapkeyrefoptional)|boolean||
|[spec.patchFrom2[].fieldRef](#specpatchfrom2fieldref)|object||
|[spec.patchFrom2[].fieldRef.apiVersion](#specpatchfrom2fieldrefapiversion)|string||
|[spec.patchFrom2[].fieldRef.fieldPath](#specpatchfrom2fieldreffieldpath)|string|✅|
|[spec.patchFrom2[].path](#specpatchfrom2path)|string|✅|
|[spec.patchFrom2[].resourceFieldRef](#specpatchfrom2resourcefieldref)|object||
|[spec.patchFrom2[].resourceFieldRef.containerName](#specpatchfrom2resourcefieldrefcontainername)|string||
|[spec.patchFrom2[].resourceFieldRef.divisor](#specpatchfrom2resourcefieldrefdivisor)|string||
|[spec.patchFrom2[].resourceFieldRef.resource](#specpatchfrom2resourcefieldrefresource)|string|✅|
|[spec.patchFrom2[].secretKeyRef](#specpatchfrom2secretkeyref)|object||
|[spec.patchFrom2[].secretKeyRef.key](#specpatchfrom2secretkeyrefkey)|string|✅|
|[spec.patchFrom2[].secretKeyRef.name](#specpatchfrom2secretkeyrefname)|string|✅|
|[spec.patchFrom2[].secretKeyRef.optional](#specpatchfrom2secretkeyrefoptional)|boolean||
|[spec.patchFrom2[].valueAs](#specpatchfrom2valueas)|string||
|[spec.realmRef](#specrealmref)|string||
|[spec.userSecret](#specusersecret)|object||
|[spec.userSecret.emailKey](#specusersecretemailkey)|string||
|[spec.userSecret.passwordKey](#specusersecretpasswordkey)|string||
|[spec.userSecret.secretName](#specusersecretsecretname)|string|✅|
|[spec.userSecret.usernameKey](#specusersecretusernamekey)|string||
|[status](#status)|object||
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
|[status.resourcePath](#statusresourcepath)|string||
|[status.status](#statusstatus)|string||

---

### spec

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clientRef](#specclientref)|string||
|[clusterRealmRef](#specclusterrealmref)|string||
|[definition](#specdefinition)|object|✅|
|[options](#specoptions)|object||
|[patchFrom](#specpatchfrom)|object||
|[patchFrom2[]](#specpatchfrom2)|object||
|[realmRef](#specrealmref)|string||
|[userSecret](#specusersecret)|object||

the KeycloakUser resource

---

### spec.clientRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.clusterRealmRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the cluster realm to which this object belongs to

---

### spec.definition

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[access](#specdefinitionaccess)|object||
|[applicationRoles](#specdefinitionapplicationroles)|object||
|[attributes](#specdefinitionattributes)|object||
|[clientConsents[]](#specdefinitionclientconsents)|object||
|[clientRoles](#specdefinitionclientroles)|object||
|[createdTimestamp](#specdefinitioncreatedtimestamp)|integer||
|[credentials[]](#specdefinitioncredentials)|object||
|[disableableCredentialTypes[]](#specdefinitiondisableablecredentialtypes)|string||
|[email](#specdefinitionemail)|string||
|[emailVerified](#specdefinitionemailverified)|boolean||
|[enabled](#specdefinitionenabled)|boolean||
|[federatedIdentities[]](#specdefinitionfederatedidentities)|object||
|[federationLink](#specdefinitionfederationlink)|string||
|[firstName](#specdefinitionfirstname)|string||
|[groups[]](#specdefinitiongroups)|string||
|[id](#specdefinitionid)|string||
|[lastName](#specdefinitionlastname)|string||
|[notBefore](#specdefinitionnotbefore)|integer||
|[origin](#specdefinitionorigin)|string||
|[realmRoles[]](#specdefinitionrealmroles)|string||
|[requiredActions[]](#specdefinitionrequiredactions)|string||
|[self](#specdefinitionself)|string||
|[serviceAccountClientId](#specdefinitionserviceaccountclientid)|string||
|[socialLinks[]](#specdefinitionsociallinks)|object||
|[totp](#specdefinitiontotp)|boolean||
|[userProfileMetadata](#specdefinitionuserprofilemetadata)|object||
|[username](#specdefinitionusername)|string||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

UserRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "access": { "type": "object", "additionalProperties": { "type": "boolean" } }, "applicationRoles": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "clientConsents": { "type": "array", "items": { "$ref": "#/$defs/UserConsentRepresentation" } }, "clientRoles": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "createdTimestamp": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "credentials": { "type": "array", "items": { "$ref": "#/$defs/CredentialRepresentation" } }, "disableableCredentialTypes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "email": { "type": "string" }, "emailVerified": { "type": "boolean" }, "enabled": { "type": "boolean" }, "federatedIdentities": { "type": "array", "items": { "$ref": "#/$defs/FederatedIdentityRepresentation" } }, "federationLink": { "type": "string" }, "firstName": { "type": "string" }, "groups": { "type": "array", "items": { "type": "string" } }, "id": { "type": "string" }, "lastName": { "type": "string" }, "notBefore": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "origin": { "type": "string" }, "realmRoles": { "type": "array", "items": { "type": "string" } }, "requiredActions": { "type": "array", "items": { "type": "string" } }, "self": { "type": "string" }, "serviceAccountClientId": { "type": "string" }, "socialLinks": { "type": "array", "items": { "$ref": "#/$defs/SocialLinkRepresentation" } }, "totp": { "type": "boolean" }, "userProfileMetadata": { "$ref": "#/$defs/UserProfileMetadata" }, "username": { "type": "string" } } } ``` </details>

---

### spec.definition.access

Type: object

*missing*

---

### spec.definition.applicationRoles

Type: object

*missing*

---

### spec.definition.attributes

Type: object

*missing*

---

### spec.definition.clientConsents[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clientId](#specdefinitionclientconsentsclientid)|string||
|[createdDate](#specdefinitionclientconsentscreateddate)|integer||
|[grantedClientScopes[]](#specdefinitionclientconsentsgrantedclientscopes)|string||
|[grantedRealmRoles[]](#specdefinitionclientconsentsgrantedrealmroles)|string||
|[lastUpdatedDate](#specdefinitionclientconsentslastupdateddate)|integer||

UserConsentRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "clientId": { "type": "string" }, "createdDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "grantedClientScopes": { "type": "array", "items": { "type": "string" } }, "grantedRealmRoles": { "type": "array", "items": { "type": "string" } }, "lastUpdatedDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 } } } ``` </details>

---

### spec.definition.clientConsents[].clientId

Type: string

*missing*

---

### spec.definition.clientConsents[].createdDate

Type: integer

*missing*

---

### spec.definition.clientConsents[].grantedClientScopes[]

Type: string

*missing*

---

### spec.definition.clientConsents[].grantedRealmRoles[]

Type: string

*missing*

---

### spec.definition.clientConsents[].lastUpdatedDate

Type: integer

*missing*

---

### spec.definition.clientRoles

Type: object

*missing*

---

### spec.definition.createdTimestamp

Type: integer

*missing*

---

### spec.definition.credentials[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[algorithm](#specdefinitioncredentialsalgorithm)|string||
|[config](#specdefinitioncredentialsconfig)|object||
|[counter](#specdefinitioncredentialscounter)|integer||
|[createdDate](#specdefinitioncredentialscreateddate)|integer||
|[credentialData](#specdefinitioncredentialscredentialdata)|string||
|[device](#specdefinitioncredentialsdevice)|string||
|[digits](#specdefinitioncredentialsdigits)|integer||
|[hashIterations](#specdefinitioncredentialshashiterations)|integer||
|[hashedSaltedValue](#specdefinitioncredentialshashedsaltedvalue)|string||
|[id](#specdefinitioncredentialsid)|string||
|[period](#specdefinitioncredentialsperiod)|integer||
|[priority](#specdefinitioncredentialspriority)|integer||
|[salt](#specdefinitioncredentialssalt)|string||
|[secretData](#specdefinitioncredentialssecretdata)|string||
|[temporary](#specdefinitioncredentialstemporary)|boolean||
|[type](#specdefinitioncredentialstype)|string||
|[userLabel](#specdefinitioncredentialsuserlabel)|string||
|[value](#specdefinitioncredentialsvalue)|string||

CredentialRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "algorithm": { "type": "string" }, "config": { "type": "object", "allOf": [ { "$ref": "#/$defs/MultivaluedHashMapStringString" } ] }, "counter": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "createdDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "credentialData": { "type": "string" }, "device": { "type": "string" }, "digits": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "hashIterations": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "hashedSaltedValue": { "type": "string" }, "id": { "type": "string" }, "period": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "priority": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "salt": { "type": "string" }, "secretData": { "type": "string" }, "temporary": { "type": "boolean" }, "type": { "type": "string" }, "userLabel": { "type": "string" }, "value": { "type": "string" } } } ``` </details>

---

### spec.definition.credentials[].algorithm

Type: string

*missing*

---

### spec.definition.credentials[].config

Type: object

*missing*

---

### spec.definition.credentials[].counter

Type: integer

*missing*

---

### spec.definition.credentials[].createdDate

Type: integer

*missing*

---

### spec.definition.credentials[].credentialData

Type: string

*missing*

---

### spec.definition.credentials[].device

Type: string

*missing*

---

### spec.definition.credentials[].digits

Type: integer

*missing*

---

### spec.definition.credentials[].hashIterations

Type: integer

*missing*

---

### spec.definition.credentials[].hashedSaltedValue

Type: string

*missing*

---

### spec.definition.credentials[].id

Type: string

*missing*

---

### spec.definition.credentials[].period

Type: integer

*missing*

---

### spec.definition.credentials[].priority

Type: integer

*missing*

---

### spec.definition.credentials[].salt

Type: string

*missing*

---

### spec.definition.credentials[].secretData

Type: string

*missing*

---

### spec.definition.credentials[].temporary

Type: boolean

*missing*

---

### spec.definition.credentials[].type

Type: string

*missing*

---

### spec.definition.credentials[].userLabel

Type: string

*missing*

---

### spec.definition.credentials[].value

Type: string

*missing*

---

### spec.definition.disableableCredentialTypes[]

Type: string

*missing*

---

### spec.definition.email

Type: string

*missing*

---

### spec.definition.emailVerified

Type: boolean

*missing*

---

### spec.definition.enabled

Type: boolean

*missing*

---

### spec.definition.federatedIdentities[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[identityProvider](#specdefinitionfederatedidentitiesidentityprovider)|string||
|[userId](#specdefinitionfederatedidentitiesuserid)|string||
|[userName](#specdefinitionfederatedidentitiesusername)|string||

FederatedIdentityRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "identityProvider": { "type": "string" }, "userId": { "type": "string" }, "userName": { "type": "string" } } } ``` </details>

---

### spec.definition.federatedIdentities[].identityProvider

Type: string

*missing*

---

### spec.definition.federatedIdentities[].userId

Type: string

*missing*

---

### spec.definition.federatedIdentities[].userName

Type: string

*missing*

---

### spec.definition.federationLink

Type: string

*missing*

---

### spec.definition.firstName

Type: string

*missing*

---

### spec.definition.groups[]

Type: string

*missing*

---

### spec.definition.id

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.definition.lastName

Type: string

*missing*

---

### spec.definition.notBefore

Type: integer

*missing*

---

### spec.definition.origin

Type: string

*missing*

---

### spec.definition.realmRoles[]

Type: string

*missing*

---

### spec.definition.requiredActions[]

Type: string

*missing*

---

### spec.definition.self

Type: string

*missing*

---

### spec.definition.serviceAccountClientId

Type: string

*missing*

---

### spec.definition.socialLinks[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[socialProvider](#specdefinitionsociallinkssocialprovider)|string||
|[socialUserId](#specdefinitionsociallinkssocialuserid)|string||
|[socialUsername](#specdefinitionsociallinkssocialusername)|string||

SocialLinkRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "socialProvider": { "type": "string" }, "socialUserId": { "type": "string" }, "socialUsername": { "type": "string" } } } ``` </details>

---

### spec.definition.socialLinks[].socialProvider

Type: string

*missing*

---

### spec.definition.socialLinks[].socialUserId

Type: string

*missing*

---

### spec.definition.socialLinks[].socialUsername

Type: string

*missing*

---

### spec.definition.totp

Type: boolean

*missing*

---

### spec.definition.userProfileMetadata

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[attributes[]](#specdefinitionuserprofilemetadataattributes)|object||
|[groups[]](#specdefinitionuserprofilemetadatagroups)|object||

UserProfileMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "attributes": { "type": "array", "items": { "$ref": "#/$defs/UserProfileAttributeMetadata" } }, "groups": { "type": "array", "items": { "$ref": "#/$defs/UserProfileAttributeGroupMetadata" } } } } ``` </details>

---

### spec.definition.userProfileMetadata.attributes[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[annotations](#specdefinitionuserprofilemetadataattributesannotations)|object||
|[displayName](#specdefinitionuserprofilemetadataattributesdisplayname)|string||
|[group](#specdefinitionuserprofilemetadataattributesgroup)|string||
|[multivalued](#specdefinitionuserprofilemetadataattributesmultivalued)|boolean||
|[name](#specdefinitionuserprofilemetadataattributesname)|string||
|[readOnly](#specdefinitionuserprofilemetadataattributesreadonly)|boolean||
|[required](#specdefinitionuserprofilemetadataattributesrequired)|boolean||
|[validators](#specdefinitionuserprofilemetadataattributesvalidators)|object||

UserProfileAttributeMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "annotations": { "type": "object", "additionalProperties": {} }, "displayName": { "type": "string" }, "group": { "type": "string" }, "multivalued": { "type": "boolean" }, "name": { "type": "string" }, "readOnly": { "type": "boolean" }, "required": { "type": "boolean" }, "validators": { "type": "object", "additionalProperties": { "type": "object", "additionalProperties": {} } } } } ``` </details>

---

### spec.definition.userProfileMetadata.attributes[].annotations

Type: object

*missing*

---

### spec.definition.userProfileMetadata.attributes[].displayName

Type: string

*missing*

---

### spec.definition.userProfileMetadata.attributes[].group

Type: string

*missing*

---

### spec.definition.userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

---

### spec.definition.userProfileMetadata.attributes[].name

Type: string

*missing*

---

### spec.definition.userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

---

### spec.definition.userProfileMetadata.attributes[].required

Type: boolean

*missing*

---

### spec.definition.userProfileMetadata.attributes[].validators

Type: object

*missing*

---

### spec.definition.userProfileMetadata.groups[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[annotations](#specdefinitionuserprofilemetadatagroupsannotations)|object||
|[displayDescription](#specdefinitionuserprofilemetadatagroupsdisplaydescription)|string||
|[displayHeader](#specdefinitionuserprofilemetadatagroupsdisplayheader)|string||
|[name](#specdefinitionuserprofilemetadatagroupsname)|string||

UserProfileAttributeGroupMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "annotations": { "type": "object", "additionalProperties": {} }, "displayDescription": { "type": "string" }, "displayHeader": { "type": "string" }, "name": { "type": "string" } } } ``` </details>

---

### spec.definition.userProfileMetadata.groups[].annotations

Type: object

*missing*

---

### spec.definition.userProfileMetadata.groups[].displayDescription

Type: string

*missing*

---

### spec.definition.userProfileMetadata.groups[].displayHeader

Type: string

*missing*

---

### spec.definition.userProfileMetadata.groups[].name

Type: string

*missing*

---

### spec.definition.username

Type: string

*missing*

---

### spec.options

Type: object

Options for the request to the Keycloak Admin API.

---

### spec.patchFrom

Type: object

Defines additional values that can be loaded from secrets or configmaps. Field selectors are not supported. For more informations see [the patches documentation](../configuration/patches.md).

---

### spec.patchFrom2[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[configMapKeyRef](#specpatchfrom2configmapkeyref)|object||
|[fieldRef](#specpatchfrom2fieldref)|object||
|[path](#specpatchfrom2path)|string|✅|
|[resourceFieldRef](#specpatchfrom2resourcefieldref)|object||
|[secretKeyRef](#specpatchfrom2secretkeyref)|object||
|[valueAs](#specpatchfrom2valueas)|string||

EnvVarSource represents a source for the value of an EnvVar.

---

### spec.patchFrom2[].configMapKeyRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[key](#specpatchfrom2configmapkeyrefkey)|string|✅|
|[name](#specpatchfrom2configmapkeyrefname)|string|✅|
|[optional](#specpatchfrom2configmapkeyrefoptional)|boolean||

Selects a key of a ConfigMap.

---

### spec.patchFrom2[].configMapKeyRef.key

Type: string

The key to select.

---

### spec.patchFrom2[].configMapKeyRef.name

Type: string

Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names

---

### spec.patchFrom2[].configMapKeyRef.optional

Type: boolean

Specify whether the ConfigMap or its key must be defined

---

### spec.patchFrom2[].fieldRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[apiVersion](#specpatchfrom2fieldrefapiversion)|string||
|[fieldPath](#specpatchfrom2fieldreffieldpath)|string|✅|

Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.

---

### spec.patchFrom2[].fieldRef.apiVersion

Type: string

Version of the schema the FieldPath is written in terms of, defaults to "v1".

---

### spec.patchFrom2[].fieldRef.fieldPath

Type: string

Path of the field to select in the specified API version.

---

### spec.patchFrom2[].path

Type: string

*missing*

---

### spec.patchFrom2[].resourceFieldRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[containerName](#specpatchfrom2resourcefieldrefcontainername)|string||
|[divisor](#specpatchfrom2resourcefieldrefdivisor)|string||
|[resource](#specpatchfrom2resourcefieldrefresource)|string|✅|

Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.

---

### spec.patchFrom2[].resourceFieldRef.containerName

Type: string

Container name: required for volumes, optional for env vars

---

### spec.patchFrom2[].resourceFieldRef.divisor

Type: string

Specifies the output format of the exposed resources, defaults to "1"

---

### spec.patchFrom2[].resourceFieldRef.resource

Type: string

Required: resource to select

---

### spec.patchFrom2[].secretKeyRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[key](#specpatchfrom2secretkeyrefkey)|string|✅|
|[name](#specpatchfrom2secretkeyrefname)|string|✅|
|[optional](#specpatchfrom2secretkeyrefoptional)|boolean||

Selects a key of a secret in the pod's namespace

---

### spec.patchFrom2[].secretKeyRef.key

Type: string

The key of the secret to select from.  Must be a valid secret key.

---

### spec.patchFrom2[].secretKeyRef.name

Type: string

Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names

---

### spec.patchFrom2[].secretKeyRef.optional

Type: boolean

Specify whether the Secret or its key must be defined

---

### spec.patchFrom2[].valueAs

Type: string

*missing*

---

### spec.realmRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the realm to which this object belongs to

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
|[conditions[]](#statusconditions)|object||
|[instance](#statusinstance)|object||
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

### status.resourcePath

Type: string

*missing*

---

### status.status

Type: string

*missing*