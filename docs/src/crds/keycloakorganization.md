# KeycloakOrganization

## v1

resource to define an Organisation within a [KeyclaokRealm](./keycloakrealm.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
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
|[spec.realmRef](#specrealmref)|string|✅|
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
|[definition](#specdefinition)|object|✅|
|[options](#specoptions)|object||
|[patchFrom](#specpatchfrom)|object||
|[realmRef](#specrealmref)|string|✅|

the KeycloakOrganization resource

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

*missing*

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

*missing*

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

*missing*

---

### spec.definition.identityProviders[].addReadTokenRoleOnCreate

Type: boolean

*missing*

---

### spec.definition.identityProviders[].alias

Type: string

*missing*

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

*missing*

---

### spec.definition.identityProviders[].enabled

Type: boolean

*missing*

---

### spec.definition.identityProviders[].firstBrokerLoginFlowAlias

Type: string

*missing*

---

### spec.definition.identityProviders[].hideOnLogin

Type: boolean

*missing*

---

### spec.definition.identityProviders[].internalId

Type: string

*missing*

---

### spec.definition.identityProviders[].linkOnly

Type: boolean

*missing*

---

### spec.definition.identityProviders[].organizationId

Type: string

*missing*

---

### spec.definition.identityProviders[].postBrokerLoginFlowAlias

Type: string

*missing*

---

### spec.definition.identityProviders[].providerId

Type: string

*missing*

---

### spec.definition.identityProviders[].storeToken

Type: boolean

*missing*

---

### spec.definition.identityProviders[].trustEmail

Type: boolean

*missing*

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

*missing*

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

*missing*

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

*missing*

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

*missing*

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

*missing*

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

*missing*

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

*missing*

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

*missing*

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

*missing*

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

### spec.realmRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

the name of the kubernetes object that created the realm.

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