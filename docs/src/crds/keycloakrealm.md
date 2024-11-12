# KeycloakRealm

## v1

resource to define an Realm within a [KeyclaokInstance](./keycloakinstance.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.accessCodeLifespan](#specdefinitionaccesscodelifespan)|integer||
|[spec.definition.accessCodeLifespanLogin](#specdefinitionaccesscodelifespanlogin)|integer||
|[spec.definition.accessCodeLifespanUserAction](#specdefinitionaccesscodelifespanuseraction)|integer||
|[spec.definition.accessTokenLifespan](#specdefinitionaccesstokenlifespan)|integer||
|[spec.definition.accessTokenLifespanForImplicitFlow](#specdefinitionaccesstokenlifespanforimplicitflow)|integer||
|[spec.definition.accountTheme](#specdefinitionaccounttheme)|string||
|[spec.definition.actionTokenGeneratedByAdminLifespan](#specdefinitionactiontokengeneratedbyadminlifespan)|integer||
|[spec.definition.actionTokenGeneratedByUserLifespan](#specdefinitionactiontokengeneratedbyuserlifespan)|integer||
|[spec.definition.adminEventsDetailsEnabled](#specdefinitionadmineventsdetailsenabled)|boolean||
|[spec.definition.adminEventsEnabled](#specdefinitionadmineventsenabled)|boolean||
|[spec.definition.adminTheme](#specdefinitionadmintheme)|string||
|[spec.definition.applicationScopeMappings](#specdefinitionapplicationscopemappings)|object||
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.authenticationFlows[]](#specdefinitionauthenticationflows)|object||
|[spec.definition.authenticationFlows[].alias](#specdefinitionauthenticationflowsalias)|string||
|[spec.definition.authenticationFlows[].authenticationExecutions[]](#specdefinitionauthenticationflowsauthenticationexecutions)|object||
|[spec.definition.authenticationFlows[].authenticationExecutions[].authenticator](#specdefinitionauthenticationflowsauthenticationexecutionsauthenticator)|string||
|[spec.definition.authenticationFlows[].authenticationExecutions[].authenticatorConfig](#specdefinitionauthenticationflowsauthenticationexecutionsauthenticatorconfig)|string||
|[spec.definition.authenticationFlows[].authenticationExecutions[].authenticatorFlow](#specdefinitionauthenticationflowsauthenticationexecutionsauthenticatorflow)|boolean||
|[spec.definition.authenticationFlows[].authenticationExecutions[].autheticatorFlow](#specdefinitionauthenticationflowsauthenticationexecutionsautheticatorflow)|boolean||
|[spec.definition.authenticationFlows[].authenticationExecutions[].flowAlias](#specdefinitionauthenticationflowsauthenticationexecutionsflowalias)|string||
|[spec.definition.authenticationFlows[].authenticationExecutions[].priority](#specdefinitionauthenticationflowsauthenticationexecutionspriority)|integer||
|[spec.definition.authenticationFlows[].authenticationExecutions[].requirement](#specdefinitionauthenticationflowsauthenticationexecutionsrequirement)|string||
|[spec.definition.authenticationFlows[].authenticationExecutions[].userSetupAllowed](#specdefinitionauthenticationflowsauthenticationexecutionsusersetupallowed)|boolean||
|[spec.definition.authenticationFlows[].builtIn](#specdefinitionauthenticationflowsbuiltin)|boolean||
|[spec.definition.authenticationFlows[].description](#specdefinitionauthenticationflowsdescription)|string||
|[spec.definition.authenticationFlows[].id](#specdefinitionauthenticationflowsid)|string||
|[spec.definition.authenticationFlows[].providerId](#specdefinitionauthenticationflowsproviderid)|string||
|[spec.definition.authenticationFlows[].topLevel](#specdefinitionauthenticationflowstoplevel)|boolean||
|[spec.definition.authenticatorConfig[]](#specdefinitionauthenticatorconfig)|object||
|[spec.definition.authenticatorConfig[].alias](#specdefinitionauthenticatorconfigalias)|string||
|[spec.definition.authenticatorConfig[].config](#specdefinitionauthenticatorconfigconfig)|object||
|[spec.definition.authenticatorConfig[].id](#specdefinitionauthenticatorconfigid)|string||
|[spec.definition.browserFlow](#specdefinitionbrowserflow)|string||
|[spec.definition.browserSecurityHeaders](#specdefinitionbrowsersecurityheaders)|object||
|[spec.definition.bruteForceProtected](#specdefinitionbruteforceprotected)|boolean||
|[spec.definition.certificate](#specdefinitioncertificate)|string||
|[spec.definition.clientAuthenticationFlow](#specdefinitionclientauthenticationflow)|string||
|[spec.definition.clientOfflineSessionIdleTimeout](#specdefinitionclientofflinesessionidletimeout)|integer||
|[spec.definition.clientOfflineSessionMaxLifespan](#specdefinitionclientofflinesessionmaxlifespan)|integer||
|[spec.definition.clientPolicies](#specdefinitionclientpolicies)|object||
|[spec.definition.clientPolicies.globalPolicies[]](#specdefinitionclientpoliciesglobalpolicies)|object||
|[spec.definition.clientPolicies.globalPolicies[].conditions[]](#specdefinitionclientpoliciesglobalpoliciesconditions)|object||
|[spec.definition.clientPolicies.globalPolicies[].conditions[].condition](#specdefinitionclientpoliciesglobalpoliciesconditionscondition)|string||
|[spec.definition.clientPolicies.globalPolicies[].conditions[].configuration](#specdefinitionclientpoliciesglobalpoliciesconditionsconfiguration)|object||
|[spec.definition.clientPolicies.globalPolicies[].description](#specdefinitionclientpoliciesglobalpoliciesdescription)|string||
|[spec.definition.clientPolicies.globalPolicies[].enabled](#specdefinitionclientpoliciesglobalpoliciesenabled)|boolean||
|[spec.definition.clientPolicies.globalPolicies[].name](#specdefinitionclientpoliciesglobalpoliciesname)|string||
|[spec.definition.clientPolicies.globalPolicies[].profiles[]](#specdefinitionclientpoliciesglobalpoliciesprofiles)|string||
|[spec.definition.clientPolicies.policies[]](#specdefinitionclientpoliciespolicies)|object||
|[spec.definition.clientPolicies.policies[].conditions[]](#specdefinitionclientpoliciespoliciesconditions)|object||
|[spec.definition.clientPolicies.policies[].conditions[].condition](#specdefinitionclientpoliciespoliciesconditionscondition)|string||
|[spec.definition.clientPolicies.policies[].conditions[].configuration](#specdefinitionclientpoliciespoliciesconditionsconfiguration)|object||
|[spec.definition.clientPolicies.policies[].description](#specdefinitionclientpoliciespoliciesdescription)|string||
|[spec.definition.clientPolicies.policies[].enabled](#specdefinitionclientpoliciespoliciesenabled)|boolean||
|[spec.definition.clientPolicies.policies[].name](#specdefinitionclientpoliciespoliciesname)|string||
|[spec.definition.clientPolicies.policies[].profiles[]](#specdefinitionclientpoliciespoliciesprofiles)|string||
|[spec.definition.clientProfiles](#specdefinitionclientprofiles)|object||
|[spec.definition.clientProfiles.globalProfiles[]](#specdefinitionclientprofilesglobalprofiles)|object||
|[spec.definition.clientProfiles.globalProfiles[].description](#specdefinitionclientprofilesglobalprofilesdescription)|string||
|[spec.definition.clientProfiles.globalProfiles[].executors[]](#specdefinitionclientprofilesglobalprofilesexecutors)|object||
|[spec.definition.clientProfiles.globalProfiles[].executors[].configuration](#specdefinitionclientprofilesglobalprofilesexecutorsconfiguration)|object||
|[spec.definition.clientProfiles.globalProfiles[].executors[].executor](#specdefinitionclientprofilesglobalprofilesexecutorsexecutor)|string||
|[spec.definition.clientProfiles.globalProfiles[].name](#specdefinitionclientprofilesglobalprofilesname)|string||
|[spec.definition.clientProfiles.profiles[]](#specdefinitionclientprofilesprofiles)|object||
|[spec.definition.clientProfiles.profiles[].description](#specdefinitionclientprofilesprofilesdescription)|string||
|[spec.definition.clientProfiles.profiles[].executors[]](#specdefinitionclientprofilesprofilesexecutors)|object||
|[spec.definition.clientProfiles.profiles[].executors[].configuration](#specdefinitionclientprofilesprofilesexecutorsconfiguration)|object||
|[spec.definition.clientProfiles.profiles[].executors[].executor](#specdefinitionclientprofilesprofilesexecutorsexecutor)|string||
|[spec.definition.clientProfiles.profiles[].name](#specdefinitionclientprofilesprofilesname)|string||
|[spec.definition.clientScopeMappings](#specdefinitionclientscopemappings)|object||
|[spec.definition.clientScopes[]](#specdefinitionclientscopes)|object||
|[spec.definition.clientScopes[].attributes](#specdefinitionclientscopesattributes)|object||
|[spec.definition.clientScopes[].description](#specdefinitionclientscopesdescription)|string||
|[spec.definition.clientScopes[].id](#specdefinitionclientscopesid)|string||
|[spec.definition.clientScopes[].name](#specdefinitionclientscopesname)|string||
|[spec.definition.clientScopes[].protocol](#specdefinitionclientscopesprotocol)|string||
|[spec.definition.clientScopes[].protocolMappers[]](#specdefinitionclientscopesprotocolmappers)|object||
|[spec.definition.clientScopes[].protocolMappers[].config](#specdefinitionclientscopesprotocolmappersconfig)|object||
|[spec.definition.clientScopes[].protocolMappers[].consentRequired](#specdefinitionclientscopesprotocolmappersconsentrequired)|boolean||
|[spec.definition.clientScopes[].protocolMappers[].consentText](#specdefinitionclientscopesprotocolmappersconsenttext)|string||
|[spec.definition.clientScopes[].protocolMappers[].id](#specdefinitionclientscopesprotocolmappersid)|string||
|[spec.definition.clientScopes[].protocolMappers[].name](#specdefinitionclientscopesprotocolmappersname)|string||
|[spec.definition.clientScopes[].protocolMappers[].protocol](#specdefinitionclientscopesprotocolmappersprotocol)|string||
|[spec.definition.clientScopes[].protocolMappers[].protocolMapper](#specdefinitionclientscopesprotocolmappersprotocolmapper)|string||
|[spec.definition.clientSessionIdleTimeout](#specdefinitionclientsessionidletimeout)|integer||
|[spec.definition.clientSessionMaxLifespan](#specdefinitionclientsessionmaxlifespan)|integer||
|[spec.definition.clientTemplates[]](#specdefinitionclienttemplates)|object||
|[spec.definition.clientTemplates[].attributes](#specdefinitionclienttemplatesattributes)|object||
|[spec.definition.clientTemplates[].bearerOnly](#specdefinitionclienttemplatesbeareronly)|boolean||
|[spec.definition.clientTemplates[].consentRequired](#specdefinitionclienttemplatesconsentrequired)|boolean||
|[spec.definition.clientTemplates[].description](#specdefinitionclienttemplatesdescription)|string||
|[spec.definition.clientTemplates[].directAccessGrantsEnabled](#specdefinitionclienttemplatesdirectaccessgrantsenabled)|boolean||
|[spec.definition.clientTemplates[].frontchannelLogout](#specdefinitionclienttemplatesfrontchannellogout)|boolean||
|[spec.definition.clientTemplates[].fullScopeAllowed](#specdefinitionclienttemplatesfullscopeallowed)|boolean||
|[spec.definition.clientTemplates[].id](#specdefinitionclienttemplatesid)|string||
|[spec.definition.clientTemplates[].implicitFlowEnabled](#specdefinitionclienttemplatesimplicitflowenabled)|boolean||
|[spec.definition.clientTemplates[].name](#specdefinitionclienttemplatesname)|string||
|[spec.definition.clientTemplates[].protocol](#specdefinitionclienttemplatesprotocol)|string||
|[spec.definition.clientTemplates[].protocolMappers[]](#specdefinitionclienttemplatesprotocolmappers)|object||
|[spec.definition.clientTemplates[].protocolMappers[].config](#specdefinitionclienttemplatesprotocolmappersconfig)|object||
|[spec.definition.clientTemplates[].protocolMappers[].consentRequired](#specdefinitionclienttemplatesprotocolmappersconsentrequired)|boolean||
|[spec.definition.clientTemplates[].protocolMappers[].consentText](#specdefinitionclienttemplatesprotocolmappersconsenttext)|string||
|[spec.definition.clientTemplates[].protocolMappers[].id](#specdefinitionclienttemplatesprotocolmappersid)|string||
|[spec.definition.clientTemplates[].protocolMappers[].name](#specdefinitionclienttemplatesprotocolmappersname)|string||
|[spec.definition.clientTemplates[].protocolMappers[].protocol](#specdefinitionclienttemplatesprotocolmappersprotocol)|string||
|[spec.definition.clientTemplates[].protocolMappers[].protocolMapper](#specdefinitionclienttemplatesprotocolmappersprotocolmapper)|string||
|[spec.definition.clientTemplates[].publicClient](#specdefinitionclienttemplatespublicclient)|boolean||
|[spec.definition.clientTemplates[].serviceAccountsEnabled](#specdefinitionclienttemplatesserviceaccountsenabled)|boolean||
|[spec.definition.clientTemplates[].standardFlowEnabled](#specdefinitionclienttemplatesstandardflowenabled)|boolean||
|[spec.definition.codeSecret](#specdefinitioncodesecret)|string||
|[spec.definition.defaultDefaultClientScopes[]](#specdefinitiondefaultdefaultclientscopes)|string||
|[spec.definition.defaultGroups[]](#specdefinitiondefaultgroups)|string||
|[spec.definition.defaultLocale](#specdefinitiondefaultlocale)|string||
|[spec.definition.defaultOptionalClientScopes[]](#specdefinitiondefaultoptionalclientscopes)|string||
|[spec.definition.defaultRole](#specdefinitiondefaultrole)|object||
|[spec.definition.defaultRole.attributes](#specdefinitiondefaultroleattributes)|object||
|[spec.definition.defaultRole.clientRole](#specdefinitiondefaultroleclientrole)|boolean||
|[spec.definition.defaultRole.composite](#specdefinitiondefaultrolecomposite)|boolean||
|[spec.definition.defaultRole.composites](#specdefinitiondefaultrolecomposites)|object||
|[spec.definition.defaultRole.composites.application](#specdefinitiondefaultrolecompositesapplication)|object||
|[spec.definition.defaultRole.composites.client](#specdefinitiondefaultrolecompositesclient)|object||
|[spec.definition.defaultRole.composites.realm[]](#specdefinitiondefaultrolecompositesrealm)|string||
|[spec.definition.defaultRole.containerId](#specdefinitiondefaultrolecontainerid)|string||
|[spec.definition.defaultRole.description](#specdefinitiondefaultroledescription)|string||
|[spec.definition.defaultRole.id](#specdefinitiondefaultroleid)|string||
|[spec.definition.defaultRole.name](#specdefinitiondefaultrolename)|string||
|[spec.definition.defaultRole.scopeParamRequired](#specdefinitiondefaultrolescopeparamrequired)|boolean||
|[spec.definition.defaultRoles[]](#specdefinitiondefaultroles)|string||
|[spec.definition.defaultSignatureAlgorithm](#specdefinitiondefaultsignaturealgorithm)|string||
|[spec.definition.directGrantFlow](#specdefinitiondirectgrantflow)|string||
|[spec.definition.displayName](#specdefinitiondisplayname)|string||
|[spec.definition.displayNameHtml](#specdefinitiondisplaynamehtml)|string||
|[spec.definition.dockerAuthenticationFlow](#specdefinitiondockerauthenticationflow)|string||
|[spec.definition.duplicateEmailsAllowed](#specdefinitionduplicateemailsallowed)|boolean||
|[spec.definition.editUsernameAllowed](#specdefinitioneditusernameallowed)|boolean||
|[spec.definition.emailTheme](#specdefinitionemailtheme)|string||
|[spec.definition.enabled](#specdefinitionenabled)|boolean||
|[spec.definition.enabledEventTypes[]](#specdefinitionenabledeventtypes)|string||
|[spec.definition.eventsEnabled](#specdefinitioneventsenabled)|boolean||
|[spec.definition.eventsExpiration](#specdefinitioneventsexpiration)|integer||
|[spec.definition.eventsListeners[]](#specdefinitioneventslisteners)|string||
|[spec.definition.failureFactor](#specdefinitionfailurefactor)|integer||
|[spec.definition.federatedUsers[]](#specdefinitionfederatedusers)|object||
|[spec.definition.federatedUsers[].access](#specdefinitionfederatedusersaccess)|object||
|[spec.definition.federatedUsers[].applicationRoles](#specdefinitionfederatedusersapplicationroles)|object||
|[spec.definition.federatedUsers[].attributes](#specdefinitionfederatedusersattributes)|object||
|[spec.definition.federatedUsers[].clientConsents[]](#specdefinitionfederatedusersclientconsents)|object||
|[spec.definition.federatedUsers[].clientConsents[].clientId](#specdefinitionfederatedusersclientconsentsclientid)|string||
|[spec.definition.federatedUsers[].clientConsents[].createdDate](#specdefinitionfederatedusersclientconsentscreateddate)|integer||
|[spec.definition.federatedUsers[].clientConsents[].grantedClientScopes[]](#specdefinitionfederatedusersclientconsentsgrantedclientscopes)|string||
|[spec.definition.federatedUsers[].clientConsents[].grantedRealmRoles[]](#specdefinitionfederatedusersclientconsentsgrantedrealmroles)|string||
|[spec.definition.federatedUsers[].clientConsents[].lastUpdatedDate](#specdefinitionfederatedusersclientconsentslastupdateddate)|integer||
|[spec.definition.federatedUsers[].clientRoles](#specdefinitionfederatedusersclientroles)|object||
|[spec.definition.federatedUsers[].createdTimestamp](#specdefinitionfederateduserscreatedtimestamp)|integer||
|[spec.definition.federatedUsers[].credentials[]](#specdefinitionfederateduserscredentials)|object||
|[spec.definition.federatedUsers[].credentials[].algorithm](#specdefinitionfederateduserscredentialsalgorithm)|string||
|[spec.definition.federatedUsers[].credentials[].config](#specdefinitionfederateduserscredentialsconfig)|object||
|[spec.definition.federatedUsers[].credentials[].counter](#specdefinitionfederateduserscredentialscounter)|integer||
|[spec.definition.federatedUsers[].credentials[].createdDate](#specdefinitionfederateduserscredentialscreateddate)|integer||
|[spec.definition.federatedUsers[].credentials[].credentialData](#specdefinitionfederateduserscredentialscredentialdata)|string||
|[spec.definition.federatedUsers[].credentials[].device](#specdefinitionfederateduserscredentialsdevice)|string||
|[spec.definition.federatedUsers[].credentials[].digits](#specdefinitionfederateduserscredentialsdigits)|integer||
|[spec.definition.federatedUsers[].credentials[].hashIterations](#specdefinitionfederateduserscredentialshashiterations)|integer||
|[spec.definition.federatedUsers[].credentials[].hashedSaltedValue](#specdefinitionfederateduserscredentialshashedsaltedvalue)|string||
|[spec.definition.federatedUsers[].credentials[].id](#specdefinitionfederateduserscredentialsid)|string||
|[spec.definition.federatedUsers[].credentials[].period](#specdefinitionfederateduserscredentialsperiod)|integer||
|[spec.definition.federatedUsers[].credentials[].priority](#specdefinitionfederateduserscredentialspriority)|integer||
|[spec.definition.federatedUsers[].credentials[].salt](#specdefinitionfederateduserscredentialssalt)|string||
|[spec.definition.federatedUsers[].credentials[].secretData](#specdefinitionfederateduserscredentialssecretdata)|string||
|[spec.definition.federatedUsers[].credentials[].temporary](#specdefinitionfederateduserscredentialstemporary)|boolean||
|[spec.definition.federatedUsers[].credentials[].type](#specdefinitionfederateduserscredentialstype)|string||
|[spec.definition.federatedUsers[].credentials[].userLabel](#specdefinitionfederateduserscredentialsuserlabel)|string||
|[spec.definition.federatedUsers[].credentials[].value](#specdefinitionfederateduserscredentialsvalue)|string||
|[spec.definition.federatedUsers[].disableableCredentialTypes[]](#specdefinitionfederatedusersdisableablecredentialtypes)|string||
|[spec.definition.federatedUsers[].email](#specdefinitionfederatedusersemail)|string||
|[spec.definition.federatedUsers[].emailVerified](#specdefinitionfederatedusersemailverified)|boolean||
|[spec.definition.federatedUsers[].enabled](#specdefinitionfederatedusersenabled)|boolean||
|[spec.definition.federatedUsers[].federatedIdentities[]](#specdefinitionfederatedusersfederatedidentities)|object||
|[spec.definition.federatedUsers[].federatedIdentities[].identityProvider](#specdefinitionfederatedusersfederatedidentitiesidentityprovider)|string||
|[spec.definition.federatedUsers[].federatedIdentities[].userId](#specdefinitionfederatedusersfederatedidentitiesuserid)|string||
|[spec.definition.federatedUsers[].federatedIdentities[].userName](#specdefinitionfederatedusersfederatedidentitiesusername)|string||
|[spec.definition.federatedUsers[].federationLink](#specdefinitionfederatedusersfederationlink)|string||
|[spec.definition.federatedUsers[].firstName](#specdefinitionfederatedusersfirstname)|string||
|[spec.definition.federatedUsers[].groups[]](#specdefinitionfederatedusersgroups)|string||
|[spec.definition.federatedUsers[].id](#specdefinitionfederatedusersid)|string||
|[spec.definition.federatedUsers[].lastName](#specdefinitionfederateduserslastname)|string||
|[spec.definition.federatedUsers[].notBefore](#specdefinitionfederatedusersnotbefore)|integer||
|[spec.definition.federatedUsers[].origin](#specdefinitionfederatedusersorigin)|string||
|[spec.definition.federatedUsers[].realmRoles[]](#specdefinitionfederatedusersrealmroles)|string||
|[spec.definition.federatedUsers[].requiredActions[]](#specdefinitionfederatedusersrequiredactions)|string||
|[spec.definition.federatedUsers[].self](#specdefinitionfederatedusersself)|string||
|[spec.definition.federatedUsers[].serviceAccountClientId](#specdefinitionfederatedusersserviceaccountclientid)|string||
|[spec.definition.federatedUsers[].socialLinks[]](#specdefinitionfederateduserssociallinks)|object||
|[spec.definition.federatedUsers[].socialLinks[].socialProvider](#specdefinitionfederateduserssociallinkssocialprovider)|string||
|[spec.definition.federatedUsers[].socialLinks[].socialUserId](#specdefinitionfederateduserssociallinkssocialuserid)|string||
|[spec.definition.federatedUsers[].socialLinks[].socialUsername](#specdefinitionfederateduserssociallinkssocialusername)|string||
|[spec.definition.federatedUsers[].totp](#specdefinitionfederateduserstotp)|boolean||
|[spec.definition.federatedUsers[].userProfileMetadata](#specdefinitionfederatedusersuserprofilemetadata)|object||
|[spec.definition.federatedUsers[].userProfileMetadata.attributes[]](#specdefinitionfederatedusersuserprofilemetadataattributes)|object||
|[spec.definition.federatedUsers[].userProfileMetadata.attributes[].annotations](#specdefinitionfederatedusersuserprofilemetadataattributesannotations)|object||
|[spec.definition.federatedUsers[].userProfileMetadata.attributes[].displayName](#specdefinitionfederatedusersuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.federatedUsers[].userProfileMetadata.attributes[].group](#specdefinitionfederatedusersuserprofilemetadataattributesgroup)|string||
|[spec.definition.federatedUsers[].userProfileMetadata.attributes[].multivalued](#specdefinitionfederatedusersuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.federatedUsers[].userProfileMetadata.attributes[].name](#specdefinitionfederatedusersuserprofilemetadataattributesname)|string||
|[spec.definition.federatedUsers[].userProfileMetadata.attributes[].readOnly](#specdefinitionfederatedusersuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.federatedUsers[].userProfileMetadata.attributes[].required](#specdefinitionfederatedusersuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.federatedUsers[].userProfileMetadata.attributes[].validators](#specdefinitionfederatedusersuserprofilemetadataattributesvalidators)|object||
|[spec.definition.federatedUsers[].userProfileMetadata.groups[]](#specdefinitionfederatedusersuserprofilemetadatagroups)|object||
|[spec.definition.federatedUsers[].userProfileMetadata.groups[].annotations](#specdefinitionfederatedusersuserprofilemetadatagroupsannotations)|object||
|[spec.definition.federatedUsers[].userProfileMetadata.groups[].displayDescription](#specdefinitionfederatedusersuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.federatedUsers[].userProfileMetadata.groups[].displayHeader](#specdefinitionfederatedusersuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.federatedUsers[].userProfileMetadata.groups[].name](#specdefinitionfederatedusersuserprofilemetadatagroupsname)|string||
|[spec.definition.federatedUsers[].username](#specdefinitionfederatedusersusername)|string||
|[spec.definition.firstBrokerLoginFlow](#specdefinitionfirstbrokerloginflow)|string||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.identityProviderMappers[]](#specdefinitionidentityprovidermappers)|object||
|[spec.definition.identityProviderMappers[].config](#specdefinitionidentityprovidermappersconfig)|object||
|[spec.definition.identityProviderMappers[].id](#specdefinitionidentityprovidermappersid)|string||
|[spec.definition.identityProviderMappers[].identityProviderAlias](#specdefinitionidentityprovidermappersidentityprovideralias)|string||
|[spec.definition.identityProviderMappers[].identityProviderMapper](#specdefinitionidentityprovidermappersidentityprovidermapper)|string||
|[spec.definition.identityProviderMappers[].name](#specdefinitionidentityprovidermappersname)|string||
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
|[spec.definition.internationalizationEnabled](#specdefinitioninternationalizationenabled)|boolean||
|[spec.definition.keycloakVersion](#specdefinitionkeycloakversion)|string||
|[spec.definition.localizationTexts](#specdefinitionlocalizationtexts)|object||
|[spec.definition.loginTheme](#specdefinitionlogintheme)|string||
|[spec.definition.loginWithEmailAllowed](#specdefinitionloginwithemailallowed)|boolean||
|[spec.definition.maxDeltaTimeSeconds](#specdefinitionmaxdeltatimeseconds)|integer||
|[spec.definition.maxFailureWaitSeconds](#specdefinitionmaxfailurewaitseconds)|integer||
|[spec.definition.maxTemporaryLockouts](#specdefinitionmaxtemporarylockouts)|integer||
|[spec.definition.minimumQuickLoginWaitSeconds](#specdefinitionminimumquickloginwaitseconds)|integer||
|[spec.definition.notBefore](#specdefinitionnotbefore)|integer||
|[spec.definition.oAuth2DeviceCodeLifespan](#specdefinitionoauth2devicecodelifespan)|integer||
|[spec.definition.oAuth2DevicePollingInterval](#specdefinitionoauth2devicepollinginterval)|integer||
|[spec.definition.oauth2DeviceCodeLifespan](#specdefinitionoauth2devicecodelifespan)|integer||
|[spec.definition.oauth2DevicePollingInterval](#specdefinitionoauth2devicepollinginterval)|integer||
|[spec.definition.offlineSessionIdleTimeout](#specdefinitionofflinesessionidletimeout)|integer||
|[spec.definition.offlineSessionMaxLifespan](#specdefinitionofflinesessionmaxlifespan)|integer||
|[spec.definition.offlineSessionMaxLifespanEnabled](#specdefinitionofflinesessionmaxlifespanenabled)|boolean||
|[spec.definition.organizations[]](#specdefinitionorganizations)|object||
|[spec.definition.organizations[].alias](#specdefinitionorganizationsalias)|string||
|[spec.definition.organizations[].attributes](#specdefinitionorganizationsattributes)|object||
|[spec.definition.organizations[].description](#specdefinitionorganizationsdescription)|string||
|[spec.definition.organizations[].domains[]](#specdefinitionorganizationsdomains)|object||
|[spec.definition.organizations[].domains[].name](#specdefinitionorganizationsdomainsname)|string||
|[spec.definition.organizations[].domains[].verified](#specdefinitionorganizationsdomainsverified)|boolean||
|[spec.definition.organizations[].enabled](#specdefinitionorganizationsenabled)|boolean||
|[spec.definition.organizations[].id](#specdefinitionorganizationsid)|string||
|[spec.definition.organizations[].identityProviders[]](#specdefinitionorganizationsidentityproviders)|object||
|[spec.definition.organizations[].identityProviders[].addReadTokenRoleOnCreate](#specdefinitionorganizationsidentityprovidersaddreadtokenroleoncreate)|boolean||
|[spec.definition.organizations[].identityProviders[].alias](#specdefinitionorganizationsidentityprovidersalias)|string||
|[spec.definition.organizations[].identityProviders[].authenticateByDefault](#specdefinitionorganizationsidentityprovidersauthenticatebydefault)|boolean||
|[spec.definition.organizations[].identityProviders[].config](#specdefinitionorganizationsidentityprovidersconfig)|object||
|[spec.definition.organizations[].identityProviders[].displayName](#specdefinitionorganizationsidentityprovidersdisplayname)|string||
|[spec.definition.organizations[].identityProviders[].enabled](#specdefinitionorganizationsidentityprovidersenabled)|boolean||
|[spec.definition.organizations[].identityProviders[].firstBrokerLoginFlowAlias](#specdefinitionorganizationsidentityprovidersfirstbrokerloginflowalias)|string||
|[spec.definition.organizations[].identityProviders[].hideOnLogin](#specdefinitionorganizationsidentityprovidershideonlogin)|boolean||
|[spec.definition.organizations[].identityProviders[].internalId](#specdefinitionorganizationsidentityprovidersinternalid)|string||
|[spec.definition.organizations[].identityProviders[].linkOnly](#specdefinitionorganizationsidentityproviderslinkonly)|boolean||
|[spec.definition.organizations[].identityProviders[].organizationId](#specdefinitionorganizationsidentityprovidersorganizationid)|string||
|[spec.definition.organizations[].identityProviders[].postBrokerLoginFlowAlias](#specdefinitionorganizationsidentityproviderspostbrokerloginflowalias)|string||
|[spec.definition.organizations[].identityProviders[].providerId](#specdefinitionorganizationsidentityprovidersproviderid)|string||
|[spec.definition.organizations[].identityProviders[].storeToken](#specdefinitionorganizationsidentityprovidersstoretoken)|boolean||
|[spec.definition.organizations[].identityProviders[].trustEmail](#specdefinitionorganizationsidentityproviderstrustemail)|boolean||
|[spec.definition.organizations[].identityProviders[].updateProfileFirstLogin](#specdefinitionorganizationsidentityprovidersupdateprofilefirstlogin)|boolean||
|[spec.definition.organizations[].identityProviders[].updateProfileFirstLoginMode](#specdefinitionorganizationsidentityprovidersupdateprofilefirstloginmode)|string||
|[spec.definition.organizations[].members[]](#specdefinitionorganizationsmembers)|object||
|[spec.definition.organizations[].members[].access](#specdefinitionorganizationsmembersaccess)|object||
|[spec.definition.organizations[].members[].applicationRoles](#specdefinitionorganizationsmembersapplicationroles)|object||
|[spec.definition.organizations[].members[].attributes](#specdefinitionorganizationsmembersattributes)|object||
|[spec.definition.organizations[].members[].clientConsents[]](#specdefinitionorganizationsmembersclientconsents)|object||
|[spec.definition.organizations[].members[].clientConsents[].clientId](#specdefinitionorganizationsmembersclientconsentsclientid)|string||
|[spec.definition.organizations[].members[].clientConsents[].createdDate](#specdefinitionorganizationsmembersclientconsentscreateddate)|integer||
|[spec.definition.organizations[].members[].clientConsents[].grantedClientScopes[]](#specdefinitionorganizationsmembersclientconsentsgrantedclientscopes)|string||
|[spec.definition.organizations[].members[].clientConsents[].grantedRealmRoles[]](#specdefinitionorganizationsmembersclientconsentsgrantedrealmroles)|string||
|[spec.definition.organizations[].members[].clientConsents[].lastUpdatedDate](#specdefinitionorganizationsmembersclientconsentslastupdateddate)|integer||
|[spec.definition.organizations[].members[].clientRoles](#specdefinitionorganizationsmembersclientroles)|object||
|[spec.definition.organizations[].members[].createdTimestamp](#specdefinitionorganizationsmemberscreatedtimestamp)|integer||
|[spec.definition.organizations[].members[].credentials[]](#specdefinitionorganizationsmemberscredentials)|object||
|[spec.definition.organizations[].members[].credentials[].algorithm](#specdefinitionorganizationsmemberscredentialsalgorithm)|string||
|[spec.definition.organizations[].members[].credentials[].config](#specdefinitionorganizationsmemberscredentialsconfig)|object||
|[spec.definition.organizations[].members[].credentials[].counter](#specdefinitionorganizationsmemberscredentialscounter)|integer||
|[spec.definition.organizations[].members[].credentials[].createdDate](#specdefinitionorganizationsmemberscredentialscreateddate)|integer||
|[spec.definition.organizations[].members[].credentials[].credentialData](#specdefinitionorganizationsmemberscredentialscredentialdata)|string||
|[spec.definition.organizations[].members[].credentials[].device](#specdefinitionorganizationsmemberscredentialsdevice)|string||
|[spec.definition.organizations[].members[].credentials[].digits](#specdefinitionorganizationsmemberscredentialsdigits)|integer||
|[spec.definition.organizations[].members[].credentials[].hashIterations](#specdefinitionorganizationsmemberscredentialshashiterations)|integer||
|[spec.definition.organizations[].members[].credentials[].hashedSaltedValue](#specdefinitionorganizationsmemberscredentialshashedsaltedvalue)|string||
|[spec.definition.organizations[].members[].credentials[].id](#specdefinitionorganizationsmemberscredentialsid)|string||
|[spec.definition.organizations[].members[].credentials[].period](#specdefinitionorganizationsmemberscredentialsperiod)|integer||
|[spec.definition.organizations[].members[].credentials[].priority](#specdefinitionorganizationsmemberscredentialspriority)|integer||
|[spec.definition.organizations[].members[].credentials[].salt](#specdefinitionorganizationsmemberscredentialssalt)|string||
|[spec.definition.organizations[].members[].credentials[].secretData](#specdefinitionorganizationsmemberscredentialssecretdata)|string||
|[spec.definition.organizations[].members[].credentials[].temporary](#specdefinitionorganizationsmemberscredentialstemporary)|boolean||
|[spec.definition.organizations[].members[].credentials[].type](#specdefinitionorganizationsmemberscredentialstype)|string||
|[spec.definition.organizations[].members[].credentials[].userLabel](#specdefinitionorganizationsmemberscredentialsuserlabel)|string||
|[spec.definition.organizations[].members[].credentials[].value](#specdefinitionorganizationsmemberscredentialsvalue)|string||
|[spec.definition.organizations[].members[].disableableCredentialTypes[]](#specdefinitionorganizationsmembersdisableablecredentialtypes)|string||
|[spec.definition.organizations[].members[].email](#specdefinitionorganizationsmembersemail)|string||
|[spec.definition.organizations[].members[].emailVerified](#specdefinitionorganizationsmembersemailverified)|boolean||
|[spec.definition.organizations[].members[].enabled](#specdefinitionorganizationsmembersenabled)|boolean||
|[spec.definition.organizations[].members[].federatedIdentities[]](#specdefinitionorganizationsmembersfederatedidentities)|object||
|[spec.definition.organizations[].members[].federatedIdentities[].identityProvider](#specdefinitionorganizationsmembersfederatedidentitiesidentityprovider)|string||
|[spec.definition.organizations[].members[].federatedIdentities[].userId](#specdefinitionorganizationsmembersfederatedidentitiesuserid)|string||
|[spec.definition.organizations[].members[].federatedIdentities[].userName](#specdefinitionorganizationsmembersfederatedidentitiesusername)|string||
|[spec.definition.organizations[].members[].federationLink](#specdefinitionorganizationsmembersfederationlink)|string||
|[spec.definition.organizations[].members[].firstName](#specdefinitionorganizationsmembersfirstname)|string||
|[spec.definition.organizations[].members[].groups[]](#specdefinitionorganizationsmembersgroups)|string||
|[spec.definition.organizations[].members[].id](#specdefinitionorganizationsmembersid)|string||
|[spec.definition.organizations[].members[].lastName](#specdefinitionorganizationsmemberslastname)|string||
|[spec.definition.organizations[].members[].membershipType](#specdefinitionorganizationsmembersmembershiptype)|string||
|[spec.definition.organizations[].members[].notBefore](#specdefinitionorganizationsmembersnotbefore)|integer||
|[spec.definition.organizations[].members[].origin](#specdefinitionorganizationsmembersorigin)|string||
|[spec.definition.organizations[].members[].realmRoles[]](#specdefinitionorganizationsmembersrealmroles)|string||
|[spec.definition.organizations[].members[].requiredActions[]](#specdefinitionorganizationsmembersrequiredactions)|string||
|[spec.definition.organizations[].members[].self](#specdefinitionorganizationsmembersself)|string||
|[spec.definition.organizations[].members[].serviceAccountClientId](#specdefinitionorganizationsmembersserviceaccountclientid)|string||
|[spec.definition.organizations[].members[].socialLinks[]](#specdefinitionorganizationsmemberssociallinks)|object||
|[spec.definition.organizations[].members[].socialLinks[].socialProvider](#specdefinitionorganizationsmemberssociallinkssocialprovider)|string||
|[spec.definition.organizations[].members[].socialLinks[].socialUserId](#specdefinitionorganizationsmemberssociallinkssocialuserid)|string||
|[spec.definition.organizations[].members[].socialLinks[].socialUsername](#specdefinitionorganizationsmemberssociallinkssocialusername)|string||
|[spec.definition.organizations[].members[].totp](#specdefinitionorganizationsmemberstotp)|boolean||
|[spec.definition.organizations[].members[].userProfileMetadata](#specdefinitionorganizationsmembersuserprofilemetadata)|object||
|[spec.definition.organizations[].members[].userProfileMetadata.attributes[]](#specdefinitionorganizationsmembersuserprofilemetadataattributes)|object||
|[spec.definition.organizations[].members[].userProfileMetadata.attributes[].annotations](#specdefinitionorganizationsmembersuserprofilemetadataattributesannotations)|object||
|[spec.definition.organizations[].members[].userProfileMetadata.attributes[].displayName](#specdefinitionorganizationsmembersuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.organizations[].members[].userProfileMetadata.attributes[].group](#specdefinitionorganizationsmembersuserprofilemetadataattributesgroup)|string||
|[spec.definition.organizations[].members[].userProfileMetadata.attributes[].multivalued](#specdefinitionorganizationsmembersuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.organizations[].members[].userProfileMetadata.attributes[].name](#specdefinitionorganizationsmembersuserprofilemetadataattributesname)|string||
|[spec.definition.organizations[].members[].userProfileMetadata.attributes[].readOnly](#specdefinitionorganizationsmembersuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.organizations[].members[].userProfileMetadata.attributes[].required](#specdefinitionorganizationsmembersuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.organizations[].members[].userProfileMetadata.attributes[].validators](#specdefinitionorganizationsmembersuserprofilemetadataattributesvalidators)|object||
|[spec.definition.organizations[].members[].userProfileMetadata.groups[]](#specdefinitionorganizationsmembersuserprofilemetadatagroups)|object||
|[spec.definition.organizations[].members[].userProfileMetadata.groups[].annotations](#specdefinitionorganizationsmembersuserprofilemetadatagroupsannotations)|object||
|[spec.definition.organizations[].members[].userProfileMetadata.groups[].displayDescription](#specdefinitionorganizationsmembersuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.organizations[].members[].userProfileMetadata.groups[].displayHeader](#specdefinitionorganizationsmembersuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.organizations[].members[].userProfileMetadata.groups[].name](#specdefinitionorganizationsmembersuserprofilemetadatagroupsname)|string||
|[spec.definition.organizations[].members[].username](#specdefinitionorganizationsmembersusername)|string||
|[spec.definition.organizations[].name](#specdefinitionorganizationsname)|string||
|[spec.definition.organizations[].redirectUrl](#specdefinitionorganizationsredirecturl)|string||
|[spec.definition.organizationsEnabled](#specdefinitionorganizationsenabled)|boolean||
|[spec.definition.otpPolicyAlgorithm](#specdefinitionotppolicyalgorithm)|string||
|[spec.definition.otpPolicyCodeReusable](#specdefinitionotppolicycodereusable)|boolean||
|[spec.definition.otpPolicyDigits](#specdefinitionotppolicydigits)|integer||
|[spec.definition.otpPolicyInitialCounter](#specdefinitionotppolicyinitialcounter)|integer||
|[spec.definition.otpPolicyLookAheadWindow](#specdefinitionotppolicylookaheadwindow)|integer||
|[spec.definition.otpPolicyPeriod](#specdefinitionotppolicyperiod)|integer||
|[spec.definition.otpPolicyType](#specdefinitionotppolicytype)|string||
|[spec.definition.otpSupportedApplications[]](#specdefinitionotpsupportedapplications)|string||
|[spec.definition.passwordCredentialGrantAllowed](#specdefinitionpasswordcredentialgrantallowed)|boolean||
|[spec.definition.passwordPolicy](#specdefinitionpasswordpolicy)|string||
|[spec.definition.permanentLockout](#specdefinitionpermanentlockout)|boolean||
|[spec.definition.privateKey](#specdefinitionprivatekey)|string||
|[spec.definition.protocolMappers[]](#specdefinitionprotocolmappers)|object||
|[spec.definition.protocolMappers[].config](#specdefinitionprotocolmappersconfig)|object||
|[spec.definition.protocolMappers[].consentRequired](#specdefinitionprotocolmappersconsentrequired)|boolean||
|[spec.definition.protocolMappers[].consentText](#specdefinitionprotocolmappersconsenttext)|string||
|[spec.definition.protocolMappers[].id](#specdefinitionprotocolmappersid)|string||
|[spec.definition.protocolMappers[].name](#specdefinitionprotocolmappersname)|string||
|[spec.definition.protocolMappers[].protocol](#specdefinitionprotocolmappersprotocol)|string||
|[spec.definition.protocolMappers[].protocolMapper](#specdefinitionprotocolmappersprotocolmapper)|string||
|[spec.definition.publicKey](#specdefinitionpublickey)|string||
|[spec.definition.quickLoginCheckMilliSeconds](#specdefinitionquicklogincheckmilliseconds)|integer||
|[spec.definition.realm](#specdefinitionrealm)|string||
|[spec.definition.realmCacheEnabled](#specdefinitionrealmcacheenabled)|boolean||
|[spec.definition.refreshTokenMaxReuse](#specdefinitionrefreshtokenmaxreuse)|integer||
|[spec.definition.registrationAllowed](#specdefinitionregistrationallowed)|boolean||
|[spec.definition.registrationEmailAsUsername](#specdefinitionregistrationemailasusername)|boolean||
|[spec.definition.registrationFlow](#specdefinitionregistrationflow)|string||
|[spec.definition.rememberMe](#specdefinitionrememberme)|boolean||
|[spec.definition.requiredActions[]](#specdefinitionrequiredactions)|object||
|[spec.definition.requiredActions[].alias](#specdefinitionrequiredactionsalias)|string||
|[spec.definition.requiredActions[].config](#specdefinitionrequiredactionsconfig)|object||
|[spec.definition.requiredActions[].defaultAction](#specdefinitionrequiredactionsdefaultaction)|boolean||
|[spec.definition.requiredActions[].enabled](#specdefinitionrequiredactionsenabled)|boolean||
|[spec.definition.requiredActions[].name](#specdefinitionrequiredactionsname)|string||
|[spec.definition.requiredActions[].priority](#specdefinitionrequiredactionspriority)|integer||
|[spec.definition.requiredActions[].providerId](#specdefinitionrequiredactionsproviderid)|string||
|[spec.definition.requiredCredentials[]](#specdefinitionrequiredcredentials)|string||
|[spec.definition.resetCredentialsFlow](#specdefinitionresetcredentialsflow)|string||
|[spec.definition.resetPasswordAllowed](#specdefinitionresetpasswordallowed)|boolean||
|[spec.definition.revokeRefreshToken](#specdefinitionrevokerefreshtoken)|boolean||
|[spec.definition.roles](#specdefinitionroles)|object||
|[spec.definition.roles.application](#specdefinitionrolesapplication)|object||
|[spec.definition.roles.client](#specdefinitionrolesclient)|object||
|[spec.definition.roles.realm[]](#specdefinitionrolesrealm)|object||
|[spec.definition.roles.realm[].attributes](#specdefinitionrolesrealmattributes)|object||
|[spec.definition.roles.realm[].clientRole](#specdefinitionrolesrealmclientrole)|boolean||
|[spec.definition.roles.realm[].composite](#specdefinitionrolesrealmcomposite)|boolean||
|[spec.definition.roles.realm[].composites](#specdefinitionrolesrealmcomposites)|object||
|[spec.definition.roles.realm[].composites.application](#specdefinitionrolesrealmcompositesapplication)|object||
|[spec.definition.roles.realm[].composites.client](#specdefinitionrolesrealmcompositesclient)|object||
|[spec.definition.roles.realm[].composites.realm[]](#specdefinitionrolesrealmcompositesrealm)|string||
|[spec.definition.roles.realm[].containerId](#specdefinitionrolesrealmcontainerid)|string||
|[spec.definition.roles.realm[].description](#specdefinitionrolesrealmdescription)|string||
|[spec.definition.roles.realm[].id](#specdefinitionrolesrealmid)|string||
|[spec.definition.roles.realm[].name](#specdefinitionrolesrealmname)|string||
|[spec.definition.roles.realm[].scopeParamRequired](#specdefinitionrolesrealmscopeparamrequired)|boolean||
|[spec.definition.scopeMappings[]](#specdefinitionscopemappings)|object||
|[spec.definition.scopeMappings[].client](#specdefinitionscopemappingsclient)|string||
|[spec.definition.scopeMappings[].clientScope](#specdefinitionscopemappingsclientscope)|string||
|[spec.definition.scopeMappings[].clientTemplate](#specdefinitionscopemappingsclienttemplate)|string||
|[spec.definition.scopeMappings[].roles[]](#specdefinitionscopemappingsroles)|string||
|[spec.definition.scopeMappings[].self](#specdefinitionscopemappingsself)|string||
|[spec.definition.smtpServer](#specdefinitionsmtpserver)|object||
|[spec.definition.social](#specdefinitionsocial)|boolean||
|[spec.definition.socialProviders](#specdefinitionsocialproviders)|object||
|[spec.definition.sslRequired](#specdefinitionsslrequired)|string||
|[spec.definition.ssoSessionIdleTimeout](#specdefinitionssosessionidletimeout)|integer||
|[spec.definition.ssoSessionIdleTimeoutRememberMe](#specdefinitionssosessionidletimeoutrememberme)|integer||
|[spec.definition.ssoSessionMaxLifespan](#specdefinitionssosessionmaxlifespan)|integer||
|[spec.definition.ssoSessionMaxLifespanRememberMe](#specdefinitionssosessionmaxlifespanrememberme)|integer||
|[spec.definition.supportedLocales[]](#specdefinitionsupportedlocales)|string||
|[spec.definition.updateProfileOnInitialSocialLogin](#specdefinitionupdateprofileoninitialsociallogin)|boolean||
|[spec.definition.userCacheEnabled](#specdefinitionusercacheenabled)|boolean||
|[spec.definition.userFederationMappers[]](#specdefinitionuserfederationmappers)|object||
|[spec.definition.userFederationMappers[].config](#specdefinitionuserfederationmappersconfig)|object||
|[spec.definition.userFederationMappers[].federationMapperType](#specdefinitionuserfederationmappersfederationmappertype)|string||
|[spec.definition.userFederationMappers[].federationProviderDisplayName](#specdefinitionuserfederationmappersfederationproviderdisplayname)|string||
|[spec.definition.userFederationMappers[].id](#specdefinitionuserfederationmappersid)|string||
|[spec.definition.userFederationMappers[].name](#specdefinitionuserfederationmappersname)|string||
|[spec.definition.userFederationProviders[]](#specdefinitionuserfederationproviders)|object||
|[spec.definition.userFederationProviders[].changedSyncPeriod](#specdefinitionuserfederationproviderschangedsyncperiod)|integer||
|[spec.definition.userFederationProviders[].config](#specdefinitionuserfederationprovidersconfig)|object||
|[spec.definition.userFederationProviders[].displayName](#specdefinitionuserfederationprovidersdisplayname)|string||
|[spec.definition.userFederationProviders[].fullSyncPeriod](#specdefinitionuserfederationprovidersfullsyncperiod)|integer||
|[spec.definition.userFederationProviders[].id](#specdefinitionuserfederationprovidersid)|string||
|[spec.definition.userFederationProviders[].lastSync](#specdefinitionuserfederationproviderslastsync)|integer||
|[spec.definition.userFederationProviders[].priority](#specdefinitionuserfederationproviderspriority)|integer||
|[spec.definition.userFederationProviders[].providerName](#specdefinitionuserfederationprovidersprovidername)|string||
|[spec.definition.userManagedAccessAllowed](#specdefinitionusermanagedaccessallowed)|boolean||
|[spec.definition.users[]](#specdefinitionusers)|object||
|[spec.definition.users[].access](#specdefinitionusersaccess)|object||
|[spec.definition.users[].applicationRoles](#specdefinitionusersapplicationroles)|object||
|[spec.definition.users[].attributes](#specdefinitionusersattributes)|object||
|[spec.definition.users[].clientConsents[]](#specdefinitionusersclientconsents)|object||
|[spec.definition.users[].clientConsents[].clientId](#specdefinitionusersclientconsentsclientid)|string||
|[spec.definition.users[].clientConsents[].createdDate](#specdefinitionusersclientconsentscreateddate)|integer||
|[spec.definition.users[].clientConsents[].grantedClientScopes[]](#specdefinitionusersclientconsentsgrantedclientscopes)|string||
|[spec.definition.users[].clientConsents[].grantedRealmRoles[]](#specdefinitionusersclientconsentsgrantedrealmroles)|string||
|[spec.definition.users[].clientConsents[].lastUpdatedDate](#specdefinitionusersclientconsentslastupdateddate)|integer||
|[spec.definition.users[].clientRoles](#specdefinitionusersclientroles)|object||
|[spec.definition.users[].createdTimestamp](#specdefinitionuserscreatedtimestamp)|integer||
|[spec.definition.users[].credentials[]](#specdefinitionuserscredentials)|object||
|[spec.definition.users[].credentials[].algorithm](#specdefinitionuserscredentialsalgorithm)|string||
|[spec.definition.users[].credentials[].config](#specdefinitionuserscredentialsconfig)|object||
|[spec.definition.users[].credentials[].counter](#specdefinitionuserscredentialscounter)|integer||
|[spec.definition.users[].credentials[].createdDate](#specdefinitionuserscredentialscreateddate)|integer||
|[spec.definition.users[].credentials[].credentialData](#specdefinitionuserscredentialscredentialdata)|string||
|[spec.definition.users[].credentials[].device](#specdefinitionuserscredentialsdevice)|string||
|[spec.definition.users[].credentials[].digits](#specdefinitionuserscredentialsdigits)|integer||
|[spec.definition.users[].credentials[].hashIterations](#specdefinitionuserscredentialshashiterations)|integer||
|[spec.definition.users[].credentials[].hashedSaltedValue](#specdefinitionuserscredentialshashedsaltedvalue)|string||
|[spec.definition.users[].credentials[].id](#specdefinitionuserscredentialsid)|string||
|[spec.definition.users[].credentials[].period](#specdefinitionuserscredentialsperiod)|integer||
|[spec.definition.users[].credentials[].priority](#specdefinitionuserscredentialspriority)|integer||
|[spec.definition.users[].credentials[].salt](#specdefinitionuserscredentialssalt)|string||
|[spec.definition.users[].credentials[].secretData](#specdefinitionuserscredentialssecretdata)|string||
|[spec.definition.users[].credentials[].temporary](#specdefinitionuserscredentialstemporary)|boolean||
|[spec.definition.users[].credentials[].type](#specdefinitionuserscredentialstype)|string||
|[spec.definition.users[].credentials[].userLabel](#specdefinitionuserscredentialsuserlabel)|string||
|[spec.definition.users[].credentials[].value](#specdefinitionuserscredentialsvalue)|string||
|[spec.definition.users[].disableableCredentialTypes[]](#specdefinitionusersdisableablecredentialtypes)|string||
|[spec.definition.users[].email](#specdefinitionusersemail)|string||
|[spec.definition.users[].emailVerified](#specdefinitionusersemailverified)|boolean||
|[spec.definition.users[].enabled](#specdefinitionusersenabled)|boolean||
|[spec.definition.users[].federatedIdentities[]](#specdefinitionusersfederatedidentities)|object||
|[spec.definition.users[].federatedIdentities[].identityProvider](#specdefinitionusersfederatedidentitiesidentityprovider)|string||
|[spec.definition.users[].federatedIdentities[].userId](#specdefinitionusersfederatedidentitiesuserid)|string||
|[spec.definition.users[].federatedIdentities[].userName](#specdefinitionusersfederatedidentitiesusername)|string||
|[spec.definition.users[].federationLink](#specdefinitionusersfederationlink)|string||
|[spec.definition.users[].firstName](#specdefinitionusersfirstname)|string||
|[spec.definition.users[].groups[]](#specdefinitionusersgroups)|string||
|[spec.definition.users[].id](#specdefinitionusersid)|string||
|[spec.definition.users[].lastName](#specdefinitionuserslastname)|string||
|[spec.definition.users[].notBefore](#specdefinitionusersnotbefore)|integer||
|[spec.definition.users[].origin](#specdefinitionusersorigin)|string||
|[spec.definition.users[].realmRoles[]](#specdefinitionusersrealmroles)|string||
|[spec.definition.users[].requiredActions[]](#specdefinitionusersrequiredactions)|string||
|[spec.definition.users[].self](#specdefinitionusersself)|string||
|[spec.definition.users[].serviceAccountClientId](#specdefinitionusersserviceaccountclientid)|string||
|[spec.definition.users[].socialLinks[]](#specdefinitionuserssociallinks)|object||
|[spec.definition.users[].socialLinks[].socialProvider](#specdefinitionuserssociallinkssocialprovider)|string||
|[spec.definition.users[].socialLinks[].socialUserId](#specdefinitionuserssociallinkssocialuserid)|string||
|[spec.definition.users[].socialLinks[].socialUsername](#specdefinitionuserssociallinkssocialusername)|string||
|[spec.definition.users[].totp](#specdefinitionuserstotp)|boolean||
|[spec.definition.users[].userProfileMetadata](#specdefinitionusersuserprofilemetadata)|object||
|[spec.definition.users[].userProfileMetadata.attributes[]](#specdefinitionusersuserprofilemetadataattributes)|object||
|[spec.definition.users[].userProfileMetadata.attributes[].annotations](#specdefinitionusersuserprofilemetadataattributesannotations)|object||
|[spec.definition.users[].userProfileMetadata.attributes[].displayName](#specdefinitionusersuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.users[].userProfileMetadata.attributes[].group](#specdefinitionusersuserprofilemetadataattributesgroup)|string||
|[spec.definition.users[].userProfileMetadata.attributes[].multivalued](#specdefinitionusersuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.users[].userProfileMetadata.attributes[].name](#specdefinitionusersuserprofilemetadataattributesname)|string||
|[spec.definition.users[].userProfileMetadata.attributes[].readOnly](#specdefinitionusersuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.users[].userProfileMetadata.attributes[].required](#specdefinitionusersuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.users[].userProfileMetadata.attributes[].validators](#specdefinitionusersuserprofilemetadataattributesvalidators)|object||
|[spec.definition.users[].userProfileMetadata.groups[]](#specdefinitionusersuserprofilemetadatagroups)|object||
|[spec.definition.users[].userProfileMetadata.groups[].annotations](#specdefinitionusersuserprofilemetadatagroupsannotations)|object||
|[spec.definition.users[].userProfileMetadata.groups[].displayDescription](#specdefinitionusersuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.users[].userProfileMetadata.groups[].displayHeader](#specdefinitionusersuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.users[].userProfileMetadata.groups[].name](#specdefinitionusersuserprofilemetadatagroupsname)|string||
|[spec.definition.users[].username](#specdefinitionusersusername)|string||
|[spec.definition.verifyEmail](#specdefinitionverifyemail)|boolean||
|[spec.definition.waitIncrementSeconds](#specdefinitionwaitincrementseconds)|integer||
|[spec.definition.webAuthnPolicyAcceptableAaguids[]](#specdefinitionwebauthnpolicyacceptableaaguids)|string||
|[spec.definition.webAuthnPolicyAttestationConveyancePreference](#specdefinitionwebauthnpolicyattestationconveyancepreference)|string||
|[spec.definition.webAuthnPolicyAuthenticatorAttachment](#specdefinitionwebauthnpolicyauthenticatorattachment)|string||
|[spec.definition.webAuthnPolicyAvoidSameAuthenticatorRegister](#specdefinitionwebauthnpolicyavoidsameauthenticatorregister)|boolean||
|[spec.definition.webAuthnPolicyCreateTimeout](#specdefinitionwebauthnpolicycreatetimeout)|integer||
|[spec.definition.webAuthnPolicyExtraOrigins[]](#specdefinitionwebauthnpolicyextraorigins)|string||
|[spec.definition.webAuthnPolicyPasswordlessAcceptableAaguids[]](#specdefinitionwebauthnpolicypasswordlessacceptableaaguids)|string||
|[spec.definition.webAuthnPolicyPasswordlessAttestationConveyancePreference](#specdefinitionwebauthnpolicypasswordlessattestationconveyancepreference)|string||
|[spec.definition.webAuthnPolicyPasswordlessAuthenticatorAttachment](#specdefinitionwebauthnpolicypasswordlessauthenticatorattachment)|string||
|[spec.definition.webAuthnPolicyPasswordlessAvoidSameAuthenticatorRegister](#specdefinitionwebauthnpolicypasswordlessavoidsameauthenticatorregister)|boolean||
|[spec.definition.webAuthnPolicyPasswordlessCreateTimeout](#specdefinitionwebauthnpolicypasswordlesscreatetimeout)|integer||
|[spec.definition.webAuthnPolicyPasswordlessExtraOrigins[]](#specdefinitionwebauthnpolicypasswordlessextraorigins)|string||
|[spec.definition.webAuthnPolicyPasswordlessRequireResidentKey](#specdefinitionwebauthnpolicypasswordlessrequireresidentkey)|string||
|[spec.definition.webAuthnPolicyPasswordlessRpEntityName](#specdefinitionwebauthnpolicypasswordlessrpentityname)|string||
|[spec.definition.webAuthnPolicyPasswordlessRpId](#specdefinitionwebauthnpolicypasswordlessrpid)|string||
|[spec.definition.webAuthnPolicyPasswordlessSignatureAlgorithms[]](#specdefinitionwebauthnpolicypasswordlesssignaturealgorithms)|string||
|[spec.definition.webAuthnPolicyPasswordlessUserVerificationRequirement](#specdefinitionwebauthnpolicypasswordlessuserverificationrequirement)|string||
|[spec.definition.webAuthnPolicyRequireResidentKey](#specdefinitionwebauthnpolicyrequireresidentkey)|string||
|[spec.definition.webAuthnPolicyRpEntityName](#specdefinitionwebauthnpolicyrpentityname)|string||
|[spec.definition.webAuthnPolicyRpId](#specdefinitionwebauthnpolicyrpid)|string||
|[spec.definition.webAuthnPolicySignatureAlgorithms[]](#specdefinitionwebauthnpolicysignaturealgorithms)|string||
|[spec.definition.webAuthnPolicyUserVerificationRequirement](#specdefinitionwebauthnpolicyuserverificationrequirement)|string||
|[spec.instanceRef](#specinstanceref)|string|✅|
|[spec.options](#specoptions)|object||
|[spec.patchFrom](#specpatchfrom)|object||
|[status](#status)|object||
|[status.conditions[]](#statusconditions)|object||
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

---

### spec

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[definition](#specdefinition)|object|✅|
|[instanceRef](#specinstanceref)|string|✅|
|[options](#specoptions)|object||
|[patchFrom](#specpatchfrom)|object||

the KeycloakRealm resource

---

### spec.definition

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[accessCodeLifespan](#specdefinitionaccesscodelifespan)|integer||
|[accessCodeLifespanLogin](#specdefinitionaccesscodelifespanlogin)|integer||
|[accessCodeLifespanUserAction](#specdefinitionaccesscodelifespanuseraction)|integer||
|[accessTokenLifespan](#specdefinitionaccesstokenlifespan)|integer||
|[accessTokenLifespanForImplicitFlow](#specdefinitionaccesstokenlifespanforimplicitflow)|integer||
|[accountTheme](#specdefinitionaccounttheme)|string||
|[actionTokenGeneratedByAdminLifespan](#specdefinitionactiontokengeneratedbyadminlifespan)|integer||
|[actionTokenGeneratedByUserLifespan](#specdefinitionactiontokengeneratedbyuserlifespan)|integer||
|[adminEventsDetailsEnabled](#specdefinitionadmineventsdetailsenabled)|boolean||
|[adminEventsEnabled](#specdefinitionadmineventsenabled)|boolean||
|[adminTheme](#specdefinitionadmintheme)|string||
|[applicationScopeMappings](#specdefinitionapplicationscopemappings)|object||
|[attributes](#specdefinitionattributes)|object||
|[authenticationFlows[]](#specdefinitionauthenticationflows)|object||
|[authenticatorConfig[]](#specdefinitionauthenticatorconfig)|object||
|[browserFlow](#specdefinitionbrowserflow)|string||
|[browserSecurityHeaders](#specdefinitionbrowsersecurityheaders)|object||
|[bruteForceProtected](#specdefinitionbruteforceprotected)|boolean||
|[certificate](#specdefinitioncertificate)|string||
|[clientAuthenticationFlow](#specdefinitionclientauthenticationflow)|string||
|[clientOfflineSessionIdleTimeout](#specdefinitionclientofflinesessionidletimeout)|integer||
|[clientOfflineSessionMaxLifespan](#specdefinitionclientofflinesessionmaxlifespan)|integer||
|[clientPolicies](#specdefinitionclientpolicies)|object||
|[clientProfiles](#specdefinitionclientprofiles)|object||
|[clientScopeMappings](#specdefinitionclientscopemappings)|object||
|[clientScopes[]](#specdefinitionclientscopes)|object||
|[clientSessionIdleTimeout](#specdefinitionclientsessionidletimeout)|integer||
|[clientSessionMaxLifespan](#specdefinitionclientsessionmaxlifespan)|integer||
|[clientTemplates[]](#specdefinitionclienttemplates)|object||
|[codeSecret](#specdefinitioncodesecret)|string||
|[defaultDefaultClientScopes[]](#specdefinitiondefaultdefaultclientscopes)|string||
|[defaultGroups[]](#specdefinitiondefaultgroups)|string||
|[defaultLocale](#specdefinitiondefaultlocale)|string||
|[defaultOptionalClientScopes[]](#specdefinitiondefaultoptionalclientscopes)|string||
|[defaultRole](#specdefinitiondefaultrole)|object||
|[defaultRoles[]](#specdefinitiondefaultroles)|string||
|[defaultSignatureAlgorithm](#specdefinitiondefaultsignaturealgorithm)|string||
|[directGrantFlow](#specdefinitiondirectgrantflow)|string||
|[displayName](#specdefinitiondisplayname)|string||
|[displayNameHtml](#specdefinitiondisplaynamehtml)|string||
|[dockerAuthenticationFlow](#specdefinitiondockerauthenticationflow)|string||
|[duplicateEmailsAllowed](#specdefinitionduplicateemailsallowed)|boolean||
|[editUsernameAllowed](#specdefinitioneditusernameallowed)|boolean||
|[emailTheme](#specdefinitionemailtheme)|string||
|[enabled](#specdefinitionenabled)|boolean||
|[enabledEventTypes[]](#specdefinitionenabledeventtypes)|string||
|[eventsEnabled](#specdefinitioneventsenabled)|boolean||
|[eventsExpiration](#specdefinitioneventsexpiration)|integer||
|[eventsListeners[]](#specdefinitioneventslisteners)|string||
|[failureFactor](#specdefinitionfailurefactor)|integer||
|[federatedUsers[]](#specdefinitionfederatedusers)|object||
|[firstBrokerLoginFlow](#specdefinitionfirstbrokerloginflow)|string||
|[id](#specdefinitionid)|string||
|[identityProviderMappers[]](#specdefinitionidentityprovidermappers)|object||
|[identityProviders[]](#specdefinitionidentityproviders)|object||
|[internationalizationEnabled](#specdefinitioninternationalizationenabled)|boolean||
|[keycloakVersion](#specdefinitionkeycloakversion)|string||
|[localizationTexts](#specdefinitionlocalizationtexts)|object||
|[loginTheme](#specdefinitionlogintheme)|string||
|[loginWithEmailAllowed](#specdefinitionloginwithemailallowed)|boolean||
|[maxDeltaTimeSeconds](#specdefinitionmaxdeltatimeseconds)|integer||
|[maxFailureWaitSeconds](#specdefinitionmaxfailurewaitseconds)|integer||
|[maxTemporaryLockouts](#specdefinitionmaxtemporarylockouts)|integer||
|[minimumQuickLoginWaitSeconds](#specdefinitionminimumquickloginwaitseconds)|integer||
|[notBefore](#specdefinitionnotbefore)|integer||
|[oAuth2DeviceCodeLifespan](#specdefinitionoauth2devicecodelifespan)|integer||
|[oAuth2DevicePollingInterval](#specdefinitionoauth2devicepollinginterval)|integer||
|[oauth2DeviceCodeLifespan](#specdefinitionoauth2devicecodelifespan)|integer||
|[oauth2DevicePollingInterval](#specdefinitionoauth2devicepollinginterval)|integer||
|[offlineSessionIdleTimeout](#specdefinitionofflinesessionidletimeout)|integer||
|[offlineSessionMaxLifespan](#specdefinitionofflinesessionmaxlifespan)|integer||
|[offlineSessionMaxLifespanEnabled](#specdefinitionofflinesessionmaxlifespanenabled)|boolean||
|[organizations[]](#specdefinitionorganizations)|object||
|[organizationsEnabled](#specdefinitionorganizationsenabled)|boolean||
|[otpPolicyAlgorithm](#specdefinitionotppolicyalgorithm)|string||
|[otpPolicyCodeReusable](#specdefinitionotppolicycodereusable)|boolean||
|[otpPolicyDigits](#specdefinitionotppolicydigits)|integer||
|[otpPolicyInitialCounter](#specdefinitionotppolicyinitialcounter)|integer||
|[otpPolicyLookAheadWindow](#specdefinitionotppolicylookaheadwindow)|integer||
|[otpPolicyPeriod](#specdefinitionotppolicyperiod)|integer||
|[otpPolicyType](#specdefinitionotppolicytype)|string||
|[otpSupportedApplications[]](#specdefinitionotpsupportedapplications)|string||
|[passwordCredentialGrantAllowed](#specdefinitionpasswordcredentialgrantallowed)|boolean||
|[passwordPolicy](#specdefinitionpasswordpolicy)|string||
|[permanentLockout](#specdefinitionpermanentlockout)|boolean||
|[privateKey](#specdefinitionprivatekey)|string||
|[protocolMappers[]](#specdefinitionprotocolmappers)|object||
|[publicKey](#specdefinitionpublickey)|string||
|[quickLoginCheckMilliSeconds](#specdefinitionquicklogincheckmilliseconds)|integer||
|[realm](#specdefinitionrealm)|string||
|[realmCacheEnabled](#specdefinitionrealmcacheenabled)|boolean||
|[refreshTokenMaxReuse](#specdefinitionrefreshtokenmaxreuse)|integer||
|[registrationAllowed](#specdefinitionregistrationallowed)|boolean||
|[registrationEmailAsUsername](#specdefinitionregistrationemailasusername)|boolean||
|[registrationFlow](#specdefinitionregistrationflow)|string||
|[rememberMe](#specdefinitionrememberme)|boolean||
|[requiredActions[]](#specdefinitionrequiredactions)|object||
|[requiredCredentials[]](#specdefinitionrequiredcredentials)|string||
|[resetCredentialsFlow](#specdefinitionresetcredentialsflow)|string||
|[resetPasswordAllowed](#specdefinitionresetpasswordallowed)|boolean||
|[revokeRefreshToken](#specdefinitionrevokerefreshtoken)|boolean||
|[roles](#specdefinitionroles)|object||
|[scopeMappings[]](#specdefinitionscopemappings)|object||
|[smtpServer](#specdefinitionsmtpserver)|object||
|[social](#specdefinitionsocial)|boolean||
|[socialProviders](#specdefinitionsocialproviders)|object||
|[sslRequired](#specdefinitionsslrequired)|string||
|[ssoSessionIdleTimeout](#specdefinitionssosessionidletimeout)|integer||
|[ssoSessionIdleTimeoutRememberMe](#specdefinitionssosessionidletimeoutrememberme)|integer||
|[ssoSessionMaxLifespan](#specdefinitionssosessionmaxlifespan)|integer||
|[ssoSessionMaxLifespanRememberMe](#specdefinitionssosessionmaxlifespanrememberme)|integer||
|[supportedLocales[]](#specdefinitionsupportedlocales)|string||
|[updateProfileOnInitialSocialLogin](#specdefinitionupdateprofileoninitialsociallogin)|boolean||
|[userCacheEnabled](#specdefinitionusercacheenabled)|boolean||
|[userFederationMappers[]](#specdefinitionuserfederationmappers)|object||
|[userFederationProviders[]](#specdefinitionuserfederationproviders)|object||
|[userManagedAccessAllowed](#specdefinitionusermanagedaccessallowed)|boolean||
|[users[]](#specdefinitionusers)|object||
|[verifyEmail](#specdefinitionverifyemail)|boolean||
|[waitIncrementSeconds](#specdefinitionwaitincrementseconds)|integer||
|[webAuthnPolicyAcceptableAaguids[]](#specdefinitionwebauthnpolicyacceptableaaguids)|string||
|[webAuthnPolicyAttestationConveyancePreference](#specdefinitionwebauthnpolicyattestationconveyancepreference)|string||
|[webAuthnPolicyAuthenticatorAttachment](#specdefinitionwebauthnpolicyauthenticatorattachment)|string||
|[webAuthnPolicyAvoidSameAuthenticatorRegister](#specdefinitionwebauthnpolicyavoidsameauthenticatorregister)|boolean||
|[webAuthnPolicyCreateTimeout](#specdefinitionwebauthnpolicycreatetimeout)|integer||
|[webAuthnPolicyExtraOrigins[]](#specdefinitionwebauthnpolicyextraorigins)|string||
|[webAuthnPolicyPasswordlessAcceptableAaguids[]](#specdefinitionwebauthnpolicypasswordlessacceptableaaguids)|string||
|[webAuthnPolicyPasswordlessAttestationConveyancePreference](#specdefinitionwebauthnpolicypasswordlessattestationconveyancepreference)|string||
|[webAuthnPolicyPasswordlessAuthenticatorAttachment](#specdefinitionwebauthnpolicypasswordlessauthenticatorattachment)|string||
|[webAuthnPolicyPasswordlessAvoidSameAuthenticatorRegister](#specdefinitionwebauthnpolicypasswordlessavoidsameauthenticatorregister)|boolean||
|[webAuthnPolicyPasswordlessCreateTimeout](#specdefinitionwebauthnpolicypasswordlesscreatetimeout)|integer||
|[webAuthnPolicyPasswordlessExtraOrigins[]](#specdefinitionwebauthnpolicypasswordlessextraorigins)|string||
|[webAuthnPolicyPasswordlessRequireResidentKey](#specdefinitionwebauthnpolicypasswordlessrequireresidentkey)|string||
|[webAuthnPolicyPasswordlessRpEntityName](#specdefinitionwebauthnpolicypasswordlessrpentityname)|string||
|[webAuthnPolicyPasswordlessRpId](#specdefinitionwebauthnpolicypasswordlessrpid)|string||
|[webAuthnPolicyPasswordlessSignatureAlgorithms[]](#specdefinitionwebauthnpolicypasswordlesssignaturealgorithms)|string||
|[webAuthnPolicyPasswordlessUserVerificationRequirement](#specdefinitionwebauthnpolicypasswordlessuserverificationrequirement)|string||
|[webAuthnPolicyRequireResidentKey](#specdefinitionwebauthnpolicyrequireresidentkey)|string||
|[webAuthnPolicyRpEntityName](#specdefinitionwebauthnpolicyrpentityname)|string||
|[webAuthnPolicyRpId](#specdefinitionwebauthnpolicyrpid)|string||
|[webAuthnPolicySignatureAlgorithms[]](#specdefinitionwebauthnpolicysignaturealgorithms)|string||
|[webAuthnPolicyUserVerificationRequirement](#specdefinitionwebauthnpolicyuserverificationrequirement)|string||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.realm) == has(oldSelf.realm)|Value is immutable|

*missing*

---

### spec.definition.accessCodeLifespan

Type: integer

*missing*

---

### spec.definition.accessCodeLifespanLogin

Type: integer

*missing*

---

### spec.definition.accessCodeLifespanUserAction

Type: integer

*missing*

---

### spec.definition.accessTokenLifespan

Type: integer

*missing*

---

### spec.definition.accessTokenLifespanForImplicitFlow

Type: integer

*missing*

---

### spec.definition.accountTheme

Type: string

*missing*

---

### spec.definition.actionTokenGeneratedByAdminLifespan

Type: integer

*missing*

---

### spec.definition.actionTokenGeneratedByUserLifespan

Type: integer

*missing*

---

### spec.definition.adminEventsDetailsEnabled

Type: boolean

*missing*

---

### spec.definition.adminEventsEnabled

Type: boolean

*missing*

---

### spec.definition.adminTheme

Type: string

*missing*

---

### spec.definition.applicationScopeMappings

Type: object

*missing*

---

### spec.definition.attributes

Type: object

*missing*

---

### spec.definition.authenticationFlows[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[alias](#specdefinitionauthenticationflowsalias)|string||
|[authenticationExecutions[]](#specdefinitionauthenticationflowsauthenticationexecutions)|object||
|[builtIn](#specdefinitionauthenticationflowsbuiltin)|boolean||
|[description](#specdefinitionauthenticationflowsdescription)|string||
|[id](#specdefinitionauthenticationflowsid)|string||
|[providerId](#specdefinitionauthenticationflowsproviderid)|string||
|[topLevel](#specdefinitionauthenticationflowstoplevel)|boolean||

*missing*

---

### spec.definition.authenticationFlows[].alias

Type: string

*missing*

---

### spec.definition.authenticationFlows[].authenticationExecutions[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[authenticator](#specdefinitionauthenticationflowsauthenticationexecutionsauthenticator)|string||
|[authenticatorConfig](#specdefinitionauthenticationflowsauthenticationexecutionsauthenticatorconfig)|string||
|[authenticatorFlow](#specdefinitionauthenticationflowsauthenticationexecutionsauthenticatorflow)|boolean||
|[autheticatorFlow](#specdefinitionauthenticationflowsauthenticationexecutionsautheticatorflow)|boolean||
|[flowAlias](#specdefinitionauthenticationflowsauthenticationexecutionsflowalias)|string||
|[priority](#specdefinitionauthenticationflowsauthenticationexecutionspriority)|integer||
|[requirement](#specdefinitionauthenticationflowsauthenticationexecutionsrequirement)|string||
|[userSetupAllowed](#specdefinitionauthenticationflowsauthenticationexecutionsusersetupallowed)|boolean||

*missing*

---

### spec.definition.authenticationFlows[].authenticationExecutions[].authenticator

Type: string

*missing*

---

### spec.definition.authenticationFlows[].authenticationExecutions[].authenticatorConfig

Type: string

*missing*

---

### spec.definition.authenticationFlows[].authenticationExecutions[].authenticatorFlow

Type: boolean

*missing*

---

### spec.definition.authenticationFlows[].authenticationExecutions[].autheticatorFlow

Type: boolean

*missing*

---

### spec.definition.authenticationFlows[].authenticationExecutions[].flowAlias

Type: string

*missing*

---

### spec.definition.authenticationFlows[].authenticationExecutions[].priority

Type: integer

*missing*

---

### spec.definition.authenticationFlows[].authenticationExecutions[].requirement

Type: string

*missing*

---

### spec.definition.authenticationFlows[].authenticationExecutions[].userSetupAllowed

Type: boolean

*missing*

---

### spec.definition.authenticationFlows[].builtIn

Type: boolean

*missing*

---

### spec.definition.authenticationFlows[].description

Type: string

*missing*

---

### spec.definition.authenticationFlows[].id

Type: string

*missing*

---

### spec.definition.authenticationFlows[].providerId

Type: string

*missing*

---

### spec.definition.authenticationFlows[].topLevel

Type: boolean

*missing*

---

### spec.definition.authenticatorConfig[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[alias](#specdefinitionauthenticatorconfigalias)|string||
|[config](#specdefinitionauthenticatorconfigconfig)|object||
|[id](#specdefinitionauthenticatorconfigid)|string||

*missing*

---

### spec.definition.authenticatorConfig[].alias

Type: string

*missing*

---

### spec.definition.authenticatorConfig[].config

Type: object

*missing*

---

### spec.definition.authenticatorConfig[].id

Type: string

*missing*

---

### spec.definition.browserFlow

Type: string

*missing*

---

### spec.definition.browserSecurityHeaders

Type: object

*missing*

---

### spec.definition.bruteForceProtected

Type: boolean

*missing*

---

### spec.definition.certificate

Type: string

*missing*

---

### spec.definition.clientAuthenticationFlow

Type: string

*missing*

---

### spec.definition.clientOfflineSessionIdleTimeout

Type: integer

*missing*

---

### spec.definition.clientOfflineSessionMaxLifespan

Type: integer

*missing*

---

### spec.definition.clientPolicies

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[globalPolicies[]](#specdefinitionclientpoliciesglobalpolicies)|object||
|[policies[]](#specdefinitionclientpoliciespolicies)|object||

*missing*

---

### spec.definition.clientPolicies.globalPolicies[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[conditions[]](#specdefinitionclientpoliciesglobalpoliciesconditions)|object||
|[description](#specdefinitionclientpoliciesglobalpoliciesdescription)|string||
|[enabled](#specdefinitionclientpoliciesglobalpoliciesenabled)|boolean||
|[name](#specdefinitionclientpoliciesglobalpoliciesname)|string||
|[profiles[]](#specdefinitionclientpoliciesglobalpoliciesprofiles)|string||

*missing*

---

### spec.definition.clientPolicies.globalPolicies[].conditions[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[condition](#specdefinitionclientpoliciesglobalpoliciesconditionscondition)|string||
|[configuration](#specdefinitionclientpoliciesglobalpoliciesconditionsconfiguration)|object||

*missing*

---

### spec.definition.clientPolicies.globalPolicies[].conditions[].condition

Type: string

*missing*

---

### spec.definition.clientPolicies.globalPolicies[].conditions[].configuration

Type: object

*missing*

---

### spec.definition.clientPolicies.globalPolicies[].description

Type: string

*missing*

---

### spec.definition.clientPolicies.globalPolicies[].enabled

Type: boolean

*missing*

---

### spec.definition.clientPolicies.globalPolicies[].name

Type: string

*missing*

---

### spec.definition.clientPolicies.globalPolicies[].profiles[]

Type: string

*missing*

---

### spec.definition.clientPolicies.policies[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[conditions[]](#specdefinitionclientpoliciespoliciesconditions)|object||
|[description](#specdefinitionclientpoliciespoliciesdescription)|string||
|[enabled](#specdefinitionclientpoliciespoliciesenabled)|boolean||
|[name](#specdefinitionclientpoliciespoliciesname)|string||
|[profiles[]](#specdefinitionclientpoliciespoliciesprofiles)|string||

*missing*

---

### spec.definition.clientPolicies.policies[].conditions[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[condition](#specdefinitionclientpoliciespoliciesconditionscondition)|string||
|[configuration](#specdefinitionclientpoliciespoliciesconditionsconfiguration)|object||

*missing*

---

### spec.definition.clientPolicies.policies[].conditions[].condition

Type: string

*missing*

---

### spec.definition.clientPolicies.policies[].conditions[].configuration

Type: object

*missing*

---

### spec.definition.clientPolicies.policies[].description

Type: string

*missing*

---

### spec.definition.clientPolicies.policies[].enabled

Type: boolean

*missing*

---

### spec.definition.clientPolicies.policies[].name

Type: string

*missing*

---

### spec.definition.clientPolicies.policies[].profiles[]

Type: string

*missing*

---

### spec.definition.clientProfiles

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[globalProfiles[]](#specdefinitionclientprofilesglobalprofiles)|object||
|[profiles[]](#specdefinitionclientprofilesprofiles)|object||

*missing*

---

### spec.definition.clientProfiles.globalProfiles[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[description](#specdefinitionclientprofilesglobalprofilesdescription)|string||
|[executors[]](#specdefinitionclientprofilesglobalprofilesexecutors)|object||
|[name](#specdefinitionclientprofilesglobalprofilesname)|string||

*missing*

---

### spec.definition.clientProfiles.globalProfiles[].description

Type: string

*missing*

---

### spec.definition.clientProfiles.globalProfiles[].executors[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[configuration](#specdefinitionclientprofilesglobalprofilesexecutorsconfiguration)|object||
|[executor](#specdefinitionclientprofilesglobalprofilesexecutorsexecutor)|string||

*missing*

---

### spec.definition.clientProfiles.globalProfiles[].executors[].configuration

Type: object

*missing*

---

### spec.definition.clientProfiles.globalProfiles[].executors[].executor

Type: string

*missing*

---

### spec.definition.clientProfiles.globalProfiles[].name

Type: string

*missing*

---

### spec.definition.clientProfiles.profiles[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[description](#specdefinitionclientprofilesprofilesdescription)|string||
|[executors[]](#specdefinitionclientprofilesprofilesexecutors)|object||
|[name](#specdefinitionclientprofilesprofilesname)|string||

*missing*

---

### spec.definition.clientProfiles.profiles[].description

Type: string

*missing*

---

### spec.definition.clientProfiles.profiles[].executors[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[configuration](#specdefinitionclientprofilesprofilesexecutorsconfiguration)|object||
|[executor](#specdefinitionclientprofilesprofilesexecutorsexecutor)|string||

*missing*

---

### spec.definition.clientProfiles.profiles[].executors[].configuration

Type: object

*missing*

---

### spec.definition.clientProfiles.profiles[].executors[].executor

Type: string

*missing*

---

### spec.definition.clientProfiles.profiles[].name

Type: string

*missing*

---

### spec.definition.clientScopeMappings

Type: object

*missing*

---

### spec.definition.clientScopes[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[attributes](#specdefinitionclientscopesattributes)|object||
|[description](#specdefinitionclientscopesdescription)|string||
|[id](#specdefinitionclientscopesid)|string||
|[name](#specdefinitionclientscopesname)|string||
|[protocol](#specdefinitionclientscopesprotocol)|string||
|[protocolMappers[]](#specdefinitionclientscopesprotocolmappers)|object||

*missing*

---

### spec.definition.clientScopes[].attributes

Type: object

*missing*

---

### spec.definition.clientScopes[].description

Type: string

*missing*

---

### spec.definition.clientScopes[].id

Type: string

*missing*

---

### spec.definition.clientScopes[].name

Type: string

*missing*

---

### spec.definition.clientScopes[].protocol

Type: string

*missing*

---

### spec.definition.clientScopes[].protocolMappers[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionclientscopesprotocolmappersconfig)|object||
|[consentRequired](#specdefinitionclientscopesprotocolmappersconsentrequired)|boolean||
|[consentText](#specdefinitionclientscopesprotocolmappersconsenttext)|string||
|[id](#specdefinitionclientscopesprotocolmappersid)|string||
|[name](#specdefinitionclientscopesprotocolmappersname)|string||
|[protocol](#specdefinitionclientscopesprotocolmappersprotocol)|string||
|[protocolMapper](#specdefinitionclientscopesprotocolmappersprotocolmapper)|string||

*missing*

---

### spec.definition.clientScopes[].protocolMappers[].config

Type: object

*missing*

---

### spec.definition.clientScopes[].protocolMappers[].consentRequired

Type: boolean

*missing*

---

### spec.definition.clientScopes[].protocolMappers[].consentText

Type: string

*missing*

---

### spec.definition.clientScopes[].protocolMappers[].id

Type: string

*missing*

---

### spec.definition.clientScopes[].protocolMappers[].name

Type: string

*missing*

---

### spec.definition.clientScopes[].protocolMappers[].protocol

Type: string

*missing*

---

### spec.definition.clientScopes[].protocolMappers[].protocolMapper

Type: string

*missing*

---

### spec.definition.clientSessionIdleTimeout

Type: integer

*missing*

---

### spec.definition.clientSessionMaxLifespan

Type: integer

*missing*

---

### spec.definition.clientTemplates[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[attributes](#specdefinitionclienttemplatesattributes)|object||
|[bearerOnly](#specdefinitionclienttemplatesbeareronly)|boolean||
|[consentRequired](#specdefinitionclienttemplatesconsentrequired)|boolean||
|[description](#specdefinitionclienttemplatesdescription)|string||
|[directAccessGrantsEnabled](#specdefinitionclienttemplatesdirectaccessgrantsenabled)|boolean||
|[frontchannelLogout](#specdefinitionclienttemplatesfrontchannellogout)|boolean||
|[fullScopeAllowed](#specdefinitionclienttemplatesfullscopeallowed)|boolean||
|[id](#specdefinitionclienttemplatesid)|string||
|[implicitFlowEnabled](#specdefinitionclienttemplatesimplicitflowenabled)|boolean||
|[name](#specdefinitionclienttemplatesname)|string||
|[protocol](#specdefinitionclienttemplatesprotocol)|string||
|[protocolMappers[]](#specdefinitionclienttemplatesprotocolmappers)|object||
|[publicClient](#specdefinitionclienttemplatespublicclient)|boolean||
|[serviceAccountsEnabled](#specdefinitionclienttemplatesserviceaccountsenabled)|boolean||
|[standardFlowEnabled](#specdefinitionclienttemplatesstandardflowenabled)|boolean||

*missing*

---

### spec.definition.clientTemplates[].attributes

Type: object

*missing*

---

### spec.definition.clientTemplates[].bearerOnly

Type: boolean

*missing*

---

### spec.definition.clientTemplates[].consentRequired

Type: boolean

*missing*

---

### spec.definition.clientTemplates[].description

Type: string

*missing*

---

### spec.definition.clientTemplates[].directAccessGrantsEnabled

Type: boolean

*missing*

---

### spec.definition.clientTemplates[].frontchannelLogout

Type: boolean

*missing*

---

### spec.definition.clientTemplates[].fullScopeAllowed

Type: boolean

*missing*

---

### spec.definition.clientTemplates[].id

Type: string

*missing*

---

### spec.definition.clientTemplates[].implicitFlowEnabled

Type: boolean

*missing*

---

### spec.definition.clientTemplates[].name

Type: string

*missing*

---

### spec.definition.clientTemplates[].protocol

Type: string

*missing*

---

### spec.definition.clientTemplates[].protocolMappers[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionclienttemplatesprotocolmappersconfig)|object||
|[consentRequired](#specdefinitionclienttemplatesprotocolmappersconsentrequired)|boolean||
|[consentText](#specdefinitionclienttemplatesprotocolmappersconsenttext)|string||
|[id](#specdefinitionclienttemplatesprotocolmappersid)|string||
|[name](#specdefinitionclienttemplatesprotocolmappersname)|string||
|[protocol](#specdefinitionclienttemplatesprotocolmappersprotocol)|string||
|[protocolMapper](#specdefinitionclienttemplatesprotocolmappersprotocolmapper)|string||

*missing*

---

### spec.definition.clientTemplates[].protocolMappers[].config

Type: object

*missing*

---

### spec.definition.clientTemplates[].protocolMappers[].consentRequired

Type: boolean

*missing*

---

### spec.definition.clientTemplates[].protocolMappers[].consentText

Type: string

*missing*

---

### spec.definition.clientTemplates[].protocolMappers[].id

Type: string

*missing*

---

### spec.definition.clientTemplates[].protocolMappers[].name

Type: string

*missing*

---

### spec.definition.clientTemplates[].protocolMappers[].protocol

Type: string

*missing*

---

### spec.definition.clientTemplates[].protocolMappers[].protocolMapper

Type: string

*missing*

---

### spec.definition.clientTemplates[].publicClient

Type: boolean

*missing*

---

### spec.definition.clientTemplates[].serviceAccountsEnabled

Type: boolean

*missing*

---

### spec.definition.clientTemplates[].standardFlowEnabled

Type: boolean

*missing*

---

### spec.definition.codeSecret

Type: string

*missing*

---

### spec.definition.defaultDefaultClientScopes[]

Type: string

*missing*

---

### spec.definition.defaultGroups[]

Type: string

*missing*

---

### spec.definition.defaultLocale

Type: string

*missing*

---

### spec.definition.defaultOptionalClientScopes[]

Type: string

*missing*

---

### spec.definition.defaultRole

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[attributes](#specdefinitiondefaultroleattributes)|object||
|[clientRole](#specdefinitiondefaultroleclientrole)|boolean||
|[composite](#specdefinitiondefaultrolecomposite)|boolean||
|[composites](#specdefinitiondefaultrolecomposites)|object||
|[containerId](#specdefinitiondefaultrolecontainerid)|string||
|[description](#specdefinitiondefaultroledescription)|string||
|[id](#specdefinitiondefaultroleid)|string||
|[name](#specdefinitiondefaultrolename)|string||
|[scopeParamRequired](#specdefinitiondefaultrolescopeparamrequired)|boolean||

*missing*

---

### spec.definition.defaultRole.attributes

Type: object

*missing*

---

### spec.definition.defaultRole.clientRole

Type: boolean

*missing*

---

### spec.definition.defaultRole.composite

Type: boolean

*missing*

---

### spec.definition.defaultRole.composites

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[application](#specdefinitiondefaultrolecompositesapplication)|object||
|[client](#specdefinitiondefaultrolecompositesclient)|object||
|[realm[]](#specdefinitiondefaultrolecompositesrealm)|string||

*missing*

---

### spec.definition.defaultRole.composites.application

Type: object

*missing*

---

### spec.definition.defaultRole.composites.client

Type: object

*missing*

---

### spec.definition.defaultRole.composites.realm[]

Type: string

*missing*

---

### spec.definition.defaultRole.containerId

Type: string

*missing*

---

### spec.definition.defaultRole.description

Type: string

*missing*

---

### spec.definition.defaultRole.id

Type: string

*missing*

---

### spec.definition.defaultRole.name

Type: string

*missing*

---

### spec.definition.defaultRole.scopeParamRequired

Type: boolean

*missing*

---

### spec.definition.defaultRoles[]

Type: string

*missing*

---

### spec.definition.defaultSignatureAlgorithm

Type: string

*missing*

---

### spec.definition.directGrantFlow

Type: string

*missing*

---

### spec.definition.displayName

Type: string

*missing*

---

### spec.definition.displayNameHtml

Type: string

*missing*

---

### spec.definition.dockerAuthenticationFlow

Type: string

*missing*

---

### spec.definition.duplicateEmailsAllowed

Type: boolean

*missing*

---

### spec.definition.editUsernameAllowed

Type: boolean

*missing*

---

### spec.definition.emailTheme

Type: string

*missing*

---

### spec.definition.enabled

Type: boolean

*missing*

---

### spec.definition.enabledEventTypes[]

Type: string

*missing*

---

### spec.definition.eventsEnabled

Type: boolean

*missing*

---

### spec.definition.eventsExpiration

Type: integer

*missing*

---

### spec.definition.eventsListeners[]

Type: string

*missing*

---

### spec.definition.failureFactor

Type: integer

*missing*

---

### spec.definition.federatedUsers[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[access](#specdefinitionfederatedusersaccess)|object||
|[applicationRoles](#specdefinitionfederatedusersapplicationroles)|object||
|[attributes](#specdefinitionfederatedusersattributes)|object||
|[clientConsents[]](#specdefinitionfederatedusersclientconsents)|object||
|[clientRoles](#specdefinitionfederatedusersclientroles)|object||
|[createdTimestamp](#specdefinitionfederateduserscreatedtimestamp)|integer||
|[credentials[]](#specdefinitionfederateduserscredentials)|object||
|[disableableCredentialTypes[]](#specdefinitionfederatedusersdisableablecredentialtypes)|string||
|[email](#specdefinitionfederatedusersemail)|string||
|[emailVerified](#specdefinitionfederatedusersemailverified)|boolean||
|[enabled](#specdefinitionfederatedusersenabled)|boolean||
|[federatedIdentities[]](#specdefinitionfederatedusersfederatedidentities)|object||
|[federationLink](#specdefinitionfederatedusersfederationlink)|string||
|[firstName](#specdefinitionfederatedusersfirstname)|string||
|[groups[]](#specdefinitionfederatedusersgroups)|string||
|[id](#specdefinitionfederatedusersid)|string||
|[lastName](#specdefinitionfederateduserslastname)|string||
|[notBefore](#specdefinitionfederatedusersnotbefore)|integer||
|[origin](#specdefinitionfederatedusersorigin)|string||
|[realmRoles[]](#specdefinitionfederatedusersrealmroles)|string||
|[requiredActions[]](#specdefinitionfederatedusersrequiredactions)|string||
|[self](#specdefinitionfederatedusersself)|string||
|[serviceAccountClientId](#specdefinitionfederatedusersserviceaccountclientid)|string||
|[socialLinks[]](#specdefinitionfederateduserssociallinks)|object||
|[totp](#specdefinitionfederateduserstotp)|boolean||
|[userProfileMetadata](#specdefinitionfederatedusersuserprofilemetadata)|object||
|[username](#specdefinitionfederatedusersusername)|string||

*missing*

---

### spec.definition.federatedUsers[].access

Type: object

*missing*

---

### spec.definition.federatedUsers[].applicationRoles

Type: object

*missing*

---

### spec.definition.federatedUsers[].attributes

Type: object

*missing*

---

### spec.definition.federatedUsers[].clientConsents[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clientId](#specdefinitionfederatedusersclientconsentsclientid)|string||
|[createdDate](#specdefinitionfederatedusersclientconsentscreateddate)|integer||
|[grantedClientScopes[]](#specdefinitionfederatedusersclientconsentsgrantedclientscopes)|string||
|[grantedRealmRoles[]](#specdefinitionfederatedusersclientconsentsgrantedrealmroles)|string||
|[lastUpdatedDate](#specdefinitionfederatedusersclientconsentslastupdateddate)|integer||

*missing*

---

### spec.definition.federatedUsers[].clientConsents[].clientId

Type: string

*missing*

---

### spec.definition.federatedUsers[].clientConsents[].createdDate

Type: integer

*missing*

---

### spec.definition.federatedUsers[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

---

### spec.definition.federatedUsers[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

---

### spec.definition.federatedUsers[].clientConsents[].lastUpdatedDate

Type: integer

*missing*

---

### spec.definition.federatedUsers[].clientRoles

Type: object

*missing*

---

### spec.definition.federatedUsers[].createdTimestamp

Type: integer

*missing*

---

### spec.definition.federatedUsers[].credentials[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[algorithm](#specdefinitionfederateduserscredentialsalgorithm)|string||
|[config](#specdefinitionfederateduserscredentialsconfig)|object||
|[counter](#specdefinitionfederateduserscredentialscounter)|integer||
|[createdDate](#specdefinitionfederateduserscredentialscreateddate)|integer||
|[credentialData](#specdefinitionfederateduserscredentialscredentialdata)|string||
|[device](#specdefinitionfederateduserscredentialsdevice)|string||
|[digits](#specdefinitionfederateduserscredentialsdigits)|integer||
|[hashIterations](#specdefinitionfederateduserscredentialshashiterations)|integer||
|[hashedSaltedValue](#specdefinitionfederateduserscredentialshashedsaltedvalue)|string||
|[id](#specdefinitionfederateduserscredentialsid)|string||
|[period](#specdefinitionfederateduserscredentialsperiod)|integer||
|[priority](#specdefinitionfederateduserscredentialspriority)|integer||
|[salt](#specdefinitionfederateduserscredentialssalt)|string||
|[secretData](#specdefinitionfederateduserscredentialssecretdata)|string||
|[temporary](#specdefinitionfederateduserscredentialstemporary)|boolean||
|[type](#specdefinitionfederateduserscredentialstype)|string||
|[userLabel](#specdefinitionfederateduserscredentialsuserlabel)|string||
|[value](#specdefinitionfederateduserscredentialsvalue)|string||

*missing*

---

### spec.definition.federatedUsers[].credentials[].algorithm

Type: string

*missing*

---

### spec.definition.federatedUsers[].credentials[].config

Type: object

*missing*

---

### spec.definition.federatedUsers[].credentials[].counter

Type: integer

*missing*

---

### spec.definition.federatedUsers[].credentials[].createdDate

Type: integer

*missing*

---

### spec.definition.federatedUsers[].credentials[].credentialData

Type: string

*missing*

---

### spec.definition.federatedUsers[].credentials[].device

Type: string

*missing*

---

### spec.definition.federatedUsers[].credentials[].digits

Type: integer

*missing*

---

### spec.definition.federatedUsers[].credentials[].hashIterations

Type: integer

*missing*

---

### spec.definition.federatedUsers[].credentials[].hashedSaltedValue

Type: string

*missing*

---

### spec.definition.federatedUsers[].credentials[].id

Type: string

*missing*

---

### spec.definition.federatedUsers[].credentials[].period

Type: integer

*missing*

---

### spec.definition.federatedUsers[].credentials[].priority

Type: integer

*missing*

---

### spec.definition.federatedUsers[].credentials[].salt

Type: string

*missing*

---

### spec.definition.federatedUsers[].credentials[].secretData

Type: string

*missing*

---

### spec.definition.federatedUsers[].credentials[].temporary

Type: boolean

*missing*

---

### spec.definition.federatedUsers[].credentials[].type

Type: string

*missing*

---

### spec.definition.federatedUsers[].credentials[].userLabel

Type: string

*missing*

---

### spec.definition.federatedUsers[].credentials[].value

Type: string

*missing*

---

### spec.definition.federatedUsers[].disableableCredentialTypes[]

Type: string

*missing*

---

### spec.definition.federatedUsers[].email

Type: string

*missing*

---

### spec.definition.federatedUsers[].emailVerified

Type: boolean

*missing*

---

### spec.definition.federatedUsers[].enabled

Type: boolean

*missing*

---

### spec.definition.federatedUsers[].federatedIdentities[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[identityProvider](#specdefinitionfederatedusersfederatedidentitiesidentityprovider)|string||
|[userId](#specdefinitionfederatedusersfederatedidentitiesuserid)|string||
|[userName](#specdefinitionfederatedusersfederatedidentitiesusername)|string||

*missing*

---

### spec.definition.federatedUsers[].federatedIdentities[].identityProvider

Type: string

*missing*

---

### spec.definition.federatedUsers[].federatedIdentities[].userId

Type: string

*missing*

---

### spec.definition.federatedUsers[].federatedIdentities[].userName

Type: string

*missing*

---

### spec.definition.federatedUsers[].federationLink

Type: string

*missing*

---

### spec.definition.federatedUsers[].firstName

Type: string

*missing*

---

### spec.definition.federatedUsers[].groups[]

Type: string

*missing*

---

### spec.definition.federatedUsers[].id

Type: string

*missing*

---

### spec.definition.federatedUsers[].lastName

Type: string

*missing*

---

### spec.definition.federatedUsers[].notBefore

Type: integer

*missing*

---

### spec.definition.federatedUsers[].origin

Type: string

*missing*

---

### spec.definition.federatedUsers[].realmRoles[]

Type: string

*missing*

---

### spec.definition.federatedUsers[].requiredActions[]

Type: string

*missing*

---

### spec.definition.federatedUsers[].self

Type: string

*missing*

---

### spec.definition.federatedUsers[].serviceAccountClientId

Type: string

*missing*

---

### spec.definition.federatedUsers[].socialLinks[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[socialProvider](#specdefinitionfederateduserssociallinkssocialprovider)|string||
|[socialUserId](#specdefinitionfederateduserssociallinkssocialuserid)|string||
|[socialUsername](#specdefinitionfederateduserssociallinkssocialusername)|string||

*missing*

---

### spec.definition.federatedUsers[].socialLinks[].socialProvider

Type: string

*missing*

---

### spec.definition.federatedUsers[].socialLinks[].socialUserId

Type: string

*missing*

---

### spec.definition.federatedUsers[].socialLinks[].socialUsername

Type: string

*missing*

---

### spec.definition.federatedUsers[].totp

Type: boolean

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[attributes[]](#specdefinitionfederatedusersuserprofilemetadataattributes)|object||
|[groups[]](#specdefinitionfederatedusersuserprofilemetadatagroups)|object||

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata.attributes[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[annotations](#specdefinitionfederatedusersuserprofilemetadataattributesannotations)|object||
|[displayName](#specdefinitionfederatedusersuserprofilemetadataattributesdisplayname)|string||
|[group](#specdefinitionfederatedusersuserprofilemetadataattributesgroup)|string||
|[multivalued](#specdefinitionfederatedusersuserprofilemetadataattributesmultivalued)|boolean||
|[name](#specdefinitionfederatedusersuserprofilemetadataattributesname)|string||
|[readOnly](#specdefinitionfederatedusersuserprofilemetadataattributesreadonly)|boolean||
|[required](#specdefinitionfederatedusersuserprofilemetadataattributesrequired)|boolean||
|[validators](#specdefinitionfederatedusersuserprofilemetadataattributesvalidators)|object||

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata.attributes[].annotations

Type: object

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata.attributes[].displayName

Type: string

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata.attributes[].group

Type: string

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata.attributes[].name

Type: string

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata.attributes[].required

Type: boolean

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata.attributes[].validators

Type: object

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata.groups[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[annotations](#specdefinitionfederatedusersuserprofilemetadatagroupsannotations)|object||
|[displayDescription](#specdefinitionfederatedusersuserprofilemetadatagroupsdisplaydescription)|string||
|[displayHeader](#specdefinitionfederatedusersuserprofilemetadatagroupsdisplayheader)|string||
|[name](#specdefinitionfederatedusersuserprofilemetadatagroupsname)|string||

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata.groups[].annotations

Type: object

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata.groups[].displayDescription

Type: string

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata.groups[].displayHeader

Type: string

*missing*

---

### spec.definition.federatedUsers[].userProfileMetadata.groups[].name

Type: string

*missing*

---

### spec.definition.federatedUsers[].username

Type: string

*missing*

---

### spec.definition.firstBrokerLoginFlow

Type: string

*missing*

---

### spec.definition.id

Type: string

*missing*

---

### spec.definition.identityProviderMappers[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionidentityprovidermappersconfig)|object||
|[id](#specdefinitionidentityprovidermappersid)|string||
|[identityProviderAlias](#specdefinitionidentityprovidermappersidentityprovideralias)|string||
|[identityProviderMapper](#specdefinitionidentityprovidermappersidentityprovidermapper)|string||
|[name](#specdefinitionidentityprovidermappersname)|string||

*missing*

---

### spec.definition.identityProviderMappers[].config

Type: object

*missing*

---

### spec.definition.identityProviderMappers[].id

Type: string

*missing*

---

### spec.definition.identityProviderMappers[].identityProviderAlias

Type: string

*missing*

---

### spec.definition.identityProviderMappers[].identityProviderMapper

Type: string

*missing*

---

### spec.definition.identityProviderMappers[].name

Type: string

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

### spec.definition.internationalizationEnabled

Type: boolean

*missing*

---

### spec.definition.keycloakVersion

Type: string

*missing*

---

### spec.definition.localizationTexts

Type: object

*missing*

---

### spec.definition.loginTheme

Type: string

*missing*

---

### spec.definition.loginWithEmailAllowed

Type: boolean

*missing*

---

### spec.definition.maxDeltaTimeSeconds

Type: integer

*missing*

---

### spec.definition.maxFailureWaitSeconds

Type: integer

*missing*

---

### spec.definition.maxTemporaryLockouts

Type: integer

*missing*

---

### spec.definition.minimumQuickLoginWaitSeconds

Type: integer

*missing*

---

### spec.definition.notBefore

Type: integer

*missing*

---

### spec.definition.oAuth2DeviceCodeLifespan

Type: integer

*missing*

---

### spec.definition.oAuth2DevicePollingInterval

Type: integer

*missing*

---

### spec.definition.oauth2DeviceCodeLifespan

Type: integer

*missing*

---

### spec.definition.oauth2DevicePollingInterval

Type: integer

*missing*

---

### spec.definition.offlineSessionIdleTimeout

Type: integer

*missing*

---

### spec.definition.offlineSessionMaxLifespan

Type: integer

*missing*

---

### spec.definition.offlineSessionMaxLifespanEnabled

Type: boolean

*missing*

---

### spec.definition.organizations[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[alias](#specdefinitionorganizationsalias)|string||
|[attributes](#specdefinitionorganizationsattributes)|object||
|[description](#specdefinitionorganizationsdescription)|string||
|[domains[]](#specdefinitionorganizationsdomains)|object||
|[enabled](#specdefinitionorganizationsenabled)|boolean||
|[id](#specdefinitionorganizationsid)|string||
|[identityProviders[]](#specdefinitionorganizationsidentityproviders)|object||
|[members[]](#specdefinitionorganizationsmembers)|object||
|[name](#specdefinitionorganizationsname)|string||
|[redirectUrl](#specdefinitionorganizationsredirecturl)|string||

*missing*

---

### spec.definition.organizations[].alias

Type: string

*missing*

---

### spec.definition.organizations[].attributes

Type: object

*missing*

---

### spec.definition.organizations[].description

Type: string

*missing*

---

### spec.definition.organizations[].domains[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[name](#specdefinitionorganizationsdomainsname)|string||
|[verified](#specdefinitionorganizationsdomainsverified)|boolean||

*missing*

---

### spec.definition.organizations[].domains[].name

Type: string

*missing*

---

### spec.definition.organizations[].domains[].verified

Type: boolean

*missing*

---

### spec.definition.organizations[].enabled

Type: boolean

*missing*

---

### spec.definition.organizations[].id

Type: string

*missing*

---

### spec.definition.organizations[].identityProviders[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[addReadTokenRoleOnCreate](#specdefinitionorganizationsidentityprovidersaddreadtokenroleoncreate)|boolean||
|[alias](#specdefinitionorganizationsidentityprovidersalias)|string||
|[authenticateByDefault](#specdefinitionorganizationsidentityprovidersauthenticatebydefault)|boolean||
|[config](#specdefinitionorganizationsidentityprovidersconfig)|object||
|[displayName](#specdefinitionorganizationsidentityprovidersdisplayname)|string||
|[enabled](#specdefinitionorganizationsidentityprovidersenabled)|boolean||
|[firstBrokerLoginFlowAlias](#specdefinitionorganizationsidentityprovidersfirstbrokerloginflowalias)|string||
|[hideOnLogin](#specdefinitionorganizationsidentityprovidershideonlogin)|boolean||
|[internalId](#specdefinitionorganizationsidentityprovidersinternalid)|string||
|[linkOnly](#specdefinitionorganizationsidentityproviderslinkonly)|boolean||
|[organizationId](#specdefinitionorganizationsidentityprovidersorganizationid)|string||
|[postBrokerLoginFlowAlias](#specdefinitionorganizationsidentityproviderspostbrokerloginflowalias)|string||
|[providerId](#specdefinitionorganizationsidentityprovidersproviderid)|string||
|[storeToken](#specdefinitionorganizationsidentityprovidersstoretoken)|boolean||
|[trustEmail](#specdefinitionorganizationsidentityproviderstrustemail)|boolean||
|[updateProfileFirstLogin](#specdefinitionorganizationsidentityprovidersupdateprofilefirstlogin)|boolean||
|[updateProfileFirstLoginMode](#specdefinitionorganizationsidentityprovidersupdateprofilefirstloginmode)|string||

*missing*

---

### spec.definition.organizations[].identityProviders[].addReadTokenRoleOnCreate

Type: boolean

*missing*

---

### spec.definition.organizations[].identityProviders[].alias

Type: string

*missing*

---

### spec.definition.organizations[].identityProviders[].authenticateByDefault

Type: boolean

*missing*

---

### spec.definition.organizations[].identityProviders[].config

Type: object

*missing*

---

### spec.definition.organizations[].identityProviders[].displayName

Type: string

*missing*

---

### spec.definition.organizations[].identityProviders[].enabled

Type: boolean

*missing*

---

### spec.definition.organizations[].identityProviders[].firstBrokerLoginFlowAlias

Type: string

*missing*

---

### spec.definition.organizations[].identityProviders[].hideOnLogin

Type: boolean

*missing*

---

### spec.definition.organizations[].identityProviders[].internalId

Type: string

*missing*

---

### spec.definition.organizations[].identityProviders[].linkOnly

Type: boolean

*missing*

---

### spec.definition.organizations[].identityProviders[].organizationId

Type: string

*missing*

---

### spec.definition.organizations[].identityProviders[].postBrokerLoginFlowAlias

Type: string

*missing*

---

### spec.definition.organizations[].identityProviders[].providerId

Type: string

*missing*

---

### spec.definition.organizations[].identityProviders[].storeToken

Type: boolean

*missing*

---

### spec.definition.organizations[].identityProviders[].trustEmail

Type: boolean

*missing*

---

### spec.definition.organizations[].identityProviders[].updateProfileFirstLogin

Type: boolean

*missing*

---

### spec.definition.organizations[].identityProviders[].updateProfileFirstLoginMode

Type: string

*missing*

---

### spec.definition.organizations[].members[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[access](#specdefinitionorganizationsmembersaccess)|object||
|[applicationRoles](#specdefinitionorganizationsmembersapplicationroles)|object||
|[attributes](#specdefinitionorganizationsmembersattributes)|object||
|[clientConsents[]](#specdefinitionorganizationsmembersclientconsents)|object||
|[clientRoles](#specdefinitionorganizationsmembersclientroles)|object||
|[createdTimestamp](#specdefinitionorganizationsmemberscreatedtimestamp)|integer||
|[credentials[]](#specdefinitionorganizationsmemberscredentials)|object||
|[disableableCredentialTypes[]](#specdefinitionorganizationsmembersdisableablecredentialtypes)|string||
|[email](#specdefinitionorganizationsmembersemail)|string||
|[emailVerified](#specdefinitionorganizationsmembersemailverified)|boolean||
|[enabled](#specdefinitionorganizationsmembersenabled)|boolean||
|[federatedIdentities[]](#specdefinitionorganizationsmembersfederatedidentities)|object||
|[federationLink](#specdefinitionorganizationsmembersfederationlink)|string||
|[firstName](#specdefinitionorganizationsmembersfirstname)|string||
|[groups[]](#specdefinitionorganizationsmembersgroups)|string||
|[id](#specdefinitionorganizationsmembersid)|string||
|[lastName](#specdefinitionorganizationsmemberslastname)|string||
|[membershipType](#specdefinitionorganizationsmembersmembershiptype)|string||
|[notBefore](#specdefinitionorganizationsmembersnotbefore)|integer||
|[origin](#specdefinitionorganizationsmembersorigin)|string||
|[realmRoles[]](#specdefinitionorganizationsmembersrealmroles)|string||
|[requiredActions[]](#specdefinitionorganizationsmembersrequiredactions)|string||
|[self](#specdefinitionorganizationsmembersself)|string||
|[serviceAccountClientId](#specdefinitionorganizationsmembersserviceaccountclientid)|string||
|[socialLinks[]](#specdefinitionorganizationsmemberssociallinks)|object||
|[totp](#specdefinitionorganizationsmemberstotp)|boolean||
|[userProfileMetadata](#specdefinitionorganizationsmembersuserprofilemetadata)|object||
|[username](#specdefinitionorganizationsmembersusername)|string||

*missing*

---

### spec.definition.organizations[].members[].access

Type: object

*missing*

---

### spec.definition.organizations[].members[].applicationRoles

Type: object

*missing*

---

### spec.definition.organizations[].members[].attributes

Type: object

*missing*

---

### spec.definition.organizations[].members[].clientConsents[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clientId](#specdefinitionorganizationsmembersclientconsentsclientid)|string||
|[createdDate](#specdefinitionorganizationsmembersclientconsentscreateddate)|integer||
|[grantedClientScopes[]](#specdefinitionorganizationsmembersclientconsentsgrantedclientscopes)|string||
|[grantedRealmRoles[]](#specdefinitionorganizationsmembersclientconsentsgrantedrealmroles)|string||
|[lastUpdatedDate](#specdefinitionorganizationsmembersclientconsentslastupdateddate)|integer||

*missing*

---

### spec.definition.organizations[].members[].clientConsents[].clientId

Type: string

*missing*

---

### spec.definition.organizations[].members[].clientConsents[].createdDate

Type: integer

*missing*

---

### spec.definition.organizations[].members[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

---

### spec.definition.organizations[].members[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

---

### spec.definition.organizations[].members[].clientConsents[].lastUpdatedDate

Type: integer

*missing*

---

### spec.definition.organizations[].members[].clientRoles

Type: object

*missing*

---

### spec.definition.organizations[].members[].createdTimestamp

Type: integer

*missing*

---

### spec.definition.organizations[].members[].credentials[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[algorithm](#specdefinitionorganizationsmemberscredentialsalgorithm)|string||
|[config](#specdefinitionorganizationsmemberscredentialsconfig)|object||
|[counter](#specdefinitionorganizationsmemberscredentialscounter)|integer||
|[createdDate](#specdefinitionorganizationsmemberscredentialscreateddate)|integer||
|[credentialData](#specdefinitionorganizationsmemberscredentialscredentialdata)|string||
|[device](#specdefinitionorganizationsmemberscredentialsdevice)|string||
|[digits](#specdefinitionorganizationsmemberscredentialsdigits)|integer||
|[hashIterations](#specdefinitionorganizationsmemberscredentialshashiterations)|integer||
|[hashedSaltedValue](#specdefinitionorganizationsmemberscredentialshashedsaltedvalue)|string||
|[id](#specdefinitionorganizationsmemberscredentialsid)|string||
|[period](#specdefinitionorganizationsmemberscredentialsperiod)|integer||
|[priority](#specdefinitionorganizationsmemberscredentialspriority)|integer||
|[salt](#specdefinitionorganizationsmemberscredentialssalt)|string||
|[secretData](#specdefinitionorganizationsmemberscredentialssecretdata)|string||
|[temporary](#specdefinitionorganizationsmemberscredentialstemporary)|boolean||
|[type](#specdefinitionorganizationsmemberscredentialstype)|string||
|[userLabel](#specdefinitionorganizationsmemberscredentialsuserlabel)|string||
|[value](#specdefinitionorganizationsmemberscredentialsvalue)|string||

*missing*

---

### spec.definition.organizations[].members[].credentials[].algorithm

Type: string

*missing*

---

### spec.definition.organizations[].members[].credentials[].config

Type: object

*missing*

---

### spec.definition.organizations[].members[].credentials[].counter

Type: integer

*missing*

---

### spec.definition.organizations[].members[].credentials[].createdDate

Type: integer

*missing*

---

### spec.definition.organizations[].members[].credentials[].credentialData

Type: string

*missing*

---

### spec.definition.organizations[].members[].credentials[].device

Type: string

*missing*

---

### spec.definition.organizations[].members[].credentials[].digits

Type: integer

*missing*

---

### spec.definition.organizations[].members[].credentials[].hashIterations

Type: integer

*missing*

---

### spec.definition.organizations[].members[].credentials[].hashedSaltedValue

Type: string

*missing*

---

### spec.definition.organizations[].members[].credentials[].id

Type: string

*missing*

---

### spec.definition.organizations[].members[].credentials[].period

Type: integer

*missing*

---

### spec.definition.organizations[].members[].credentials[].priority

Type: integer

*missing*

---

### spec.definition.organizations[].members[].credentials[].salt

Type: string

*missing*

---

### spec.definition.organizations[].members[].credentials[].secretData

Type: string

*missing*

---

### spec.definition.organizations[].members[].credentials[].temporary

Type: boolean

*missing*

---

### spec.definition.organizations[].members[].credentials[].type

Type: string

*missing*

---

### spec.definition.organizations[].members[].credentials[].userLabel

Type: string

*missing*

---

### spec.definition.organizations[].members[].credentials[].value

Type: string

*missing*

---

### spec.definition.organizations[].members[].disableableCredentialTypes[]

Type: string

*missing*

---

### spec.definition.organizations[].members[].email

Type: string

*missing*

---

### spec.definition.organizations[].members[].emailVerified

Type: boolean

*missing*

---

### spec.definition.organizations[].members[].enabled

Type: boolean

*missing*

---

### spec.definition.organizations[].members[].federatedIdentities[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[identityProvider](#specdefinitionorganizationsmembersfederatedidentitiesidentityprovider)|string||
|[userId](#specdefinitionorganizationsmembersfederatedidentitiesuserid)|string||
|[userName](#specdefinitionorganizationsmembersfederatedidentitiesusername)|string||

*missing*

---

### spec.definition.organizations[].members[].federatedIdentities[].identityProvider

Type: string

*missing*

---

### spec.definition.organizations[].members[].federatedIdentities[].userId

Type: string

*missing*

---

### spec.definition.organizations[].members[].federatedIdentities[].userName

Type: string

*missing*

---

### spec.definition.organizations[].members[].federationLink

Type: string

*missing*

---

### spec.definition.organizations[].members[].firstName

Type: string

*missing*

---

### spec.definition.organizations[].members[].groups[]

Type: string

*missing*

---

### spec.definition.organizations[].members[].id

Type: string

*missing*

---

### spec.definition.organizations[].members[].lastName

Type: string

*missing*

---

### spec.definition.organizations[].members[].membershipType

Type: string

*missing*

---

### spec.definition.organizations[].members[].notBefore

Type: integer

*missing*

---

### spec.definition.organizations[].members[].origin

Type: string

*missing*

---

### spec.definition.organizations[].members[].realmRoles[]

Type: string

*missing*

---

### spec.definition.organizations[].members[].requiredActions[]

Type: string

*missing*

---

### spec.definition.organizations[].members[].self

Type: string

*missing*

---

### spec.definition.organizations[].members[].serviceAccountClientId

Type: string

*missing*

---

### spec.definition.organizations[].members[].socialLinks[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[socialProvider](#specdefinitionorganizationsmemberssociallinkssocialprovider)|string||
|[socialUserId](#specdefinitionorganizationsmemberssociallinkssocialuserid)|string||
|[socialUsername](#specdefinitionorganizationsmemberssociallinkssocialusername)|string||

*missing*

---

### spec.definition.organizations[].members[].socialLinks[].socialProvider

Type: string

*missing*

---

### spec.definition.organizations[].members[].socialLinks[].socialUserId

Type: string

*missing*

---

### spec.definition.organizations[].members[].socialLinks[].socialUsername

Type: string

*missing*

---

### spec.definition.organizations[].members[].totp

Type: boolean

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[attributes[]](#specdefinitionorganizationsmembersuserprofilemetadataattributes)|object||
|[groups[]](#specdefinitionorganizationsmembersuserprofilemetadatagroups)|object||

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata.attributes[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[annotations](#specdefinitionorganizationsmembersuserprofilemetadataattributesannotations)|object||
|[displayName](#specdefinitionorganizationsmembersuserprofilemetadataattributesdisplayname)|string||
|[group](#specdefinitionorganizationsmembersuserprofilemetadataattributesgroup)|string||
|[multivalued](#specdefinitionorganizationsmembersuserprofilemetadataattributesmultivalued)|boolean||
|[name](#specdefinitionorganizationsmembersuserprofilemetadataattributesname)|string||
|[readOnly](#specdefinitionorganizationsmembersuserprofilemetadataattributesreadonly)|boolean||
|[required](#specdefinitionorganizationsmembersuserprofilemetadataattributesrequired)|boolean||
|[validators](#specdefinitionorganizationsmembersuserprofilemetadataattributesvalidators)|object||

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata.attributes[].annotations

Type: object

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata.attributes[].displayName

Type: string

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata.attributes[].group

Type: string

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata.attributes[].name

Type: string

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata.attributes[].required

Type: boolean

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata.attributes[].validators

Type: object

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata.groups[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[annotations](#specdefinitionorganizationsmembersuserprofilemetadatagroupsannotations)|object||
|[displayDescription](#specdefinitionorganizationsmembersuserprofilemetadatagroupsdisplaydescription)|string||
|[displayHeader](#specdefinitionorganizationsmembersuserprofilemetadatagroupsdisplayheader)|string||
|[name](#specdefinitionorganizationsmembersuserprofilemetadatagroupsname)|string||

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata.groups[].annotations

Type: object

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata.groups[].displayDescription

Type: string

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata.groups[].displayHeader

Type: string

*missing*

---

### spec.definition.organizations[].members[].userProfileMetadata.groups[].name

Type: string

*missing*

---

### spec.definition.organizations[].members[].username

Type: string

*missing*

---

### spec.definition.organizations[].name

Type: string

*missing*

---

### spec.definition.organizations[].redirectUrl

Type: string

*missing*

---

### spec.definition.organizationsEnabled

Type: boolean

*missing*

---

### spec.definition.otpPolicyAlgorithm

Type: string

*missing*

---

### spec.definition.otpPolicyCodeReusable

Type: boolean

*missing*

---

### spec.definition.otpPolicyDigits

Type: integer

*missing*

---

### spec.definition.otpPolicyInitialCounter

Type: integer

*missing*

---

### spec.definition.otpPolicyLookAheadWindow

Type: integer

*missing*

---

### spec.definition.otpPolicyPeriod

Type: integer

*missing*

---

### spec.definition.otpPolicyType

Type: string

*missing*

---

### spec.definition.otpSupportedApplications[]

Type: string

*missing*

---

### spec.definition.passwordCredentialGrantAllowed

Type: boolean

*missing*

---

### spec.definition.passwordPolicy

Type: string

*missing*

---

### spec.definition.permanentLockout

Type: boolean

*missing*

---

### spec.definition.privateKey

Type: string

*missing*

---

### spec.definition.protocolMappers[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionprotocolmappersconfig)|object||
|[consentRequired](#specdefinitionprotocolmappersconsentrequired)|boolean||
|[consentText](#specdefinitionprotocolmappersconsenttext)|string||
|[id](#specdefinitionprotocolmappersid)|string||
|[name](#specdefinitionprotocolmappersname)|string||
|[protocol](#specdefinitionprotocolmappersprotocol)|string||
|[protocolMapper](#specdefinitionprotocolmappersprotocolmapper)|string||

*missing*

---

### spec.definition.protocolMappers[].config

Type: object

*missing*

---

### spec.definition.protocolMappers[].consentRequired

Type: boolean

*missing*

---

### spec.definition.protocolMappers[].consentText

Type: string

*missing*

---

### spec.definition.protocolMappers[].id

Type: string

*missing*

---

### spec.definition.protocolMappers[].name

Type: string

*missing*

---

### spec.definition.protocolMappers[].protocol

Type: string

*missing*

---

### spec.definition.protocolMappers[].protocolMapper

Type: string

*missing*

---

### spec.definition.publicKey

Type: string

*missing*

---

### spec.definition.quickLoginCheckMilliSeconds

Type: integer

*missing*

---

### spec.definition.realm

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.definition.realmCacheEnabled

Type: boolean

*missing*

---

### spec.definition.refreshTokenMaxReuse

Type: integer

*missing*

---

### spec.definition.registrationAllowed

Type: boolean

*missing*

---

### spec.definition.registrationEmailAsUsername

Type: boolean

*missing*

---

### spec.definition.registrationFlow

Type: string

*missing*

---

### spec.definition.rememberMe

Type: boolean

*missing*

---

### spec.definition.requiredActions[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[alias](#specdefinitionrequiredactionsalias)|string||
|[config](#specdefinitionrequiredactionsconfig)|object||
|[defaultAction](#specdefinitionrequiredactionsdefaultaction)|boolean||
|[enabled](#specdefinitionrequiredactionsenabled)|boolean||
|[name](#specdefinitionrequiredactionsname)|string||
|[priority](#specdefinitionrequiredactionspriority)|integer||
|[providerId](#specdefinitionrequiredactionsproviderid)|string||

*missing*

---

### spec.definition.requiredActions[].alias

Type: string

*missing*

---

### spec.definition.requiredActions[].config

Type: object

*missing*

---

### spec.definition.requiredActions[].defaultAction

Type: boolean

*missing*

---

### spec.definition.requiredActions[].enabled

Type: boolean

*missing*

---

### spec.definition.requiredActions[].name

Type: string

*missing*

---

### spec.definition.requiredActions[].priority

Type: integer

*missing*

---

### spec.definition.requiredActions[].providerId

Type: string

*missing*

---

### spec.definition.requiredCredentials[]

Type: string

*missing*

---

### spec.definition.resetCredentialsFlow

Type: string

*missing*

---

### spec.definition.resetPasswordAllowed

Type: boolean

*missing*

---

### spec.definition.revokeRefreshToken

Type: boolean

*missing*

---

### spec.definition.roles

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[application](#specdefinitionrolesapplication)|object||
|[client](#specdefinitionrolesclient)|object||
|[realm[]](#specdefinitionrolesrealm)|object||

*missing*

---

### spec.definition.roles.application

Type: object

*missing*

---

### spec.definition.roles.client

Type: object

*missing*

---

### spec.definition.roles.realm[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[attributes](#specdefinitionrolesrealmattributes)|object||
|[clientRole](#specdefinitionrolesrealmclientrole)|boolean||
|[composite](#specdefinitionrolesrealmcomposite)|boolean||
|[composites](#specdefinitionrolesrealmcomposites)|object||
|[containerId](#specdefinitionrolesrealmcontainerid)|string||
|[description](#specdefinitionrolesrealmdescription)|string||
|[id](#specdefinitionrolesrealmid)|string||
|[name](#specdefinitionrolesrealmname)|string||
|[scopeParamRequired](#specdefinitionrolesrealmscopeparamrequired)|boolean||

*missing*

---

### spec.definition.roles.realm[].attributes

Type: object

*missing*

---

### spec.definition.roles.realm[].clientRole

Type: boolean

*missing*

---

### spec.definition.roles.realm[].composite

Type: boolean

*missing*

---

### spec.definition.roles.realm[].composites

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[application](#specdefinitionrolesrealmcompositesapplication)|object||
|[client](#specdefinitionrolesrealmcompositesclient)|object||
|[realm[]](#specdefinitionrolesrealmcompositesrealm)|string||

*missing*

---

### spec.definition.roles.realm[].composites.application

Type: object

*missing*

---

### spec.definition.roles.realm[].composites.client

Type: object

*missing*

---

### spec.definition.roles.realm[].composites.realm[]

Type: string

*missing*

---

### spec.definition.roles.realm[].containerId

Type: string

*missing*

---

### spec.definition.roles.realm[].description

Type: string

*missing*

---

### spec.definition.roles.realm[].id

Type: string

*missing*

---

### spec.definition.roles.realm[].name

Type: string

*missing*

---

### spec.definition.roles.realm[].scopeParamRequired

Type: boolean

*missing*

---

### spec.definition.scopeMappings[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[client](#specdefinitionscopemappingsclient)|string||
|[clientScope](#specdefinitionscopemappingsclientscope)|string||
|[clientTemplate](#specdefinitionscopemappingsclienttemplate)|string||
|[roles[]](#specdefinitionscopemappingsroles)|string||
|[self](#specdefinitionscopemappingsself)|string||

*missing*

---

### spec.definition.scopeMappings[].client

Type: string

*missing*

---

### spec.definition.scopeMappings[].clientScope

Type: string

*missing*

---

### spec.definition.scopeMappings[].clientTemplate

Type: string

*missing*

---

### spec.definition.scopeMappings[].roles[]

Type: string

*missing*

---

### spec.definition.scopeMappings[].self

Type: string

*missing*

---

### spec.definition.smtpServer

Type: object

*missing*

---

### spec.definition.social

Type: boolean

*missing*

---

### spec.definition.socialProviders

Type: object

*missing*

---

### spec.definition.sslRequired

Type: string

*missing*

---

### spec.definition.ssoSessionIdleTimeout

Type: integer

*missing*

---

### spec.definition.ssoSessionIdleTimeoutRememberMe

Type: integer

*missing*

---

### spec.definition.ssoSessionMaxLifespan

Type: integer

*missing*

---

### spec.definition.ssoSessionMaxLifespanRememberMe

Type: integer

*missing*

---

### spec.definition.supportedLocales[]

Type: string

*missing*

---

### spec.definition.updateProfileOnInitialSocialLogin

Type: boolean

*missing*

---

### spec.definition.userCacheEnabled

Type: boolean

*missing*

---

### spec.definition.userFederationMappers[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionuserfederationmappersconfig)|object||
|[federationMapperType](#specdefinitionuserfederationmappersfederationmappertype)|string||
|[federationProviderDisplayName](#specdefinitionuserfederationmappersfederationproviderdisplayname)|string||
|[id](#specdefinitionuserfederationmappersid)|string||
|[name](#specdefinitionuserfederationmappersname)|string||

*missing*

---

### spec.definition.userFederationMappers[].config

Type: object

*missing*

---

### spec.definition.userFederationMappers[].federationMapperType

Type: string

*missing*

---

### spec.definition.userFederationMappers[].federationProviderDisplayName

Type: string

*missing*

---

### spec.definition.userFederationMappers[].id

Type: string

*missing*

---

### spec.definition.userFederationMappers[].name

Type: string

*missing*

---

### spec.definition.userFederationProviders[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[changedSyncPeriod](#specdefinitionuserfederationproviderschangedsyncperiod)|integer||
|[config](#specdefinitionuserfederationprovidersconfig)|object||
|[displayName](#specdefinitionuserfederationprovidersdisplayname)|string||
|[fullSyncPeriod](#specdefinitionuserfederationprovidersfullsyncperiod)|integer||
|[id](#specdefinitionuserfederationprovidersid)|string||
|[lastSync](#specdefinitionuserfederationproviderslastsync)|integer||
|[priority](#specdefinitionuserfederationproviderspriority)|integer||
|[providerName](#specdefinitionuserfederationprovidersprovidername)|string||

*missing*

---

### spec.definition.userFederationProviders[].changedSyncPeriod

Type: integer

*missing*

---

### spec.definition.userFederationProviders[].config

Type: object

*missing*

---

### spec.definition.userFederationProviders[].displayName

Type: string

*missing*

---

### spec.definition.userFederationProviders[].fullSyncPeriod

Type: integer

*missing*

---

### spec.definition.userFederationProviders[].id

Type: string

*missing*

---

### spec.definition.userFederationProviders[].lastSync

Type: integer

*missing*

---

### spec.definition.userFederationProviders[].priority

Type: integer

*missing*

---

### spec.definition.userFederationProviders[].providerName

Type: string

*missing*

---

### spec.definition.userManagedAccessAllowed

Type: boolean

*missing*

---

### spec.definition.users[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[access](#specdefinitionusersaccess)|object||
|[applicationRoles](#specdefinitionusersapplicationroles)|object||
|[attributes](#specdefinitionusersattributes)|object||
|[clientConsents[]](#specdefinitionusersclientconsents)|object||
|[clientRoles](#specdefinitionusersclientroles)|object||
|[createdTimestamp](#specdefinitionuserscreatedtimestamp)|integer||
|[credentials[]](#specdefinitionuserscredentials)|object||
|[disableableCredentialTypes[]](#specdefinitionusersdisableablecredentialtypes)|string||
|[email](#specdefinitionusersemail)|string||
|[emailVerified](#specdefinitionusersemailverified)|boolean||
|[enabled](#specdefinitionusersenabled)|boolean||
|[federatedIdentities[]](#specdefinitionusersfederatedidentities)|object||
|[federationLink](#specdefinitionusersfederationlink)|string||
|[firstName](#specdefinitionusersfirstname)|string||
|[groups[]](#specdefinitionusersgroups)|string||
|[id](#specdefinitionusersid)|string||
|[lastName](#specdefinitionuserslastname)|string||
|[notBefore](#specdefinitionusersnotbefore)|integer||
|[origin](#specdefinitionusersorigin)|string||
|[realmRoles[]](#specdefinitionusersrealmroles)|string||
|[requiredActions[]](#specdefinitionusersrequiredactions)|string||
|[self](#specdefinitionusersself)|string||
|[serviceAccountClientId](#specdefinitionusersserviceaccountclientid)|string||
|[socialLinks[]](#specdefinitionuserssociallinks)|object||
|[totp](#specdefinitionuserstotp)|boolean||
|[userProfileMetadata](#specdefinitionusersuserprofilemetadata)|object||
|[username](#specdefinitionusersusername)|string||

*missing*

---

### spec.definition.users[].access

Type: object

*missing*

---

### spec.definition.users[].applicationRoles

Type: object

*missing*

---

### spec.definition.users[].attributes

Type: object

*missing*

---

### spec.definition.users[].clientConsents[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clientId](#specdefinitionusersclientconsentsclientid)|string||
|[createdDate](#specdefinitionusersclientconsentscreateddate)|integer||
|[grantedClientScopes[]](#specdefinitionusersclientconsentsgrantedclientscopes)|string||
|[grantedRealmRoles[]](#specdefinitionusersclientconsentsgrantedrealmroles)|string||
|[lastUpdatedDate](#specdefinitionusersclientconsentslastupdateddate)|integer||

*missing*

---

### spec.definition.users[].clientConsents[].clientId

Type: string

*missing*

---

### spec.definition.users[].clientConsents[].createdDate

Type: integer

*missing*

---

### spec.definition.users[].clientConsents[].grantedClientScopes[]

Type: string

*missing*

---

### spec.definition.users[].clientConsents[].grantedRealmRoles[]

Type: string

*missing*

---

### spec.definition.users[].clientConsents[].lastUpdatedDate

Type: integer

*missing*

---

### spec.definition.users[].clientRoles

Type: object

*missing*

---

### spec.definition.users[].createdTimestamp

Type: integer

*missing*

---

### spec.definition.users[].credentials[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[algorithm](#specdefinitionuserscredentialsalgorithm)|string||
|[config](#specdefinitionuserscredentialsconfig)|object||
|[counter](#specdefinitionuserscredentialscounter)|integer||
|[createdDate](#specdefinitionuserscredentialscreateddate)|integer||
|[credentialData](#specdefinitionuserscredentialscredentialdata)|string||
|[device](#specdefinitionuserscredentialsdevice)|string||
|[digits](#specdefinitionuserscredentialsdigits)|integer||
|[hashIterations](#specdefinitionuserscredentialshashiterations)|integer||
|[hashedSaltedValue](#specdefinitionuserscredentialshashedsaltedvalue)|string||
|[id](#specdefinitionuserscredentialsid)|string||
|[period](#specdefinitionuserscredentialsperiod)|integer||
|[priority](#specdefinitionuserscredentialspriority)|integer||
|[salt](#specdefinitionuserscredentialssalt)|string||
|[secretData](#specdefinitionuserscredentialssecretdata)|string||
|[temporary](#specdefinitionuserscredentialstemporary)|boolean||
|[type](#specdefinitionuserscredentialstype)|string||
|[userLabel](#specdefinitionuserscredentialsuserlabel)|string||
|[value](#specdefinitionuserscredentialsvalue)|string||

*missing*

---

### spec.definition.users[].credentials[].algorithm

Type: string

*missing*

---

### spec.definition.users[].credentials[].config

Type: object

*missing*

---

### spec.definition.users[].credentials[].counter

Type: integer

*missing*

---

### spec.definition.users[].credentials[].createdDate

Type: integer

*missing*

---

### spec.definition.users[].credentials[].credentialData

Type: string

*missing*

---

### spec.definition.users[].credentials[].device

Type: string

*missing*

---

### spec.definition.users[].credentials[].digits

Type: integer

*missing*

---

### spec.definition.users[].credentials[].hashIterations

Type: integer

*missing*

---

### spec.definition.users[].credentials[].hashedSaltedValue

Type: string

*missing*

---

### spec.definition.users[].credentials[].id

Type: string

*missing*

---

### spec.definition.users[].credentials[].period

Type: integer

*missing*

---

### spec.definition.users[].credentials[].priority

Type: integer

*missing*

---

### spec.definition.users[].credentials[].salt

Type: string

*missing*

---

### spec.definition.users[].credentials[].secretData

Type: string

*missing*

---

### spec.definition.users[].credentials[].temporary

Type: boolean

*missing*

---

### spec.definition.users[].credentials[].type

Type: string

*missing*

---

### spec.definition.users[].credentials[].userLabel

Type: string

*missing*

---

### spec.definition.users[].credentials[].value

Type: string

*missing*

---

### spec.definition.users[].disableableCredentialTypes[]

Type: string

*missing*

---

### spec.definition.users[].email

Type: string

*missing*

---

### spec.definition.users[].emailVerified

Type: boolean

*missing*

---

### spec.definition.users[].enabled

Type: boolean

*missing*

---

### spec.definition.users[].federatedIdentities[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[identityProvider](#specdefinitionusersfederatedidentitiesidentityprovider)|string||
|[userId](#specdefinitionusersfederatedidentitiesuserid)|string||
|[userName](#specdefinitionusersfederatedidentitiesusername)|string||

*missing*

---

### spec.definition.users[].federatedIdentities[].identityProvider

Type: string

*missing*

---

### spec.definition.users[].federatedIdentities[].userId

Type: string

*missing*

---

### spec.definition.users[].federatedIdentities[].userName

Type: string

*missing*

---

### spec.definition.users[].federationLink

Type: string

*missing*

---

### spec.definition.users[].firstName

Type: string

*missing*

---

### spec.definition.users[].groups[]

Type: string

*missing*

---

### spec.definition.users[].id

Type: string

*missing*

---

### spec.definition.users[].lastName

Type: string

*missing*

---

### spec.definition.users[].notBefore

Type: integer

*missing*

---

### spec.definition.users[].origin

Type: string

*missing*

---

### spec.definition.users[].realmRoles[]

Type: string

*missing*

---

### spec.definition.users[].requiredActions[]

Type: string

*missing*

---

### spec.definition.users[].self

Type: string

*missing*

---

### spec.definition.users[].serviceAccountClientId

Type: string

*missing*

---

### spec.definition.users[].socialLinks[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[socialProvider](#specdefinitionuserssociallinkssocialprovider)|string||
|[socialUserId](#specdefinitionuserssociallinkssocialuserid)|string||
|[socialUsername](#specdefinitionuserssociallinkssocialusername)|string||

*missing*

---

### spec.definition.users[].socialLinks[].socialProvider

Type: string

*missing*

---

### spec.definition.users[].socialLinks[].socialUserId

Type: string

*missing*

---

### spec.definition.users[].socialLinks[].socialUsername

Type: string

*missing*

---

### spec.definition.users[].totp

Type: boolean

*missing*

---

### spec.definition.users[].userProfileMetadata

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[attributes[]](#specdefinitionusersuserprofilemetadataattributes)|object||
|[groups[]](#specdefinitionusersuserprofilemetadatagroups)|object||

*missing*

---

### spec.definition.users[].userProfileMetadata.attributes[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[annotations](#specdefinitionusersuserprofilemetadataattributesannotations)|object||
|[displayName](#specdefinitionusersuserprofilemetadataattributesdisplayname)|string||
|[group](#specdefinitionusersuserprofilemetadataattributesgroup)|string||
|[multivalued](#specdefinitionusersuserprofilemetadataattributesmultivalued)|boolean||
|[name](#specdefinitionusersuserprofilemetadataattributesname)|string||
|[readOnly](#specdefinitionusersuserprofilemetadataattributesreadonly)|boolean||
|[required](#specdefinitionusersuserprofilemetadataattributesrequired)|boolean||
|[validators](#specdefinitionusersuserprofilemetadataattributesvalidators)|object||

*missing*

---

### spec.definition.users[].userProfileMetadata.attributes[].annotations

Type: object

*missing*

---

### spec.definition.users[].userProfileMetadata.attributes[].displayName

Type: string

*missing*

---

### spec.definition.users[].userProfileMetadata.attributes[].group

Type: string

*missing*

---

### spec.definition.users[].userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

---

### spec.definition.users[].userProfileMetadata.attributes[].name

Type: string

*missing*

---

### spec.definition.users[].userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

---

### spec.definition.users[].userProfileMetadata.attributes[].required

Type: boolean

*missing*

---

### spec.definition.users[].userProfileMetadata.attributes[].validators

Type: object

*missing*

---

### spec.definition.users[].userProfileMetadata.groups[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[annotations](#specdefinitionusersuserprofilemetadatagroupsannotations)|object||
|[displayDescription](#specdefinitionusersuserprofilemetadatagroupsdisplaydescription)|string||
|[displayHeader](#specdefinitionusersuserprofilemetadatagroupsdisplayheader)|string||
|[name](#specdefinitionusersuserprofilemetadatagroupsname)|string||

*missing*

---

### spec.definition.users[].userProfileMetadata.groups[].annotations

Type: object

*missing*

---

### spec.definition.users[].userProfileMetadata.groups[].displayDescription

Type: string

*missing*

---

### spec.definition.users[].userProfileMetadata.groups[].displayHeader

Type: string

*missing*

---

### spec.definition.users[].userProfileMetadata.groups[].name

Type: string

*missing*

---

### spec.definition.users[].username

Type: string

*missing*

---

### spec.definition.verifyEmail

Type: boolean

*missing*

---

### spec.definition.waitIncrementSeconds

Type: integer

*missing*

---

### spec.definition.webAuthnPolicyAcceptableAaguids[]

Type: string

*missing*

---

### spec.definition.webAuthnPolicyAttestationConveyancePreference

Type: string

*missing*

---

### spec.definition.webAuthnPolicyAuthenticatorAttachment

Type: string

*missing*

---

### spec.definition.webAuthnPolicyAvoidSameAuthenticatorRegister

Type: boolean

*missing*

---

### spec.definition.webAuthnPolicyCreateTimeout

Type: integer

*missing*

---

### spec.definition.webAuthnPolicyExtraOrigins[]

Type: string

*missing*

---

### spec.definition.webAuthnPolicyPasswordlessAcceptableAaguids[]

Type: string

*missing*

---

### spec.definition.webAuthnPolicyPasswordlessAttestationConveyancePreference

Type: string

*missing*

---

### spec.definition.webAuthnPolicyPasswordlessAuthenticatorAttachment

Type: string

*missing*

---

### spec.definition.webAuthnPolicyPasswordlessAvoidSameAuthenticatorRegister

Type: boolean

*missing*

---

### spec.definition.webAuthnPolicyPasswordlessCreateTimeout

Type: integer

*missing*

---

### spec.definition.webAuthnPolicyPasswordlessExtraOrigins[]

Type: string

*missing*

---

### spec.definition.webAuthnPolicyPasswordlessRequireResidentKey

Type: string

*missing*

---

### spec.definition.webAuthnPolicyPasswordlessRpEntityName

Type: string

*missing*

---

### spec.definition.webAuthnPolicyPasswordlessRpId

Type: string

*missing*

---

### spec.definition.webAuthnPolicyPasswordlessSignatureAlgorithms[]

Type: string

*missing*

---

### spec.definition.webAuthnPolicyPasswordlessUserVerificationRequirement

Type: string

*missing*

---

### spec.definition.webAuthnPolicyRequireResidentKey

Type: string

*missing*

---

### spec.definition.webAuthnPolicyRpEntityName

Type: string

*missing*

---

### spec.definition.webAuthnPolicyRpId

Type: string

*missing*

---

### spec.definition.webAuthnPolicySignatureAlgorithms[]

Type: string

*missing*

---

### spec.definition.webAuthnPolicyUserVerificationRequirement

Type: string

*missing*

---

### spec.instanceRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the instance to which this realm belongs

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
|[conditions[]](#statusconditions)|object||
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