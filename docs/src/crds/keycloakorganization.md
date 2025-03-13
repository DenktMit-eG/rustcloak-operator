# KeycloakOrganization

## v1beta1

resource to define an Organisation within a [KeyclaokRealm](./keycloakrealm.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clusterRealmRef](#specclusterrealmref)|string||
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.alias](#specdefinitionalias)|string||
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.description](#specdefinitiondescription)|string||
|[spec.definition.domains[]](#specdefinitiondomains)|object||
|[spec.definition.domains[].name](#specdefinitiondomainsname)|string||
|[spec.definition.domains[].verified](#specdefinitiondomainsverified)|boolean||
|[spec.definition.enabled](#specdefinitionenabled)|boolean||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.identityProviders[]](#specdefinitionidentityproviders)|object||
|[spec.definition.identityProviders[].addReadTokenRoleOnCreate](#specdefinitionidentityprovidersaddreadtokenroleoncreate)|boolean||
|[spec.definition.identityProviders[].alias](#specdefinitionidentityprovidersalias)|string||
|[spec.definition.identityProviders[].authenticateByDefault](#specdefinitionidentityprovidersauthenticatebydefault)|boolean||
|[spec.definition.identityProviders[].config](#specdefinitionidentityprovidersconfig)|object||
|[spec.definition.identityProviders[].displayName](#specdefinitionidentityprovidersdisplayname)|string||
|[spec.definition.identityProviders[].enabled](#specdefinitionidentityprovidersenabled)|boolean||
|[spec.definition.identityProviders[].firstBrokerLoginFlowAlias](#specdefinitionidentityprovidersfirstbrokerloginflowalias)|string||
|[spec.definition.identityProviders[].hideOnLogin](#specdefinitionidentityprovidershideonlogin)|boolean||
|[spec.definition.identityProviders[].internalId](#specdefinitionidentityprovidersinternalid)|string||
|[spec.definition.identityProviders[].linkOnly](#specdefinitionidentityproviderslinkonly)|boolean||
|[spec.definition.identityProviders[].organizationId](#specdefinitionidentityprovidersorganizationid)|string||
|[spec.definition.identityProviders[].postBrokerLoginFlowAlias](#specdefinitionidentityproviderspostbrokerloginflowalias)|string||
|[spec.definition.identityProviders[].providerId](#specdefinitionidentityprovidersproviderid)|string||
|[spec.definition.identityProviders[].storeToken](#specdefinitionidentityprovidersstoretoken)|boolean||
|[spec.definition.identityProviders[].trustEmail](#specdefinitionidentityproviderstrustemail)|boolean||
|[spec.definition.identityProviders[].updateProfileFirstLogin](#specdefinitionidentityprovidersupdateprofilefirstlogin)|boolean||
|[spec.definition.identityProviders[].updateProfileFirstLoginMode](#specdefinitionidentityprovidersupdateprofilefirstloginmode)|string||
|[spec.definition.members[]](#specdefinitionmembers)|object||
|[spec.definition.members[].access](#specdefinitionmembersaccess)|object||
|[spec.definition.members[].applicationRoles](#specdefinitionmembersapplicationroles)|object||
|[spec.definition.members[].attributes](#specdefinitionmembersattributes)|object||
|[spec.definition.members[].clientConsents[]](#specdefinitionmembersclientconsents)|object||
|[spec.definition.members[].clientConsents[].clientId](#specdefinitionmembersclientconsentsclientid)|string||
|[spec.definition.members[].clientConsents[].createdDate](#specdefinitionmembersclientconsentscreateddate)|integer||
|[spec.definition.members[].clientConsents[].grantedClientScopes[]](#specdefinitionmembersclientconsentsgrantedclientscopes)|string||
|[spec.definition.members[].clientConsents[].grantedRealmRoles[]](#specdefinitionmembersclientconsentsgrantedrealmroles)|string||
|[spec.definition.members[].clientConsents[].lastUpdatedDate](#specdefinitionmembersclientconsentslastupdateddate)|integer||
|[spec.definition.members[].clientRoles](#specdefinitionmembersclientroles)|object||
|[spec.definition.members[].createdTimestamp](#specdefinitionmemberscreatedtimestamp)|integer||
|[spec.definition.members[].credentials[]](#specdefinitionmemberscredentials)|object||
|[spec.definition.members[].credentials[].algorithm](#specdefinitionmemberscredentialsalgorithm)|string||
|[spec.definition.members[].credentials[].config](#specdefinitionmemberscredentialsconfig)|object||
|[spec.definition.members[].credentials[].counter](#specdefinitionmemberscredentialscounter)|integer||
|[spec.definition.members[].credentials[].createdDate](#specdefinitionmemberscredentialscreateddate)|integer||
|[spec.definition.members[].credentials[].credentialData](#specdefinitionmemberscredentialscredentialdata)|string||
|[spec.definition.members[].credentials[].device](#specdefinitionmemberscredentialsdevice)|string||
|[spec.definition.members[].credentials[].digits](#specdefinitionmemberscredentialsdigits)|integer||
|[spec.definition.members[].credentials[].hashIterations](#specdefinitionmemberscredentialshashiterations)|integer||
|[spec.definition.members[].credentials[].hashedSaltedValue](#specdefinitionmemberscredentialshashedsaltedvalue)|string||
|[spec.definition.members[].credentials[].id](#specdefinitionmemberscredentialsid)|string||
|[spec.definition.members[].credentials[].period](#specdefinitionmemberscredentialsperiod)|integer||
|[spec.definition.members[].credentials[].priority](#specdefinitionmemberscredentialspriority)|integer||
|[spec.definition.members[].credentials[].salt](#specdefinitionmemberscredentialssalt)|string||
|[spec.definition.members[].credentials[].secretData](#specdefinitionmemberscredentialssecretdata)|string||
|[spec.definition.members[].credentials[].temporary](#specdefinitionmemberscredentialstemporary)|boolean||
|[spec.definition.members[].credentials[].type](#specdefinitionmemberscredentialstype)|string||
|[spec.definition.members[].credentials[].userLabel](#specdefinitionmemberscredentialsuserlabel)|string||
|[spec.definition.members[].credentials[].value](#specdefinitionmemberscredentialsvalue)|string||
|[spec.definition.members[].disableableCredentialTypes[]](#specdefinitionmembersdisableablecredentialtypes)|string||
|[spec.definition.members[].email](#specdefinitionmembersemail)|string||
|[spec.definition.members[].emailVerified](#specdefinitionmembersemailverified)|boolean||
|[spec.definition.members[].enabled](#specdefinitionmembersenabled)|boolean||
|[spec.definition.members[].federatedIdentities[]](#specdefinitionmembersfederatedidentities)|object||
|[spec.definition.members[].federatedIdentities[].identityProvider](#specdefinitionmembersfederatedidentitiesidentityprovider)|string||
|[spec.definition.members[].federatedIdentities[].userId](#specdefinitionmembersfederatedidentitiesuserid)|string||
|[spec.definition.members[].federatedIdentities[].userName](#specdefinitionmembersfederatedidentitiesusername)|string||
|[spec.definition.members[].federationLink](#specdefinitionmembersfederationlink)|string||
|[spec.definition.members[].firstName](#specdefinitionmembersfirstname)|string||
|[spec.definition.members[].groups[]](#specdefinitionmembersgroups)|string||
|[spec.definition.members[].id](#specdefinitionmembersid)|string||
|[spec.definition.members[].lastName](#specdefinitionmemberslastname)|string||
|[spec.definition.members[].membershipType](#specdefinitionmembersmembershiptype)|string||
|[spec.definition.members[].notBefore](#specdefinitionmembersnotbefore)|integer||
|[spec.definition.members[].origin](#specdefinitionmembersorigin)|string||
|[spec.definition.members[].realmRoles[]](#specdefinitionmembersrealmroles)|string||
|[spec.definition.members[].requiredActions[]](#specdefinitionmembersrequiredactions)|string||
|[spec.definition.members[].self](#specdefinitionmembersself)|string||
|[spec.definition.members[].serviceAccountClientId](#specdefinitionmembersserviceaccountclientid)|string||
|[spec.definition.members[].socialLinks[]](#specdefinitionmemberssociallinks)|object||
|[spec.definition.members[].socialLinks[].socialProvider](#specdefinitionmemberssociallinkssocialprovider)|string||
|[spec.definition.members[].socialLinks[].socialUserId](#specdefinitionmemberssociallinkssocialuserid)|string||
|[spec.definition.members[].socialLinks[].socialUsername](#specdefinitionmemberssociallinkssocialusername)|string||
|[spec.definition.members[].totp](#specdefinitionmemberstotp)|boolean||
|[spec.definition.members[].userProfileMetadata](#specdefinitionmembersuserprofilemetadata)|object||
|[spec.definition.members[].userProfileMetadata.attributes[]](#specdefinitionmembersuserprofilemetadataattributes)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].annotations](#specdefinitionmembersuserprofilemetadataattributesannotations)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].displayName](#specdefinitionmembersuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].group](#specdefinitionmembersuserprofilemetadataattributesgroup)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].multivalued](#specdefinitionmembersuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].name](#specdefinitionmembersuserprofilemetadataattributesname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].readOnly](#specdefinitionmembersuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].required](#specdefinitionmembersuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].validators](#specdefinitionmembersuserprofilemetadataattributesvalidators)|object||
|[spec.definition.members[].userProfileMetadata.groups[]](#specdefinitionmembersuserprofilemetadatagroups)|object||
|[spec.definition.members[].userProfileMetadata.groups[].annotations](#specdefinitionmembersuserprofilemetadatagroupsannotations)|object||
|[spec.definition.members[].userProfileMetadata.groups[].displayDescription](#specdefinitionmembersuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.members[].userProfileMetadata.groups[].displayHeader](#specdefinitionmembersuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.members[].userProfileMetadata.groups[].name](#specdefinitionmembersuserprofilemetadatagroupsname)|string||
|[spec.definition.members[].username](#specdefinitionmembersusername)|string||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.definition.redirectUrl](#specdefinitionredirecturl)|string||
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
|[status.reconcileAttempts](#statusreconcileattempts)|integer||
|[status.resourcePath](#statusresourcepath)|string||
|[status.status](#statusstatus)|string||

---

### spec

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clusterRealmRef](#specclusterrealmref)|string||
|[definition](#specdefinition)|object|✅|
|[options](#specoptions)|object||
|[patchFrom](#specpatchfrom)|object||
|[patchFrom2[]](#specpatchfrom2)|object||
|[realmRef](#specrealmref)|string||

the KeycloakOrganization resource

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
|[alias](#specdefinitionalias)|string||
|[attributes](#specdefinitionattributes)|object||
|[description](#specdefinitiondescription)|string||
|[domains[]](#specdefinitiondomains)|object||
|[enabled](#specdefinitionenabled)|boolean||
|[id](#specdefinitionid)|string||
|[identityProviders[]](#specdefinitionidentityproviders)|object||
|[members[]](#specdefinitionmembers)|object||
|[name](#specdefinitionname)|string||
|[redirectUrl](#specdefinitionredirecturl)|string||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

OrganizationRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "alias": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "description": { "type": "string" }, "domains": { "type": "array", "items": { "$ref": "#/$defs/OrganizationDomainRepresentation" }, "uniqueItems": true }, "enabled": { "type": "boolean" }, "id": { "type": "string" }, "identityProviders": { "type": "array", "items": { "$ref": "#/$defs/IdentityProviderRepresentation" } }, "members": { "type": "array", "items": { "$ref": "#/$defs/MemberRepresentation" } }, "name": { "type": "string" }, "redirectUrl": { "type": "string" } } } ``` </details>

---

### spec.definition.alias

Type: string

*missing*

---

### spec.definition.attributes

Type: object

*missing*

---

### spec.definition.description

Type: string

*missing*

---

### spec.definition.domains[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[name](#specdefinitiondomainsname)|string||
|[verified](#specdefinitiondomainsverified)|boolean||

OrganizationDomainRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "name": { "type": "string" }, "verified": { "type": "boolean" } } } ``` </details>

---

### spec.definition.domains[].name

Type: string

*missing*

---

### spec.definition.domains[].verified

Type: boolean

*missing*

---

### spec.definition.enabled

Type: boolean

*missing*

---

### spec.definition.id

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.definition.identityProviders[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[addReadTokenRoleOnCreate](#specdefinitionidentityprovidersaddreadtokenroleoncreate)|boolean||
|[alias](#specdefinitionidentityprovidersalias)|string||
|[authenticateByDefault](#specdefinitionidentityprovidersauthenticatebydefault)|boolean||
|[config](#specdefinitionidentityprovidersconfig)|object||
|[displayName](#specdefinitionidentityprovidersdisplayname)|string||
|[enabled](#specdefinitionidentityprovidersenabled)|boolean||
|[firstBrokerLoginFlowAlias](#specdefinitionidentityprovidersfirstbrokerloginflowalias)|string||
|[hideOnLogin](#specdefinitionidentityprovidershideonlogin)|boolean||
|[internalId](#specdefinitionidentityprovidersinternalid)|string||
|[linkOnly](#specdefinitionidentityproviderslinkonly)|boolean||
|[organizationId](#specdefinitionidentityprovidersorganizationid)|string||
|[postBrokerLoginFlowAlias](#specdefinitionidentityproviderspostbrokerloginflowalias)|string||
|[providerId](#specdefinitionidentityprovidersproviderid)|string||
|[storeToken](#specdefinitionidentityprovidersstoretoken)|boolean||
|[trustEmail](#specdefinitionidentityproviderstrustemail)|boolean||
|[updateProfileFirstLogin](#specdefinitionidentityprovidersupdateprofilefirstlogin)|boolean||
|[updateProfileFirstLoginMode](#specdefinitionidentityprovidersupdateprofilefirstloginmode)|string||

IdentityProviderRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "addReadTokenRoleOnCreate": { "title": "Stored tokens readable", "description": "Enable/disable if new users can read any stored tokens. This assigns the broker.read-token role.", "type": "boolean" }, "alias": { "title": "Alias", "description": "The alias uniquely identifies an identity provider and it is also used to build the redirect uri.", "type": "string" }, "authenticateByDefault": { "type": "boolean" }, "config": { "type": "object", "additionalProperties": { "type": "string" } }, "displayName": { "title": "Display name", "description": "Friendly name for Identity Providers.", "type": "string" }, "enabled": { "title": "Enabled", "type": "boolean" }, "firstBrokerLoginFlowAlias": { "title": "First login flow override", "description": "Alias of authentication flow, which is triggered after first login with this identity provider. Term 'First Login' means that no Keycloak account is currently linked to the authenticated identity provider account.", "type": "string" }, "hideOnLogin": { "title": "Hide on login page", "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.", "type": "boolean" }, "internalId": { "type": "string" }, "linkOnly": { "title": "Account linking only", "description": "If true, users cannot log in through this provider.  They can only link to this provider.  This is useful if you don't want to allow login from the provider, but want to integrate with a provider.", "type": "boolean" }, "organizationId": { "type": "string" }, "postBrokerLoginFlowAlias": { "title": "Post login flow", "description": "Alias of authentication flow, which is triggered after each login with this identity provider. Useful if you want additional verification of each user authenticated with this identity provider (for example OTP). Leave this to \"None\" if you need no any additional authenticators to be triggered after login with this identity provider. Also note that authenticator implementations must assume that user is already set in ClientSession as identity provider already set it.", "type": "string" }, "providerId": { "type": "string" }, "storeToken": { "title": "Store tokens", "description": "Enable/disable if tokens must be stored after authenticating users.", "type": "boolean" }, "trustEmail": { "title": "Trust Email", "description": "If enabled, email provided by this provider is not verified even if verification is enabled for the realm.", "type": "boolean" }, "updateProfileFirstLogin": { "type": "boolean" }, "updateProfileFirstLoginMode": { "type": "string" } } } ``` </details>

---

### spec.definition.identityProviders[].addReadTokenRoleOnCreate

Type: boolean

Enable/disable if new users can read any stored tokens. This assigns the broker.read-token role.

---

### spec.definition.identityProviders[].alias

Type: string

The alias uniquely identifies an identity provider and it is also used to build the redirect uri.

---

### spec.definition.identityProviders[].authenticateByDefault

Type: boolean

*missing*

---

### spec.definition.identityProviders[].config

Type: object

*missing*

---

### spec.definition.identityProviders[].displayName

Type: string

Friendly name for Identity Providers.

---

### spec.definition.identityProviders[].enabled

Type: boolean

*missing*

---

### spec.definition.identityProviders[].firstBrokerLoginFlowAlias

Type: string

Alias of authentication flow, which is triggered after first login with this identity provider. Term 'First Login' means that no Keycloak account is currently linked to the authenticated identity provider account.

---

### spec.definition.identityProviders[].hideOnLogin

Type: boolean

If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.

---

### spec.definition.identityProviders[].internalId

Type: string

*missing*

---

### spec.definition.identityProviders[].linkOnly

Type: boolean

If true, users cannot log in through this provider.  They can only link to this provider.  This is useful if you don't want to allow login from the provider, but want to integrate with a provider.

---

### spec.definition.identityProviders[].organizationId

Type: string

*missing*

---

### spec.definition.identityProviders[].postBrokerLoginFlowAlias

Type: string

Alias of authentication flow, which is triggered after each login with this identity provider. Useful if you want additional verification of each user authenticated with this identity provider (for example OTP). Leave this to "None" if you need no any additional authenticators to be triggered after login with this identity provider. Also note that authenticator implementations must assume that user is already set in ClientSession as identity provider already set it.

---

### spec.definition.identityProviders[].providerId

Type: string

*missing*

---

### spec.definition.identityProviders[].storeToken

Type: boolean

Enable/disable if tokens must be stored after authenticating users.

---

### spec.definition.identityProviders[].trustEmail

Type: boolean

If enabled, email provided by this provider is not verified even if verification is enabled for the realm.

---

### spec.definition.identityProviders[].updateProfileFirstLogin

Type: boolean

*missing*

---

### spec.definition.identityProviders[].updateProfileFirstLoginMode

Type: string

*missing*

---

### spec.definition.members[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[access](#specdefinitionmembersaccess)|object||
|[applicationRoles](#specdefinitionmembersapplicationroles)|object||
|[attributes](#specdefinitionmembersattributes)|object||
|[clientConsents[]](#specdefinitionmembersclientconsents)|object||
|[clientRoles](#specdefinitionmembersclientroles)|object||
|[createdTimestamp](#specdefinitionmemberscreatedtimestamp)|integer||
|[credentials[]](#specdefinitionmemberscredentials)|object||
|[disableableCredentialTypes[]](#specdefinitionmembersdisableablecredentialtypes)|string||
|[email](#specdefinitionmembersemail)|string||
|[emailVerified](#specdefinitionmembersemailverified)|boolean||
|[enabled](#specdefinitionmembersenabled)|boolean||
|[federatedIdentities[]](#specdefinitionmembersfederatedidentities)|object||
|[federationLink](#specdefinitionmembersfederationlink)|string||
|[firstName](#specdefinitionmembersfirstname)|string||
|[groups[]](#specdefinitionmembersgroups)|string||
|[id](#specdefinitionmembersid)|string||
|[lastName](#specdefinitionmemberslastname)|string||
|[membershipType](#specdefinitionmembersmembershiptype)|string||
|[notBefore](#specdefinitionmembersnotbefore)|integer||
|[origin](#specdefinitionmembersorigin)|string||
|[realmRoles[]](#specdefinitionmembersrealmroles)|string||
|[requiredActions[]](#specdefinitionmembersrequiredactions)|string||
|[self](#specdefinitionmembersself)|string||
|[serviceAccountClientId](#specdefinitionmembersserviceaccountclientid)|string||
|[socialLinks[]](#specdefinitionmemberssociallinks)|object||
|[totp](#specdefinitionmemberstotp)|boolean||
|[userProfileMetadata](#specdefinitionmembersuserprofilemetadata)|object||
|[username](#specdefinitionmembersusername)|string||

MemberRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "access": { "type": "object", "additionalProperties": { "type": "boolean" } }, "applicationRoles": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "clientConsents": { "type": "array", "items": { "$ref": "#/$defs/UserConsentRepresentation" } }, "clientRoles": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "createdTimestamp": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "credentials": { "type": "array", "items": { "$ref": "#/$defs/CredentialRepresentation" } }, "disableableCredentialTypes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "email": { "type": "string" }, "emailVerified": { "type": "boolean" }, "enabled": { "type": "boolean" }, "federatedIdentities": { "type": "array", "items": { "$ref": "#/$defs/FederatedIdentityRepresentation" } }, "federationLink": { "type": "string" }, "firstName": { "type": "string" }, "groups": { "type": "array", "items": { "type": "string" } }, "id": { "type": "string" }, "lastName": { "type": "string" }, "membershipType": { "$ref": "#/$defs/MembershipType" }, "notBefore": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "origin": { "type": "string" }, "realmRoles": { "type": "array", "items": { "type": "string" } }, "requiredActions": { "type": "array", "items": { "type": "string" } }, "self": { "type": "string" }, "serviceAccountClientId": { "type": "string" }, "socialLinks": { "type": "array", "items": { "$ref": "#/$defs/SocialLinkRepresentation" } }, "totp": { "type": "boolean" }, "userProfileMetadata": { "$ref": "#/$defs/UserProfileMetadata" }, "username": { "type": "string" } } } ``` </details>

---

### spec.definition.members[].access

Type: object

*missing*

---

### spec.definition.members[].applicationRoles

Type: object

*missing*

---

### spec.definition.members[].attributes

Type: object

*missing*

---

### spec.definition.members[].clientConsents[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clientId](#specdefinitionmembersclientconsentsclientid)|string||
|[createdDate](#specdefinitionmembersclientconsentscreateddate)|integer||
|[grantedClientScopes[]](#specdefinitionmembersclientconsentsgrantedclientscopes)|string||
|[grantedRealmRoles[]](#specdefinitionmembersclientconsentsgrantedrealmroles)|string||
|[lastUpdatedDate](#specdefinitionmembersclientconsentslastupdateddate)|integer||

UserConsentRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "clientId": { "type": "string" }, "createdDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "grantedClientScopes": { "type": "array", "items": { "type": "string" } }, "grantedRealmRoles": { "type": "array", "items": { "type": "string" } }, "lastUpdatedDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 } } } ``` </details>

---

### spec.definition.members[].clientConsents[].clientId

Type: string

*missing*

---

### spec.definition.members[].clientConsents[].createdDate

Type: integer

*missing*

---

### spec.definition.members[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

---

### spec.definition.members[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

---

### spec.definition.members[].clientConsents[].lastUpdatedDate

Type: integer

*missing*

---

### spec.definition.members[].clientRoles

Type: object

*missing*

---

### spec.definition.members[].createdTimestamp

Type: integer

*missing*

---

### spec.definition.members[].credentials[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[algorithm](#specdefinitionmemberscredentialsalgorithm)|string||
|[config](#specdefinitionmemberscredentialsconfig)|object||
|[counter](#specdefinitionmemberscredentialscounter)|integer||
|[createdDate](#specdefinitionmemberscredentialscreateddate)|integer||
|[credentialData](#specdefinitionmemberscredentialscredentialdata)|string||
|[device](#specdefinitionmemberscredentialsdevice)|string||
|[digits](#specdefinitionmemberscredentialsdigits)|integer||
|[hashIterations](#specdefinitionmemberscredentialshashiterations)|integer||
|[hashedSaltedValue](#specdefinitionmemberscredentialshashedsaltedvalue)|string||
|[id](#specdefinitionmemberscredentialsid)|string||
|[period](#specdefinitionmemberscredentialsperiod)|integer||
|[priority](#specdefinitionmemberscredentialspriority)|integer||
|[salt](#specdefinitionmemberscredentialssalt)|string||
|[secretData](#specdefinitionmemberscredentialssecretdata)|string||
|[temporary](#specdefinitionmemberscredentialstemporary)|boolean||
|[type](#specdefinitionmemberscredentialstype)|string||
|[userLabel](#specdefinitionmemberscredentialsuserlabel)|string||
|[value](#specdefinitionmemberscredentialsvalue)|string||

CredentialRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "algorithm": { "type": "string" }, "config": { "type": "object", "allOf": [ { "$ref": "#/$defs/MultivaluedHashMapStringString" } ] }, "counter": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "createdDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "credentialData": { "type": "string" }, "device": { "type": "string" }, "digits": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "hashIterations": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "hashedSaltedValue": { "type": "string" }, "id": { "type": "string" }, "period": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "priority": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "salt": { "type": "string" }, "secretData": { "type": "string" }, "temporary": { "type": "boolean" }, "type": { "type": "string" }, "userLabel": { "type": "string" }, "value": { "type": "string" } } } ``` </details>

---

### spec.definition.members[].credentials[].algorithm

Type: string

*missing*

---

### spec.definition.members[].credentials[].config

Type: object

*missing*

---

### spec.definition.members[].credentials[].counter

Type: integer

*missing*

---

### spec.definition.members[].credentials[].createdDate

Type: integer

*missing*

---

### spec.definition.members[].credentials[].credentialData

Type: string

*missing*

---

### spec.definition.members[].credentials[].device

Type: string

*missing*

---

### spec.definition.members[].credentials[].digits

Type: integer

*missing*

---

### spec.definition.members[].credentials[].hashIterations

Type: integer

*missing*

---

### spec.definition.members[].credentials[].hashedSaltedValue

Type: string

*missing*

---

### spec.definition.members[].credentials[].id

Type: string

*missing*

---

### spec.definition.members[].credentials[].period

Type: integer

*missing*

---

### spec.definition.members[].credentials[].priority

Type: integer

*missing*

---

### spec.definition.members[].credentials[].salt

Type: string

*missing*

---

### spec.definition.members[].credentials[].secretData

Type: string

*missing*

---

### spec.definition.members[].credentials[].temporary

Type: boolean

*missing*

---

### spec.definition.members[].credentials[].type

Type: string

*missing*

---

### spec.definition.members[].credentials[].userLabel

Type: string

*missing*

---

### spec.definition.members[].credentials[].value

Type: string

*missing*

---

### spec.definition.members[].disableableCredentialTypes[]

Type: string

*missing*

---

### spec.definition.members[].email

Type: string

*missing*

---

### spec.definition.members[].emailVerified

Type: boolean

*missing*

---

### spec.definition.members[].enabled

Type: boolean

*missing*

---

### spec.definition.members[].federatedIdentities[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[identityProvider](#specdefinitionmembersfederatedidentitiesidentityprovider)|string||
|[userId](#specdefinitionmembersfederatedidentitiesuserid)|string||
|[userName](#specdefinitionmembersfederatedidentitiesusername)|string||

FederatedIdentityRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "identityProvider": { "type": "string" }, "userId": { "type": "string" }, "userName": { "type": "string" } } } ``` </details>

---

### spec.definition.members[].federatedIdentities[].identityProvider

Type: string

*missing*

---

### spec.definition.members[].federatedIdentities[].userId

Type: string

*missing*

---

### spec.definition.members[].federatedIdentities[].userName

Type: string

*missing*

---

### spec.definition.members[].federationLink

Type: string

*missing*

---

### spec.definition.members[].firstName

Type: string

*missing*

---

### spec.definition.members[].groups[]

Type: string

*missing*

---

### spec.definition.members[].id

Type: string

*missing*

---

### spec.definition.members[].lastName

Type: string

*missing*

---

### spec.definition.members[].membershipType

Type: string

MembershipType

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "UNMANAGED", "MANAGED" ] } ``` </details>

---

### spec.definition.members[].notBefore

Type: integer

*missing*

---

### spec.definition.members[].origin

Type: string

*missing*

---

### spec.definition.members[].realmRoles[]

Type: string

*missing*

---

### spec.definition.members[].requiredActions[]

Type: string

*missing*

---

### spec.definition.members[].self

Type: string

*missing*

---

### spec.definition.members[].serviceAccountClientId

Type: string

*missing*

---

### spec.definition.members[].socialLinks[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[socialProvider](#specdefinitionmemberssociallinkssocialprovider)|string||
|[socialUserId](#specdefinitionmemberssociallinkssocialuserid)|string||
|[socialUsername](#specdefinitionmemberssociallinkssocialusername)|string||

SocialLinkRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "socialProvider": { "type": "string" }, "socialUserId": { "type": "string" }, "socialUsername": { "type": "string" } } } ``` </details>

---

### spec.definition.members[].socialLinks[].socialProvider

Type: string

*missing*

---

### spec.definition.members[].socialLinks[].socialUserId

Type: string

*missing*

---

### spec.definition.members[].socialLinks[].socialUsername

Type: string

*missing*

---

### spec.definition.members[].totp

Type: boolean

*missing*

---

### spec.definition.members[].userProfileMetadata

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[attributes[]](#specdefinitionmembersuserprofilemetadataattributes)|object||
|[groups[]](#specdefinitionmembersuserprofilemetadatagroups)|object||

UserProfileMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "attributes": { "type": "array", "items": { "$ref": "#/$defs/UserProfileAttributeMetadata" } }, "groups": { "type": "array", "items": { "$ref": "#/$defs/UserProfileAttributeGroupMetadata" } } } } ``` </details>

---

### spec.definition.members[].userProfileMetadata.attributes[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[annotations](#specdefinitionmembersuserprofilemetadataattributesannotations)|object||
|[displayName](#specdefinitionmembersuserprofilemetadataattributesdisplayname)|string||
|[group](#specdefinitionmembersuserprofilemetadataattributesgroup)|string||
|[multivalued](#specdefinitionmembersuserprofilemetadataattributesmultivalued)|boolean||
|[name](#specdefinitionmembersuserprofilemetadataattributesname)|string||
|[readOnly](#specdefinitionmembersuserprofilemetadataattributesreadonly)|boolean||
|[required](#specdefinitionmembersuserprofilemetadataattributesrequired)|boolean||
|[validators](#specdefinitionmembersuserprofilemetadataattributesvalidators)|object||

UserProfileAttributeMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "annotations": { "type": "object", "additionalProperties": {} }, "displayName": { "type": "string" }, "group": { "type": "string" }, "multivalued": { "type": "boolean" }, "name": { "type": "string" }, "readOnly": { "type": "boolean" }, "required": { "type": "boolean" }, "validators": { "type": "object", "additionalProperties": { "type": "object", "additionalProperties": {} } } } } ``` </details>

---

### spec.definition.members[].userProfileMetadata.attributes[].annotations

Type: object

*missing*

---

### spec.definition.members[].userProfileMetadata.attributes[].displayName

Type: string

*missing*

---

### spec.definition.members[].userProfileMetadata.attributes[].group

Type: string

*missing*

---

### spec.definition.members[].userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

---

### spec.definition.members[].userProfileMetadata.attributes[].name

Type: string

*missing*

---

### spec.definition.members[].userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

---

### spec.definition.members[].userProfileMetadata.attributes[].required

Type: boolean

*missing*

---

### spec.definition.members[].userProfileMetadata.attributes[].validators

Type: object

*missing*

---

### spec.definition.members[].userProfileMetadata.groups[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[annotations](#specdefinitionmembersuserprofilemetadatagroupsannotations)|object||
|[displayDescription](#specdefinitionmembersuserprofilemetadatagroupsdisplaydescription)|string||
|[displayHeader](#specdefinitionmembersuserprofilemetadatagroupsdisplayheader)|string||
|[name](#specdefinitionmembersuserprofilemetadatagroupsname)|string||

UserProfileAttributeGroupMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "annotations": { "type": "object", "additionalProperties": {} }, "displayDescription": { "type": "string" }, "displayHeader": { "type": "string" }, "name": { "type": "string" } } } ``` </details>

---

### spec.definition.members[].userProfileMetadata.groups[].annotations

Type: object

*missing*

---

### spec.definition.members[].userProfileMetadata.groups[].displayDescription

Type: string

*missing*

---

### spec.definition.members[].userProfileMetadata.groups[].displayHeader

Type: string

*missing*

---

### spec.definition.members[].userProfileMetadata.groups[].name

Type: string

*missing*

---

### spec.definition.members[].username

Type: string

*missing*

---

### spec.definition.name

Type: string

*missing*

---

### spec.definition.redirectUrl

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

### status

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[conditions[]](#statusconditions)|object||
|[instance](#statusinstance)|object||
|[message](#statusmessage)|string||
|[ready](#statusready)|boolean|✅|
|[reconcileAttempts](#statusreconcileattempts)|integer||
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

### status.reconcileAttempts

Type: integer

*missing*

---

### status.resourcePath

Type: string

*missing*

---

### status.status

Type: string

*missing*