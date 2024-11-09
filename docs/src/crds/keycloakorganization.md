# KeycloakOrganization

## v1

Auto-generated derived type for KeycloakOrganizationSpec via `CustomResource`

|Name|Type|Required|
|:---|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.alias](#specdefinitionalias)|string||
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.description](#specdefinitiondescription)|string||
|[spec.definition.domains](#specdefinitiondomains)|array||
|[spec.definition.domains[]](#specdefinitiondomains)|object||
|[spec.definition.domains[].name](#specdefinitiondomainsname)|string||
|[spec.definition.domains[].verified](#specdefinitiondomainsverified)|boolean||
|[spec.definition.domains[].name](#specdefinitiondomainsname)|string||
|[spec.definition.domains[].verified](#specdefinitiondomainsverified)|boolean||
|[spec.definition.enabled](#specdefinitionenabled)|boolean||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.identityProviders](#specdefinitionidentityproviders)|array||
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
|[spec.definition.members](#specdefinitionmembers)|array||
|[spec.definition.members[]](#specdefinitionmembers)|object||
|[spec.definition.members[].access](#specdefinitionmembersaccess)|object||
|[spec.definition.members[].applicationRoles](#specdefinitionmembersapplicationroles)|object||
|[spec.definition.members[].attributes](#specdefinitionmembersattributes)|object||
|[spec.definition.members[].clientConsents](#specdefinitionmembersclientconsents)|array||
|[spec.definition.members[].clientConsents[]](#specdefinitionmembersclientconsents)|object||
|[spec.definition.members[].clientConsents[].clientId](#specdefinitionmembersclientconsentsclientid)|string||
|[spec.definition.members[].clientConsents[].createdDate](#specdefinitionmembersclientconsentscreateddate)|integer||
|[spec.definition.members[].clientConsents[].grantedClientScopes](#specdefinitionmembersclientconsentsgrantedclientscopes)|array||
|[spec.definition.members[].clientConsents[].grantedClientScopes[]](#specdefinitionmembersclientconsentsgrantedclientscopes)|string||
|[spec.definition.members[].clientConsents[].grantedRealmRoles](#specdefinitionmembersclientconsentsgrantedrealmroles)|array||
|[spec.definition.members[].clientConsents[].grantedRealmRoles[]](#specdefinitionmembersclientconsentsgrantedrealmroles)|string||
|[spec.definition.members[].clientConsents[].lastUpdatedDate](#specdefinitionmembersclientconsentslastupdateddate)|integer||
|[spec.definition.members[].clientConsents[].clientId](#specdefinitionmembersclientconsentsclientid)|string||
|[spec.definition.members[].clientConsents[].createdDate](#specdefinitionmembersclientconsentscreateddate)|integer||
|[spec.definition.members[].clientConsents[].grantedClientScopes](#specdefinitionmembersclientconsentsgrantedclientscopes)|array||
|[spec.definition.members[].clientConsents[].grantedClientScopes[]](#specdefinitionmembersclientconsentsgrantedclientscopes)|string||
|[spec.definition.members[].clientConsents[].grantedClientScopes[]](#specdefinitionmembersclientconsentsgrantedclientscopes)|string||
|[spec.definition.members[].clientConsents[].grantedRealmRoles](#specdefinitionmembersclientconsentsgrantedrealmroles)|array||
|[spec.definition.members[].clientConsents[].grantedRealmRoles[]](#specdefinitionmembersclientconsentsgrantedrealmroles)|string||
|[spec.definition.members[].clientConsents[].grantedRealmRoles[]](#specdefinitionmembersclientconsentsgrantedrealmroles)|string||
|[spec.definition.members[].clientConsents[].lastUpdatedDate](#specdefinitionmembersclientconsentslastupdateddate)|integer||
|[spec.definition.members[].clientRoles](#specdefinitionmembersclientroles)|object||
|[spec.definition.members[].createdTimestamp](#specdefinitionmemberscreatedtimestamp)|integer||
|[spec.definition.members[].credentials](#specdefinitionmemberscredentials)|array||
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
|[spec.definition.members[].disableableCredentialTypes](#specdefinitionmembersdisableablecredentialtypes)|array||
|[spec.definition.members[].disableableCredentialTypes[]](#specdefinitionmembersdisableablecredentialtypes)|string||
|[spec.definition.members[].email](#specdefinitionmembersemail)|string||
|[spec.definition.members[].emailVerified](#specdefinitionmembersemailverified)|boolean||
|[spec.definition.members[].enabled](#specdefinitionmembersenabled)|boolean||
|[spec.definition.members[].federatedIdentities](#specdefinitionmembersfederatedidentities)|array||
|[spec.definition.members[].federatedIdentities[]](#specdefinitionmembersfederatedidentities)|object||
|[spec.definition.members[].federatedIdentities[].identityProvider](#specdefinitionmembersfederatedidentitiesidentityprovider)|string||
|[spec.definition.members[].federatedIdentities[].userId](#specdefinitionmembersfederatedidentitiesuserid)|string||
|[spec.definition.members[].federatedIdentities[].userName](#specdefinitionmembersfederatedidentitiesusername)|string||
|[spec.definition.members[].federatedIdentities[].identityProvider](#specdefinitionmembersfederatedidentitiesidentityprovider)|string||
|[spec.definition.members[].federatedIdentities[].userId](#specdefinitionmembersfederatedidentitiesuserid)|string||
|[spec.definition.members[].federatedIdentities[].userName](#specdefinitionmembersfederatedidentitiesusername)|string||
|[spec.definition.members[].federationLink](#specdefinitionmembersfederationlink)|string||
|[spec.definition.members[].firstName](#specdefinitionmembersfirstname)|string||
|[spec.definition.members[].groups](#specdefinitionmembersgroups)|array||
|[spec.definition.members[].groups[]](#specdefinitionmembersgroups)|string||
|[spec.definition.members[].id](#specdefinitionmembersid)|string||
|[spec.definition.members[].lastName](#specdefinitionmemberslastname)|string||
|[spec.definition.members[].membershipType](#specdefinitionmembersmembershiptype)|string||
|[spec.definition.members[].notBefore](#specdefinitionmembersnotbefore)|integer||
|[spec.definition.members[].origin](#specdefinitionmembersorigin)|string||
|[spec.definition.members[].realmRoles](#specdefinitionmembersrealmroles)|array||
|[spec.definition.members[].realmRoles[]](#specdefinitionmembersrealmroles)|string||
|[spec.definition.members[].requiredActions](#specdefinitionmembersrequiredactions)|array||
|[spec.definition.members[].requiredActions[]](#specdefinitionmembersrequiredactions)|string||
|[spec.definition.members[].self](#specdefinitionmembersself)|string||
|[spec.definition.members[].serviceAccountClientId](#specdefinitionmembersserviceaccountclientid)|string||
|[spec.definition.members[].socialLinks](#specdefinitionmemberssociallinks)|array||
|[spec.definition.members[].socialLinks[]](#specdefinitionmemberssociallinks)|object||
|[spec.definition.members[].socialLinks[].socialProvider](#specdefinitionmemberssociallinkssocialprovider)|string||
|[spec.definition.members[].socialLinks[].socialUserId](#specdefinitionmemberssociallinkssocialuserid)|string||
|[spec.definition.members[].socialLinks[].socialUsername](#specdefinitionmemberssociallinkssocialusername)|string||
|[spec.definition.members[].socialLinks[].socialProvider](#specdefinitionmemberssociallinkssocialprovider)|string||
|[spec.definition.members[].socialLinks[].socialUserId](#specdefinitionmemberssociallinkssocialuserid)|string||
|[spec.definition.members[].socialLinks[].socialUsername](#specdefinitionmemberssociallinkssocialusername)|string||
|[spec.definition.members[].totp](#specdefinitionmemberstotp)|boolean||
|[spec.definition.members[].userProfileMetadata](#specdefinitionmembersuserprofilemetadata)|object||
|[spec.definition.members[].userProfileMetadata.attributes](#specdefinitionmembersuserprofilemetadataattributes)|array||
|[spec.definition.members[].userProfileMetadata.attributes[]](#specdefinitionmembersuserprofilemetadataattributes)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].annotations](#specdefinitionmembersuserprofilemetadataattributesannotations)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].displayName](#specdefinitionmembersuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].group](#specdefinitionmembersuserprofilemetadataattributesgroup)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].multivalued](#specdefinitionmembersuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].name](#specdefinitionmembersuserprofilemetadataattributesname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].readOnly](#specdefinitionmembersuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].required](#specdefinitionmembersuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].validators](#specdefinitionmembersuserprofilemetadataattributesvalidators)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].annotations](#specdefinitionmembersuserprofilemetadataattributesannotations)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].displayName](#specdefinitionmembersuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].group](#specdefinitionmembersuserprofilemetadataattributesgroup)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].multivalued](#specdefinitionmembersuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].name](#specdefinitionmembersuserprofilemetadataattributesname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].readOnly](#specdefinitionmembersuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].required](#specdefinitionmembersuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].validators](#specdefinitionmembersuserprofilemetadataattributesvalidators)|object||
|[spec.definition.members[].userProfileMetadata.groups](#specdefinitionmembersuserprofilemetadatagroups)|array||
|[spec.definition.members[].userProfileMetadata.groups[]](#specdefinitionmembersuserprofilemetadatagroups)|object||
|[spec.definition.members[].userProfileMetadata.groups[].annotations](#specdefinitionmembersuserprofilemetadatagroupsannotations)|object||
|[spec.definition.members[].userProfileMetadata.groups[].displayDescription](#specdefinitionmembersuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.members[].userProfileMetadata.groups[].displayHeader](#specdefinitionmembersuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.members[].userProfileMetadata.groups[].name](#specdefinitionmembersuserprofilemetadatagroupsname)|string||
|[spec.definition.members[].userProfileMetadata.groups[].annotations](#specdefinitionmembersuserprofilemetadatagroupsannotations)|object||
|[spec.definition.members[].userProfileMetadata.groups[].displayDescription](#specdefinitionmembersuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.members[].userProfileMetadata.groups[].displayHeader](#specdefinitionmembersuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.members[].userProfileMetadata.groups[].name](#specdefinitionmembersuserprofilemetadatagroupsname)|string||
|[spec.definition.members[].username](#specdefinitionmembersusername)|string||
|[spec.definition.members[].access](#specdefinitionmembersaccess)|object||
|[spec.definition.members[].applicationRoles](#specdefinitionmembersapplicationroles)|object||
|[spec.definition.members[].attributes](#specdefinitionmembersattributes)|object||
|[spec.definition.members[].clientConsents](#specdefinitionmembersclientconsents)|array||
|[spec.definition.members[].clientConsents[]](#specdefinitionmembersclientconsents)|object||
|[spec.definition.members[].clientConsents[].clientId](#specdefinitionmembersclientconsentsclientid)|string||
|[spec.definition.members[].clientConsents[].createdDate](#specdefinitionmembersclientconsentscreateddate)|integer||
|[spec.definition.members[].clientConsents[].grantedClientScopes](#specdefinitionmembersclientconsentsgrantedclientscopes)|array||
|[spec.definition.members[].clientConsents[].grantedClientScopes[]](#specdefinitionmembersclientconsentsgrantedclientscopes)|string||
|[spec.definition.members[].clientConsents[].grantedRealmRoles](#specdefinitionmembersclientconsentsgrantedrealmroles)|array||
|[spec.definition.members[].clientConsents[].grantedRealmRoles[]](#specdefinitionmembersclientconsentsgrantedrealmroles)|string||
|[spec.definition.members[].clientConsents[].lastUpdatedDate](#specdefinitionmembersclientconsentslastupdateddate)|integer||
|[spec.definition.members[].clientConsents[].clientId](#specdefinitionmembersclientconsentsclientid)|string||
|[spec.definition.members[].clientConsents[].createdDate](#specdefinitionmembersclientconsentscreateddate)|integer||
|[spec.definition.members[].clientConsents[].grantedClientScopes](#specdefinitionmembersclientconsentsgrantedclientscopes)|array||
|[spec.definition.members[].clientConsents[].grantedClientScopes[]](#specdefinitionmembersclientconsentsgrantedclientscopes)|string||
|[spec.definition.members[].clientConsents[].grantedClientScopes[]](#specdefinitionmembersclientconsentsgrantedclientscopes)|string||
|[spec.definition.members[].clientConsents[].grantedRealmRoles](#specdefinitionmembersclientconsentsgrantedrealmroles)|array||
|[spec.definition.members[].clientConsents[].grantedRealmRoles[]](#specdefinitionmembersclientconsentsgrantedrealmroles)|string||
|[spec.definition.members[].clientConsents[].grantedRealmRoles[]](#specdefinitionmembersclientconsentsgrantedrealmroles)|string||
|[spec.definition.members[].clientConsents[].lastUpdatedDate](#specdefinitionmembersclientconsentslastupdateddate)|integer||
|[spec.definition.members[].clientConsents[]](#specdefinitionmembersclientconsents)|object||
|[spec.definition.members[].clientConsents[].clientId](#specdefinitionmembersclientconsentsclientid)|string||
|[spec.definition.members[].clientConsents[].createdDate](#specdefinitionmembersclientconsentscreateddate)|integer||
|[spec.definition.members[].clientConsents[].grantedClientScopes](#specdefinitionmembersclientconsentsgrantedclientscopes)|array||
|[spec.definition.members[].clientConsents[].grantedClientScopes[]](#specdefinitionmembersclientconsentsgrantedclientscopes)|string||
|[spec.definition.members[].clientConsents[].grantedRealmRoles](#specdefinitionmembersclientconsentsgrantedrealmroles)|array||
|[spec.definition.members[].clientConsents[].grantedRealmRoles[]](#specdefinitionmembersclientconsentsgrantedrealmroles)|string||
|[spec.definition.members[].clientConsents[].lastUpdatedDate](#specdefinitionmembersclientconsentslastupdateddate)|integer||
|[spec.definition.members[].clientConsents[].clientId](#specdefinitionmembersclientconsentsclientid)|string||
|[spec.definition.members[].clientConsents[].createdDate](#specdefinitionmembersclientconsentscreateddate)|integer||
|[spec.definition.members[].clientConsents[].grantedClientScopes](#specdefinitionmembersclientconsentsgrantedclientscopes)|array||
|[spec.definition.members[].clientConsents[].grantedClientScopes[]](#specdefinitionmembersclientconsentsgrantedclientscopes)|string||
|[spec.definition.members[].clientConsents[].grantedClientScopes[]](#specdefinitionmembersclientconsentsgrantedclientscopes)|string||
|[spec.definition.members[].clientConsents[].grantedRealmRoles](#specdefinitionmembersclientconsentsgrantedrealmroles)|array||
|[spec.definition.members[].clientConsents[].grantedRealmRoles[]](#specdefinitionmembersclientconsentsgrantedrealmroles)|string||
|[spec.definition.members[].clientConsents[].grantedRealmRoles[]](#specdefinitionmembersclientconsentsgrantedrealmroles)|string||
|[spec.definition.members[].clientConsents[].lastUpdatedDate](#specdefinitionmembersclientconsentslastupdateddate)|integer||
|[spec.definition.members[].clientConsents[].clientId](#specdefinitionmembersclientconsentsclientid)|string||
|[spec.definition.members[].clientConsents[].createdDate](#specdefinitionmembersclientconsentscreateddate)|integer||
|[spec.definition.members[].clientConsents[].grantedClientScopes](#specdefinitionmembersclientconsentsgrantedclientscopes)|array||
|[spec.definition.members[].clientConsents[].grantedClientScopes[]](#specdefinitionmembersclientconsentsgrantedclientscopes)|string||
|[spec.definition.members[].clientConsents[].grantedClientScopes[]](#specdefinitionmembersclientconsentsgrantedclientscopes)|string||
|[spec.definition.members[].clientConsents[].grantedClientScopes[]](#specdefinitionmembersclientconsentsgrantedclientscopes)|string||
|[spec.definition.members[].clientConsents[].grantedRealmRoles](#specdefinitionmembersclientconsentsgrantedrealmroles)|array||
|[spec.definition.members[].clientConsents[].grantedRealmRoles[]](#specdefinitionmembersclientconsentsgrantedrealmroles)|string||
|[spec.definition.members[].clientConsents[].grantedRealmRoles[]](#specdefinitionmembersclientconsentsgrantedrealmroles)|string||
|[spec.definition.members[].clientConsents[].grantedRealmRoles[]](#specdefinitionmembersclientconsentsgrantedrealmroles)|string||
|[spec.definition.members[].clientConsents[].lastUpdatedDate](#specdefinitionmembersclientconsentslastupdateddate)|integer||
|[spec.definition.members[].clientRoles](#specdefinitionmembersclientroles)|object||
|[spec.definition.members[].createdTimestamp](#specdefinitionmemberscreatedtimestamp)|integer||
|[spec.definition.members[].credentials](#specdefinitionmemberscredentials)|array||
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
|[spec.definition.members[].disableableCredentialTypes](#specdefinitionmembersdisableablecredentialtypes)|array||
|[spec.definition.members[].disableableCredentialTypes[]](#specdefinitionmembersdisableablecredentialtypes)|string||
|[spec.definition.members[].disableableCredentialTypes[]](#specdefinitionmembersdisableablecredentialtypes)|string||
|[spec.definition.members[].email](#specdefinitionmembersemail)|string||
|[spec.definition.members[].emailVerified](#specdefinitionmembersemailverified)|boolean||
|[spec.definition.members[].enabled](#specdefinitionmembersenabled)|boolean||
|[spec.definition.members[].federatedIdentities](#specdefinitionmembersfederatedidentities)|array||
|[spec.definition.members[].federatedIdentities[]](#specdefinitionmembersfederatedidentities)|object||
|[spec.definition.members[].federatedIdentities[].identityProvider](#specdefinitionmembersfederatedidentitiesidentityprovider)|string||
|[spec.definition.members[].federatedIdentities[].userId](#specdefinitionmembersfederatedidentitiesuserid)|string||
|[spec.definition.members[].federatedIdentities[].userName](#specdefinitionmembersfederatedidentitiesusername)|string||
|[spec.definition.members[].federatedIdentities[].identityProvider](#specdefinitionmembersfederatedidentitiesidentityprovider)|string||
|[spec.definition.members[].federatedIdentities[].userId](#specdefinitionmembersfederatedidentitiesuserid)|string||
|[spec.definition.members[].federatedIdentities[].userName](#specdefinitionmembersfederatedidentitiesusername)|string||
|[spec.definition.members[].federatedIdentities[]](#specdefinitionmembersfederatedidentities)|object||
|[spec.definition.members[].federatedIdentities[].identityProvider](#specdefinitionmembersfederatedidentitiesidentityprovider)|string||
|[spec.definition.members[].federatedIdentities[].userId](#specdefinitionmembersfederatedidentitiesuserid)|string||
|[spec.definition.members[].federatedIdentities[].userName](#specdefinitionmembersfederatedidentitiesusername)|string||
|[spec.definition.members[].federatedIdentities[].identityProvider](#specdefinitionmembersfederatedidentitiesidentityprovider)|string||
|[spec.definition.members[].federatedIdentities[].userId](#specdefinitionmembersfederatedidentitiesuserid)|string||
|[spec.definition.members[].federatedIdentities[].userName](#specdefinitionmembersfederatedidentitiesusername)|string||
|[spec.definition.members[].federatedIdentities[].identityProvider](#specdefinitionmembersfederatedidentitiesidentityprovider)|string||
|[spec.definition.members[].federatedIdentities[].userId](#specdefinitionmembersfederatedidentitiesuserid)|string||
|[spec.definition.members[].federatedIdentities[].userName](#specdefinitionmembersfederatedidentitiesusername)|string||
|[spec.definition.members[].federationLink](#specdefinitionmembersfederationlink)|string||
|[spec.definition.members[].firstName](#specdefinitionmembersfirstname)|string||
|[spec.definition.members[].groups](#specdefinitionmembersgroups)|array||
|[spec.definition.members[].groups[]](#specdefinitionmembersgroups)|string||
|[spec.definition.members[].groups[]](#specdefinitionmembersgroups)|string||
|[spec.definition.members[].id](#specdefinitionmembersid)|string||
|[spec.definition.members[].lastName](#specdefinitionmemberslastname)|string||
|[spec.definition.members[].membershipType](#specdefinitionmembersmembershiptype)|string||
|[spec.definition.members[].notBefore](#specdefinitionmembersnotbefore)|integer||
|[spec.definition.members[].origin](#specdefinitionmembersorigin)|string||
|[spec.definition.members[].realmRoles](#specdefinitionmembersrealmroles)|array||
|[spec.definition.members[].realmRoles[]](#specdefinitionmembersrealmroles)|string||
|[spec.definition.members[].realmRoles[]](#specdefinitionmembersrealmroles)|string||
|[spec.definition.members[].requiredActions](#specdefinitionmembersrequiredactions)|array||
|[spec.definition.members[].requiredActions[]](#specdefinitionmembersrequiredactions)|string||
|[spec.definition.members[].requiredActions[]](#specdefinitionmembersrequiredactions)|string||
|[spec.definition.members[].self](#specdefinitionmembersself)|string||
|[spec.definition.members[].serviceAccountClientId](#specdefinitionmembersserviceaccountclientid)|string||
|[spec.definition.members[].socialLinks](#specdefinitionmemberssociallinks)|array||
|[spec.definition.members[].socialLinks[]](#specdefinitionmemberssociallinks)|object||
|[spec.definition.members[].socialLinks[].socialProvider](#specdefinitionmemberssociallinkssocialprovider)|string||
|[spec.definition.members[].socialLinks[].socialUserId](#specdefinitionmemberssociallinkssocialuserid)|string||
|[spec.definition.members[].socialLinks[].socialUsername](#specdefinitionmemberssociallinkssocialusername)|string||
|[spec.definition.members[].socialLinks[].socialProvider](#specdefinitionmemberssociallinkssocialprovider)|string||
|[spec.definition.members[].socialLinks[].socialUserId](#specdefinitionmemberssociallinkssocialuserid)|string||
|[spec.definition.members[].socialLinks[].socialUsername](#specdefinitionmemberssociallinkssocialusername)|string||
|[spec.definition.members[].socialLinks[]](#specdefinitionmemberssociallinks)|object||
|[spec.definition.members[].socialLinks[].socialProvider](#specdefinitionmemberssociallinkssocialprovider)|string||
|[spec.definition.members[].socialLinks[].socialUserId](#specdefinitionmemberssociallinkssocialuserid)|string||
|[spec.definition.members[].socialLinks[].socialUsername](#specdefinitionmemberssociallinkssocialusername)|string||
|[spec.definition.members[].socialLinks[].socialProvider](#specdefinitionmemberssociallinkssocialprovider)|string||
|[spec.definition.members[].socialLinks[].socialUserId](#specdefinitionmemberssociallinkssocialuserid)|string||
|[spec.definition.members[].socialLinks[].socialUsername](#specdefinitionmemberssociallinkssocialusername)|string||
|[spec.definition.members[].socialLinks[].socialProvider](#specdefinitionmemberssociallinkssocialprovider)|string||
|[spec.definition.members[].socialLinks[].socialUserId](#specdefinitionmemberssociallinkssocialuserid)|string||
|[spec.definition.members[].socialLinks[].socialUsername](#specdefinitionmemberssociallinkssocialusername)|string||
|[spec.definition.members[].totp](#specdefinitionmemberstotp)|boolean||
|[spec.definition.members[].userProfileMetadata](#specdefinitionmembersuserprofilemetadata)|object||
|[spec.definition.members[].userProfileMetadata.attributes](#specdefinitionmembersuserprofilemetadataattributes)|array||
|[spec.definition.members[].userProfileMetadata.attributes[]](#specdefinitionmembersuserprofilemetadataattributes)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].annotations](#specdefinitionmembersuserprofilemetadataattributesannotations)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].displayName](#specdefinitionmembersuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].group](#specdefinitionmembersuserprofilemetadataattributesgroup)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].multivalued](#specdefinitionmembersuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].name](#specdefinitionmembersuserprofilemetadataattributesname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].readOnly](#specdefinitionmembersuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].required](#specdefinitionmembersuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].validators](#specdefinitionmembersuserprofilemetadataattributesvalidators)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].annotations](#specdefinitionmembersuserprofilemetadataattributesannotations)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].displayName](#specdefinitionmembersuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].group](#specdefinitionmembersuserprofilemetadataattributesgroup)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].multivalued](#specdefinitionmembersuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].name](#specdefinitionmembersuserprofilemetadataattributesname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].readOnly](#specdefinitionmembersuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].required](#specdefinitionmembersuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].validators](#specdefinitionmembersuserprofilemetadataattributesvalidators)|object||
|[spec.definition.members[].userProfileMetadata.groups](#specdefinitionmembersuserprofilemetadatagroups)|array||
|[spec.definition.members[].userProfileMetadata.groups[]](#specdefinitionmembersuserprofilemetadatagroups)|object||
|[spec.definition.members[].userProfileMetadata.groups[].annotations](#specdefinitionmembersuserprofilemetadatagroupsannotations)|object||
|[spec.definition.members[].userProfileMetadata.groups[].displayDescription](#specdefinitionmembersuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.members[].userProfileMetadata.groups[].displayHeader](#specdefinitionmembersuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.members[].userProfileMetadata.groups[].name](#specdefinitionmembersuserprofilemetadatagroupsname)|string||
|[spec.definition.members[].userProfileMetadata.groups[].annotations](#specdefinitionmembersuserprofilemetadatagroupsannotations)|object||
|[spec.definition.members[].userProfileMetadata.groups[].displayDescription](#specdefinitionmembersuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.members[].userProfileMetadata.groups[].displayHeader](#specdefinitionmembersuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.members[].userProfileMetadata.groups[].name](#specdefinitionmembersuserprofilemetadatagroupsname)|string||
|[spec.definition.members[].userProfileMetadata.attributes](#specdefinitionmembersuserprofilemetadataattributes)|array||
|[spec.definition.members[].userProfileMetadata.attributes[]](#specdefinitionmembersuserprofilemetadataattributes)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].annotations](#specdefinitionmembersuserprofilemetadataattributesannotations)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].displayName](#specdefinitionmembersuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].group](#specdefinitionmembersuserprofilemetadataattributesgroup)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].multivalued](#specdefinitionmembersuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].name](#specdefinitionmembersuserprofilemetadataattributesname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].readOnly](#specdefinitionmembersuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].required](#specdefinitionmembersuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].validators](#specdefinitionmembersuserprofilemetadataattributesvalidators)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].annotations](#specdefinitionmembersuserprofilemetadataattributesannotations)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].displayName](#specdefinitionmembersuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].group](#specdefinitionmembersuserprofilemetadataattributesgroup)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].multivalued](#specdefinitionmembersuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].name](#specdefinitionmembersuserprofilemetadataattributesname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].readOnly](#specdefinitionmembersuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].required](#specdefinitionmembersuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].validators](#specdefinitionmembersuserprofilemetadataattributesvalidators)|object||
|[spec.definition.members[].userProfileMetadata.attributes[]](#specdefinitionmembersuserprofilemetadataattributes)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].annotations](#specdefinitionmembersuserprofilemetadataattributesannotations)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].displayName](#specdefinitionmembersuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].group](#specdefinitionmembersuserprofilemetadataattributesgroup)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].multivalued](#specdefinitionmembersuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].name](#specdefinitionmembersuserprofilemetadataattributesname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].readOnly](#specdefinitionmembersuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].required](#specdefinitionmembersuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].validators](#specdefinitionmembersuserprofilemetadataattributesvalidators)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].annotations](#specdefinitionmembersuserprofilemetadataattributesannotations)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].displayName](#specdefinitionmembersuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].group](#specdefinitionmembersuserprofilemetadataattributesgroup)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].multivalued](#specdefinitionmembersuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].name](#specdefinitionmembersuserprofilemetadataattributesname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].readOnly](#specdefinitionmembersuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].required](#specdefinitionmembersuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].validators](#specdefinitionmembersuserprofilemetadataattributesvalidators)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].annotations](#specdefinitionmembersuserprofilemetadataattributesannotations)|object||
|[spec.definition.members[].userProfileMetadata.attributes[].displayName](#specdefinitionmembersuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].group](#specdefinitionmembersuserprofilemetadataattributesgroup)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].multivalued](#specdefinitionmembersuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].name](#specdefinitionmembersuserprofilemetadataattributesname)|string||
|[spec.definition.members[].userProfileMetadata.attributes[].readOnly](#specdefinitionmembersuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].required](#specdefinitionmembersuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.members[].userProfileMetadata.attributes[].validators](#specdefinitionmembersuserprofilemetadataattributesvalidators)|object||
|[spec.definition.members[].userProfileMetadata.groups](#specdefinitionmembersuserprofilemetadatagroups)|array||
|[spec.definition.members[].userProfileMetadata.groups[]](#specdefinitionmembersuserprofilemetadatagroups)|object||
|[spec.definition.members[].userProfileMetadata.groups[].annotations](#specdefinitionmembersuserprofilemetadatagroupsannotations)|object||
|[spec.definition.members[].userProfileMetadata.groups[].displayDescription](#specdefinitionmembersuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.members[].userProfileMetadata.groups[].displayHeader](#specdefinitionmembersuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.members[].userProfileMetadata.groups[].name](#specdefinitionmembersuserprofilemetadatagroupsname)|string||
|[spec.definition.members[].userProfileMetadata.groups[].annotations](#specdefinitionmembersuserprofilemetadatagroupsannotations)|object||
|[spec.definition.members[].userProfileMetadata.groups[].displayDescription](#specdefinitionmembersuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.members[].userProfileMetadata.groups[].displayHeader](#specdefinitionmembersuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.members[].userProfileMetadata.groups[].name](#specdefinitionmembersuserprofilemetadatagroupsname)|string||
|[spec.definition.members[].userProfileMetadata.groups[]](#specdefinitionmembersuserprofilemetadatagroups)|object||
|[spec.definition.members[].userProfileMetadata.groups[].annotations](#specdefinitionmembersuserprofilemetadatagroupsannotations)|object||
|[spec.definition.members[].userProfileMetadata.groups[].displayDescription](#specdefinitionmembersuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.members[].userProfileMetadata.groups[].displayHeader](#specdefinitionmembersuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.members[].userProfileMetadata.groups[].name](#specdefinitionmembersuserprofilemetadatagroupsname)|string||
|[spec.definition.members[].userProfileMetadata.groups[].annotations](#specdefinitionmembersuserprofilemetadatagroupsannotations)|object||
|[spec.definition.members[].userProfileMetadata.groups[].displayDescription](#specdefinitionmembersuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.members[].userProfileMetadata.groups[].displayHeader](#specdefinitionmembersuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.members[].userProfileMetadata.groups[].name](#specdefinitionmembersuserprofilemetadatagroupsname)|string||
|[spec.definition.members[].userProfileMetadata.groups[].annotations](#specdefinitionmembersuserprofilemetadatagroupsannotations)|object||
|[spec.definition.members[].userProfileMetadata.groups[].displayDescription](#specdefinitionmembersuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.members[].userProfileMetadata.groups[].displayHeader](#specdefinitionmembersuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.members[].userProfileMetadata.groups[].name](#specdefinitionmembersuserprofilemetadatagroupsname)|string||
|[spec.definition.members[].username](#specdefinitionmembersusername)|string||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.definition.redirectUrl](#specdefinitionredirecturl)|string||
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

the KeycloakOrganization resource

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

### spec.definition.attributes

Type: object

*missing*

### spec.definition.description

Type: string

*missing*

### spec.definition.domains

Type: array

*missing*

### spec.definition.domains[]

Type: object

*missing*

### spec.definition.domains[].name

Type: string

*missing*

### spec.definition.domains[].verified

Type: boolean

*missing*

### spec.definition.domains[].name

Type: string

*missing*

### spec.definition.domains[].verified

Type: boolean

*missing*

### spec.definition.enabled

Type: boolean

*missing*

### spec.definition.id

Type: string

#### Validations

|Rule|Error Message|
|:---|:------------|
|self == oldSelf|Value is immutable|

*missing*

### spec.definition.identityProviders

Type: array

*missing*

### spec.definition.identityProviders[]

Type: object

*missing*

### spec.definition.identityProviders[].addReadTokenRoleOnCreate

Type: boolean

*missing*

### spec.definition.identityProviders[].alias

Type: string

*missing*

### spec.definition.identityProviders[].authenticateByDefault

Type: boolean

*missing*

### spec.definition.identityProviders[].config

Type: object

*missing*

### spec.definition.identityProviders[].displayName

Type: string

*missing*

### spec.definition.identityProviders[].enabled

Type: boolean

*missing*

### spec.definition.identityProviders[].firstBrokerLoginFlowAlias

Type: string

*missing*

### spec.definition.identityProviders[].hideOnLogin

Type: boolean

*missing*

### spec.definition.identityProviders[].internalId

Type: string

*missing*

### spec.definition.identityProviders[].linkOnly

Type: boolean

*missing*

### spec.definition.identityProviders[].organizationId

Type: string

*missing*

### spec.definition.identityProviders[].postBrokerLoginFlowAlias

Type: string

*missing*

### spec.definition.identityProviders[].providerId

Type: string

*missing*

### spec.definition.identityProviders[].storeToken

Type: boolean

*missing*

### spec.definition.identityProviders[].trustEmail

Type: boolean

*missing*

### spec.definition.identityProviders[].updateProfileFirstLogin

Type: boolean

*missing*

### spec.definition.identityProviders[].updateProfileFirstLoginMode

Type: string

*missing*

### spec.definition.identityProviders[].addReadTokenRoleOnCreate

Type: boolean

*missing*

### spec.definition.identityProviders[].alias

Type: string

*missing*

### spec.definition.identityProviders[].authenticateByDefault

Type: boolean

*missing*

### spec.definition.identityProviders[].config

Type: object

*missing*

### spec.definition.identityProviders[].displayName

Type: string

*missing*

### spec.definition.identityProviders[].enabled

Type: boolean

*missing*

### spec.definition.identityProviders[].firstBrokerLoginFlowAlias

Type: string

*missing*

### spec.definition.identityProviders[].hideOnLogin

Type: boolean

*missing*

### spec.definition.identityProviders[].internalId

Type: string

*missing*

### spec.definition.identityProviders[].linkOnly

Type: boolean

*missing*

### spec.definition.identityProviders[].organizationId

Type: string

*missing*

### spec.definition.identityProviders[].postBrokerLoginFlowAlias

Type: string

*missing*

### spec.definition.identityProviders[].providerId

Type: string

*missing*

### spec.definition.identityProviders[].storeToken

Type: boolean

*missing*

### spec.definition.identityProviders[].trustEmail

Type: boolean

*missing*

### spec.definition.identityProviders[].updateProfileFirstLogin

Type: boolean

*missing*

### spec.definition.identityProviders[].updateProfileFirstLoginMode

Type: string

*missing*

### spec.definition.members

Type: array

*missing*

### spec.definition.members[]

Type: object

*missing*

### spec.definition.members[].access

Type: object

*missing*

### spec.definition.members[].applicationRoles

Type: object

*missing*

### spec.definition.members[].attributes

Type: object

*missing*

### spec.definition.members[].clientConsents

Type: array

*missing*

### spec.definition.members[].clientConsents[]

Type: object

*missing*

### spec.definition.members[].clientConsents[].clientId

Type: string

*missing*

### spec.definition.members[].clientConsents[].createdDate

Type: integer

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes

Type: array

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles

Type: array

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].lastUpdatedDate

Type: integer

*missing*

### spec.definition.members[].clientConsents[].clientId

Type: string

*missing*

### spec.definition.members[].clientConsents[].createdDate

Type: integer

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes

Type: array

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles

Type: array

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].lastUpdatedDate

Type: integer

*missing*

### spec.definition.members[].clientRoles

Type: object

*missing*

### spec.definition.members[].createdTimestamp

Type: integer

*missing*

### spec.definition.members[].credentials

Type: array

*missing*

### spec.definition.members[].credentials[]

Type: object

*missing*

### spec.definition.members[].credentials[].algorithm

Type: string

*missing*

### spec.definition.members[].credentials[].config

Type: object

*missing*

### spec.definition.members[].credentials[].counter

Type: integer

*missing*

### spec.definition.members[].credentials[].createdDate

Type: integer

*missing*

### spec.definition.members[].credentials[].credentialData

Type: string

*missing*

### spec.definition.members[].credentials[].device

Type: string

*missing*

### spec.definition.members[].credentials[].digits

Type: integer

*missing*

### spec.definition.members[].credentials[].hashIterations

Type: integer

*missing*

### spec.definition.members[].credentials[].hashedSaltedValue

Type: string

*missing*

### spec.definition.members[].credentials[].id

Type: string

*missing*

### spec.definition.members[].credentials[].period

Type: integer

*missing*

### spec.definition.members[].credentials[].priority

Type: integer

*missing*

### spec.definition.members[].credentials[].salt

Type: string

*missing*

### spec.definition.members[].credentials[].secretData

Type: string

*missing*

### spec.definition.members[].credentials[].temporary

Type: boolean

*missing*

### spec.definition.members[].credentials[].type

Type: string

*missing*

### spec.definition.members[].credentials[].userLabel

Type: string

*missing*

### spec.definition.members[].credentials[].value

Type: string

*missing*

### spec.definition.members[].credentials[].algorithm

Type: string

*missing*

### spec.definition.members[].credentials[].config

Type: object

*missing*

### spec.definition.members[].credentials[].counter

Type: integer

*missing*

### spec.definition.members[].credentials[].createdDate

Type: integer

*missing*

### spec.definition.members[].credentials[].credentialData

Type: string

*missing*

### spec.definition.members[].credentials[].device

Type: string

*missing*

### spec.definition.members[].credentials[].digits

Type: integer

*missing*

### spec.definition.members[].credentials[].hashIterations

Type: integer

*missing*

### spec.definition.members[].credentials[].hashedSaltedValue

Type: string

*missing*

### spec.definition.members[].credentials[].id

Type: string

*missing*

### spec.definition.members[].credentials[].period

Type: integer

*missing*

### spec.definition.members[].credentials[].priority

Type: integer

*missing*

### spec.definition.members[].credentials[].salt

Type: string

*missing*

### spec.definition.members[].credentials[].secretData

Type: string

*missing*

### spec.definition.members[].credentials[].temporary

Type: boolean

*missing*

### spec.definition.members[].credentials[].type

Type: string

*missing*

### spec.definition.members[].credentials[].userLabel

Type: string

*missing*

### spec.definition.members[].credentials[].value

Type: string

*missing*

### spec.definition.members[].disableableCredentialTypes

Type: array

*missing*

### spec.definition.members[].disableableCredentialTypes[]

Type: string

*missing*

### spec.definition.members[].email

Type: string

*missing*

### spec.definition.members[].emailVerified

Type: boolean

*missing*

### spec.definition.members[].enabled

Type: boolean

*missing*

### spec.definition.members[].federatedIdentities

Type: array

*missing*

### spec.definition.members[].federatedIdentities[]

Type: object

*missing*

### spec.definition.members[].federatedIdentities[].identityProvider

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].userId

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].userName

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].identityProvider

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].userId

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].userName

Type: string

*missing*

### spec.definition.members[].federationLink

Type: string

*missing*

### spec.definition.members[].firstName

Type: string

*missing*

### spec.definition.members[].groups

Type: array

*missing*

### spec.definition.members[].groups[]

Type: string

*missing*

### spec.definition.members[].id

Type: string

*missing*

### spec.definition.members[].lastName

Type: string

*missing*

### spec.definition.members[].membershipType

Type: string

*missing*

### spec.definition.members[].notBefore

Type: integer

*missing*

### spec.definition.members[].origin

Type: string

*missing*

### spec.definition.members[].realmRoles

Type: array

*missing*

### spec.definition.members[].realmRoles[]

Type: string

*missing*

### spec.definition.members[].requiredActions

Type: array

*missing*

### spec.definition.members[].requiredActions[]

Type: string

*missing*

### spec.definition.members[].self

Type: string

*missing*

### spec.definition.members[].serviceAccountClientId

Type: string

*missing*

### spec.definition.members[].socialLinks

Type: array

*missing*

### spec.definition.members[].socialLinks[]

Type: object

*missing*

### spec.definition.members[].socialLinks[].socialProvider

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialUserId

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialUsername

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialProvider

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialUserId

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialUsername

Type: string

*missing*

### spec.definition.members[].totp

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes

Type: array

*missing*

### spec.definition.members[].userProfileMetadata.attributes[]

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].displayName

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].group

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].required

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].validators

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].displayName

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].group

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].required

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].validators

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups

Type: array

*missing*

### spec.definition.members[].userProfileMetadata.groups[]

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayDescription

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayHeader

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayDescription

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayHeader

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].name

Type: string

*missing*

### spec.definition.members[].username

Type: string

*missing*

### spec.definition.members[].access

Type: object

*missing*

### spec.definition.members[].applicationRoles

Type: object

*missing*

### spec.definition.members[].attributes

Type: object

*missing*

### spec.definition.members[].clientConsents

Type: array

*missing*

### spec.definition.members[].clientConsents[]

Type: object

*missing*

### spec.definition.members[].clientConsents[].clientId

Type: string

*missing*

### spec.definition.members[].clientConsents[].createdDate

Type: integer

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes

Type: array

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles

Type: array

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].lastUpdatedDate

Type: integer

*missing*

### spec.definition.members[].clientConsents[].clientId

Type: string

*missing*

### spec.definition.members[].clientConsents[].createdDate

Type: integer

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes

Type: array

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles

Type: array

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].lastUpdatedDate

Type: integer

*missing*

### spec.definition.members[].clientConsents[]

Type: object

*missing*

### spec.definition.members[].clientConsents[].clientId

Type: string

*missing*

### spec.definition.members[].clientConsents[].createdDate

Type: integer

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes

Type: array

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles

Type: array

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].lastUpdatedDate

Type: integer

*missing*

### spec.definition.members[].clientConsents[].clientId

Type: string

*missing*

### spec.definition.members[].clientConsents[].createdDate

Type: integer

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes

Type: array

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles

Type: array

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].lastUpdatedDate

Type: integer

*missing*

### spec.definition.members[].clientConsents[].clientId

Type: string

*missing*

### spec.definition.members[].clientConsents[].createdDate

Type: integer

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes

Type: array

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles

Type: array

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

### spec.definition.members[].clientConsents[].lastUpdatedDate

Type: integer

*missing*

### spec.definition.members[].clientRoles

Type: object

*missing*

### spec.definition.members[].createdTimestamp

Type: integer

*missing*

### spec.definition.members[].credentials

Type: array

*missing*

### spec.definition.members[].credentials[]

Type: object

*missing*

### spec.definition.members[].credentials[].algorithm

Type: string

*missing*

### spec.definition.members[].credentials[].config

Type: object

*missing*

### spec.definition.members[].credentials[].counter

Type: integer

*missing*

### spec.definition.members[].credentials[].createdDate

Type: integer

*missing*

### spec.definition.members[].credentials[].credentialData

Type: string

*missing*

### spec.definition.members[].credentials[].device

Type: string

*missing*

### spec.definition.members[].credentials[].digits

Type: integer

*missing*

### spec.definition.members[].credentials[].hashIterations

Type: integer

*missing*

### spec.definition.members[].credentials[].hashedSaltedValue

Type: string

*missing*

### spec.definition.members[].credentials[].id

Type: string

*missing*

### spec.definition.members[].credentials[].period

Type: integer

*missing*

### spec.definition.members[].credentials[].priority

Type: integer

*missing*

### spec.definition.members[].credentials[].salt

Type: string

*missing*

### spec.definition.members[].credentials[].secretData

Type: string

*missing*

### spec.definition.members[].credentials[].temporary

Type: boolean

*missing*

### spec.definition.members[].credentials[].type

Type: string

*missing*

### spec.definition.members[].credentials[].userLabel

Type: string

*missing*

### spec.definition.members[].credentials[].value

Type: string

*missing*

### spec.definition.members[].credentials[].algorithm

Type: string

*missing*

### spec.definition.members[].credentials[].config

Type: object

*missing*

### spec.definition.members[].credentials[].counter

Type: integer

*missing*

### spec.definition.members[].credentials[].createdDate

Type: integer

*missing*

### spec.definition.members[].credentials[].credentialData

Type: string

*missing*

### spec.definition.members[].credentials[].device

Type: string

*missing*

### spec.definition.members[].credentials[].digits

Type: integer

*missing*

### spec.definition.members[].credentials[].hashIterations

Type: integer

*missing*

### spec.definition.members[].credentials[].hashedSaltedValue

Type: string

*missing*

### spec.definition.members[].credentials[].id

Type: string

*missing*

### spec.definition.members[].credentials[].period

Type: integer

*missing*

### spec.definition.members[].credentials[].priority

Type: integer

*missing*

### spec.definition.members[].credentials[].salt

Type: string

*missing*

### spec.definition.members[].credentials[].secretData

Type: string

*missing*

### spec.definition.members[].credentials[].temporary

Type: boolean

*missing*

### spec.definition.members[].credentials[].type

Type: string

*missing*

### spec.definition.members[].credentials[].userLabel

Type: string

*missing*

### spec.definition.members[].credentials[].value

Type: string

*missing*

### spec.definition.members[].credentials[]

Type: object

*missing*

### spec.definition.members[].credentials[].algorithm

Type: string

*missing*

### spec.definition.members[].credentials[].config

Type: object

*missing*

### spec.definition.members[].credentials[].counter

Type: integer

*missing*

### spec.definition.members[].credentials[].createdDate

Type: integer

*missing*

### spec.definition.members[].credentials[].credentialData

Type: string

*missing*

### spec.definition.members[].credentials[].device

Type: string

*missing*

### spec.definition.members[].credentials[].digits

Type: integer

*missing*

### spec.definition.members[].credentials[].hashIterations

Type: integer

*missing*

### spec.definition.members[].credentials[].hashedSaltedValue

Type: string

*missing*

### spec.definition.members[].credentials[].id

Type: string

*missing*

### spec.definition.members[].credentials[].period

Type: integer

*missing*

### spec.definition.members[].credentials[].priority

Type: integer

*missing*

### spec.definition.members[].credentials[].salt

Type: string

*missing*

### spec.definition.members[].credentials[].secretData

Type: string

*missing*

### spec.definition.members[].credentials[].temporary

Type: boolean

*missing*

### spec.definition.members[].credentials[].type

Type: string

*missing*

### spec.definition.members[].credentials[].userLabel

Type: string

*missing*

### spec.definition.members[].credentials[].value

Type: string

*missing*

### spec.definition.members[].credentials[].algorithm

Type: string

*missing*

### spec.definition.members[].credentials[].config

Type: object

*missing*

### spec.definition.members[].credentials[].counter

Type: integer

*missing*

### spec.definition.members[].credentials[].createdDate

Type: integer

*missing*

### spec.definition.members[].credentials[].credentialData

Type: string

*missing*

### spec.definition.members[].credentials[].device

Type: string

*missing*

### spec.definition.members[].credentials[].digits

Type: integer

*missing*

### spec.definition.members[].credentials[].hashIterations

Type: integer

*missing*

### spec.definition.members[].credentials[].hashedSaltedValue

Type: string

*missing*

### spec.definition.members[].credentials[].id

Type: string

*missing*

### spec.definition.members[].credentials[].period

Type: integer

*missing*

### spec.definition.members[].credentials[].priority

Type: integer

*missing*

### spec.definition.members[].credentials[].salt

Type: string

*missing*

### spec.definition.members[].credentials[].secretData

Type: string

*missing*

### spec.definition.members[].credentials[].temporary

Type: boolean

*missing*

### spec.definition.members[].credentials[].type

Type: string

*missing*

### spec.definition.members[].credentials[].userLabel

Type: string

*missing*

### spec.definition.members[].credentials[].value

Type: string

*missing*

### spec.definition.members[].credentials[].algorithm

Type: string

*missing*

### spec.definition.members[].credentials[].config

Type: object

*missing*

### spec.definition.members[].credentials[].counter

Type: integer

*missing*

### spec.definition.members[].credentials[].createdDate

Type: integer

*missing*

### spec.definition.members[].credentials[].credentialData

Type: string

*missing*

### spec.definition.members[].credentials[].device

Type: string

*missing*

### spec.definition.members[].credentials[].digits

Type: integer

*missing*

### spec.definition.members[].credentials[].hashIterations

Type: integer

*missing*

### spec.definition.members[].credentials[].hashedSaltedValue

Type: string

*missing*

### spec.definition.members[].credentials[].id

Type: string

*missing*

### spec.definition.members[].credentials[].period

Type: integer

*missing*

### spec.definition.members[].credentials[].priority

Type: integer

*missing*

### spec.definition.members[].credentials[].salt

Type: string

*missing*

### spec.definition.members[].credentials[].secretData

Type: string

*missing*

### spec.definition.members[].credentials[].temporary

Type: boolean

*missing*

### spec.definition.members[].credentials[].type

Type: string

*missing*

### spec.definition.members[].credentials[].userLabel

Type: string

*missing*

### spec.definition.members[].credentials[].value

Type: string

*missing*

### spec.definition.members[].disableableCredentialTypes

Type: array

*missing*

### spec.definition.members[].disableableCredentialTypes[]

Type: string

*missing*

### spec.definition.members[].disableableCredentialTypes[]

Type: string

*missing*

### spec.definition.members[].email

Type: string

*missing*

### spec.definition.members[].emailVerified

Type: boolean

*missing*

### spec.definition.members[].enabled

Type: boolean

*missing*

### spec.definition.members[].federatedIdentities

Type: array

*missing*

### spec.definition.members[].federatedIdentities[]

Type: object

*missing*

### spec.definition.members[].federatedIdentities[].identityProvider

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].userId

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].userName

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].identityProvider

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].userId

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].userName

Type: string

*missing*

### spec.definition.members[].federatedIdentities[]

Type: object

*missing*

### spec.definition.members[].federatedIdentities[].identityProvider

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].userId

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].userName

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].identityProvider

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].userId

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].userName

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].identityProvider

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].userId

Type: string

*missing*

### spec.definition.members[].federatedIdentities[].userName

Type: string

*missing*

### spec.definition.members[].federationLink

Type: string

*missing*

### spec.definition.members[].firstName

Type: string

*missing*

### spec.definition.members[].groups

Type: array

*missing*

### spec.definition.members[].groups[]

Type: string

*missing*

### spec.definition.members[].groups[]

Type: string

*missing*

### spec.definition.members[].id

Type: string

*missing*

### spec.definition.members[].lastName

Type: string

*missing*

### spec.definition.members[].membershipType

Type: string

*missing*

### spec.definition.members[].notBefore

Type: integer

*missing*

### spec.definition.members[].origin

Type: string

*missing*

### spec.definition.members[].realmRoles

Type: array

*missing*

### spec.definition.members[].realmRoles[]

Type: string

*missing*

### spec.definition.members[].realmRoles[]

Type: string

*missing*

### spec.definition.members[].requiredActions

Type: array

*missing*

### spec.definition.members[].requiredActions[]

Type: string

*missing*

### spec.definition.members[].requiredActions[]

Type: string

*missing*

### spec.definition.members[].self

Type: string

*missing*

### spec.definition.members[].serviceAccountClientId

Type: string

*missing*

### spec.definition.members[].socialLinks

Type: array

*missing*

### spec.definition.members[].socialLinks[]

Type: object

*missing*

### spec.definition.members[].socialLinks[].socialProvider

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialUserId

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialUsername

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialProvider

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialUserId

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialUsername

Type: string

*missing*

### spec.definition.members[].socialLinks[]

Type: object

*missing*

### spec.definition.members[].socialLinks[].socialProvider

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialUserId

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialUsername

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialProvider

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialUserId

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialUsername

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialProvider

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialUserId

Type: string

*missing*

### spec.definition.members[].socialLinks[].socialUsername

Type: string

*missing*

### spec.definition.members[].totp

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes

Type: array

*missing*

### spec.definition.members[].userProfileMetadata.attributes[]

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].displayName

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].group

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].required

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].validators

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].displayName

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].group

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].required

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].validators

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups

Type: array

*missing*

### spec.definition.members[].userProfileMetadata.groups[]

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayDescription

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayHeader

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayDescription

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayHeader

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes

Type: array

*missing*

### spec.definition.members[].userProfileMetadata.attributes[]

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].displayName

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].group

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].required

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].validators

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].displayName

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].group

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].required

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].validators

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[]

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].displayName

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].group

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].required

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].validators

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].displayName

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].group

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].required

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].validators

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].displayName

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].group

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].required

Type: boolean

*missing*

### spec.definition.members[].userProfileMetadata.attributes[].validators

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups

Type: array

*missing*

### spec.definition.members[].userProfileMetadata.groups[]

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayDescription

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayHeader

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayDescription

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayHeader

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[]

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayDescription

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayHeader

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayDescription

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayHeader

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].name

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].annotations

Type: object

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayDescription

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].displayHeader

Type: string

*missing*

### spec.definition.members[].userProfileMetadata.groups[].name

Type: string

*missing*

### spec.definition.members[].username

Type: string

*missing*

### spec.definition.name

Type: string

*missing*

### spec.definition.redirectUrl

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