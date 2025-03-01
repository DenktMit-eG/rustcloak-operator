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
|[spec.definition.adminPermissionsEnabled](#specdefinitionadminpermissionsenabled)|boolean||
|[spec.definition.adminTheme](#specdefinitionadmintheme)|string||
|[spec.definition.applicationScopeMappings](#specdefinitionapplicationscopemappings)|object||
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.attributes.adminEventsExpiration](#specdefinitionattributesadmineventsexpiration)|string||
|[spec.definition.attributes.cibaAuthRequestedUserHint](#specdefinitionattributescibaauthrequesteduserhint)|string||
|[spec.definition.attributes.cibaBackchannelTokenDeliveryMode](#specdefinitionattributescibabackchanneltokendeliverymode)|string||
|[spec.definition.attributes.cibaExpiresIn](#specdefinitionattributescibaexpiresin)|string||
|[spec.definition.attributes.cibaInterval](#specdefinitionattributescibainterval)|string||
|[spec.definition.attributes.frontendUrl](#specdefinitionattributesfrontendurl)|string||
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
|[spec.definition.browserSecurityHeaders.contentSecurityPolicy](#specdefinitionbrowsersecurityheaderscontentsecuritypolicy)|string||
|[spec.definition.browserSecurityHeaders.contentSecurityPolicyReportOnly](#specdefinitionbrowsersecurityheaderscontentsecuritypolicyreportonly)|string||
|[spec.definition.browserSecurityHeaders.strictTransportSecurity](#specdefinitionbrowsersecurityheadersstricttransportsecurity)|string||
|[spec.definition.browserSecurityHeaders.xContentTypeOptions](#specdefinitionbrowsersecurityheadersxcontenttypeoptions)|string||
|[spec.definition.browserSecurityHeaders.xFrameOptions](#specdefinitionbrowsersecurityheadersxframeoptions)|string||
|[spec.definition.browserSecurityHeaders.xRobotsTag](#specdefinitionbrowsersecurityheadersxrobotstag)|string||
|[spec.definition.browserSecurityHeaders.xXSSProtection](#specdefinitionbrowsersecurityheadersxxssprotection)|string||
|[spec.definition.bruteForceDetection](#specdefinitionbruteforcedetection)|object||
|[spec.definition.bruteForceProtected](#specdefinitionbruteforceprotected)|boolean||
|[spec.definition.bruteForceStrategy](#specdefinitionbruteforcestrategy)|string||
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
|[spec.definition.identityProviderMappers[].config.attribute.friendly.name](#specdefinitionidentityprovidermappersconfigattributefriendlyname)|string||
|[spec.definition.identityProviderMappers[].config.attribute.name.format](#specdefinitionidentityprovidermappersconfigattributenameformat)|string||
|[spec.definition.identityProviderMappers[].config.syncMode](#specdefinitionidentityprovidermappersconfigsyncmode)|string||
|[spec.definition.identityProviderMappers[].config.user.attribute](#specdefinitionidentityprovidermappersconfiguserattribute)|string||
|[spec.definition.identityProviderMappers[].id](#specdefinitionidentityprovidermappersid)|string||
|[spec.definition.identityProviderMappers[].identityProviderAlias](#specdefinitionidentityprovidermappersidentityprovideralias)|string||
|[spec.definition.identityProviderMappers[].identityProviderMapper](#specdefinitionidentityprovidermappersidentityprovidermapper)|string||
|[spec.definition.identityProviderMappers[].name](#specdefinitionidentityprovidermappersname)|string||
|[spec.definition.identityProviders[]](#specdefinitionidentityproviders)|object||
|[spec.definition.identityProviders[].addReadTokenRoleOnCreate](#specdefinitionidentityprovidersaddreadtokenroleoncreate)|boolean||
|[spec.definition.identityProviders[].alias](#specdefinitionidentityprovidersalias)|string||
|[spec.definition.identityProviders[].authenticateByDefault](#specdefinitionidentityprovidersauthenticatebydefault)|boolean||
|[spec.definition.identityProviders[].config](#specdefinitionidentityprovidersconfig)|object||
|[spec.definition.identityProviders[].config.allowCreate](#specdefinitionidentityprovidersconfigallowcreate)|string||
|[spec.definition.identityProviders[].config.allowedClockSkew](#specdefinitionidentityprovidersconfigallowedclockskew)|string||
|[spec.definition.identityProviders[].config.attributeConsumingServiceIndex](#specdefinitionidentityprovidersconfigattributeconsumingserviceindex)|string||
|[spec.definition.identityProviders[].config.authnContextClassRefs](#specdefinitionidentityprovidersconfigauthncontextclassrefs)|string||
|[spec.definition.identityProviders[].config.authnContextComparisonType](#specdefinitionidentityprovidersconfigauthncontextcomparisontype)|string||
|[spec.definition.identityProviders[].config.authnContextDeclRefs](#specdefinitionidentityprovidersconfigauthncontextdeclrefs)|string||
|[spec.definition.identityProviders[].config.backchannelSupported](#specdefinitionidentityprovidersconfigbackchannelsupported)|string||
|[spec.definition.identityProviders[].config.encryptionAlgorithm](#specdefinitionidentityprovidersconfigencryptionalgorithm)|string||
|[spec.definition.identityProviders[].config.entityId](#specdefinitionidentityprovidersconfigentityid)|string||
|[spec.definition.identityProviders[].config.forceAuthn](#specdefinitionidentityprovidersconfigforceauthn)|string||
|[spec.definition.identityProviders[].config.hideOnLoginPage](#specdefinitionidentityprovidersconfighideonloginpage)|string||
|[spec.definition.identityProviders[].config.idpEntityId](#specdefinitionidentityprovidersconfigidpentityid)|string||
|[spec.definition.identityProviders[].config.loginHint](#specdefinitionidentityprovidersconfigloginhint)|string||
|[spec.definition.identityProviders[].config.nameIDPolicyFormat](#specdefinitionidentityprovidersconfignameidpolicyformat)|string||
|[spec.definition.identityProviders[].config.postBindingAuthnRequest](#specdefinitionidentityprovidersconfigpostbindingauthnrequest)|string||
|[spec.definition.identityProviders[].config.postBindingLogout](#specdefinitionidentityprovidersconfigpostbindinglogout)|string||
|[spec.definition.identityProviders[].config.postBindingResponse](#specdefinitionidentityprovidersconfigpostbindingresponse)|string||
|[spec.definition.identityProviders[].config.principalAttribute](#specdefinitionidentityprovidersconfigprincipalattribute)|string||
|[spec.definition.identityProviders[].config.principalType](#specdefinitionidentityprovidersconfigprincipaltype)|string||
|[spec.definition.identityProviders[].config.signSpMetadata](#specdefinitionidentityprovidersconfigsignspmetadata)|string||
|[spec.definition.identityProviders[].config.signatureAlgorithm](#specdefinitionidentityprovidersconfigsignaturealgorithm)|string||
|[spec.definition.identityProviders[].config.signingCertificate](#specdefinitionidentityprovidersconfigsigningcertificate)|string||
|[spec.definition.identityProviders[].config.singleSignOnServiceUrl](#specdefinitionidentityprovidersconfigsinglesignonserviceurl)|string||
|[spec.definition.identityProviders[].config.syncMode](#specdefinitionidentityprovidersconfigsyncmode)|string||
|[spec.definition.identityProviders[].config.validateSignature](#specdefinitionidentityprovidersconfigvalidatesignature)|string||
|[spec.definition.identityProviders[].config.wantAssertionsEncrypted](#specdefinitionidentityprovidersconfigwantassertionsencrypted)|string||
|[spec.definition.identityProviders[].config.wantAssertionsSigned](#specdefinitionidentityprovidersconfigwantassertionssigned)|string||
|[spec.definition.identityProviders[].config.wantAuthnRequestsSigned](#specdefinitionidentityprovidersconfigwantauthnrequestssigned)|string||
|[spec.definition.identityProviders[].config.xmlSigKeyInfoKeyNameTransformer](#specdefinitionidentityprovidersconfigxmlsigkeyinfokeynametransformer)|string||
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
|[spec.definition.organizations[].identityProviders[].config.allowCreate](#specdefinitionorganizationsidentityprovidersconfigallowcreate)|string||
|[spec.definition.organizations[].identityProviders[].config.allowedClockSkew](#specdefinitionorganizationsidentityprovidersconfigallowedclockskew)|string||
|[spec.definition.organizations[].identityProviders[].config.attributeConsumingServiceIndex](#specdefinitionorganizationsidentityprovidersconfigattributeconsumingserviceindex)|string||
|[spec.definition.organizations[].identityProviders[].config.authnContextClassRefs](#specdefinitionorganizationsidentityprovidersconfigauthncontextclassrefs)|string||
|[spec.definition.organizations[].identityProviders[].config.authnContextComparisonType](#specdefinitionorganizationsidentityprovidersconfigauthncontextcomparisontype)|string||
|[spec.definition.organizations[].identityProviders[].config.authnContextDeclRefs](#specdefinitionorganizationsidentityprovidersconfigauthncontextdeclrefs)|string||
|[spec.definition.organizations[].identityProviders[].config.backchannelSupported](#specdefinitionorganizationsidentityprovidersconfigbackchannelsupported)|string||
|[spec.definition.organizations[].identityProviders[].config.encryptionAlgorithm](#specdefinitionorganizationsidentityprovidersconfigencryptionalgorithm)|string||
|[spec.definition.organizations[].identityProviders[].config.entityId](#specdefinitionorganizationsidentityprovidersconfigentityid)|string||
|[spec.definition.organizations[].identityProviders[].config.forceAuthn](#specdefinitionorganizationsidentityprovidersconfigforceauthn)|string||
|[spec.definition.organizations[].identityProviders[].config.hideOnLoginPage](#specdefinitionorganizationsidentityprovidersconfighideonloginpage)|string||
|[spec.definition.organizations[].identityProviders[].config.idpEntityId](#specdefinitionorganizationsidentityprovidersconfigidpentityid)|string||
|[spec.definition.organizations[].identityProviders[].config.loginHint](#specdefinitionorganizationsidentityprovidersconfigloginhint)|string||
|[spec.definition.organizations[].identityProviders[].config.nameIDPolicyFormat](#specdefinitionorganizationsidentityprovidersconfignameidpolicyformat)|string||
|[spec.definition.organizations[].identityProviders[].config.postBindingAuthnRequest](#specdefinitionorganizationsidentityprovidersconfigpostbindingauthnrequest)|string||
|[spec.definition.organizations[].identityProviders[].config.postBindingLogout](#specdefinitionorganizationsidentityprovidersconfigpostbindinglogout)|string||
|[spec.definition.organizations[].identityProviders[].config.postBindingResponse](#specdefinitionorganizationsidentityprovidersconfigpostbindingresponse)|string||
|[spec.definition.organizations[].identityProviders[].config.principalAttribute](#specdefinitionorganizationsidentityprovidersconfigprincipalattribute)|string||
|[spec.definition.organizations[].identityProviders[].config.principalType](#specdefinitionorganizationsidentityprovidersconfigprincipaltype)|string||
|[spec.definition.organizations[].identityProviders[].config.signSpMetadata](#specdefinitionorganizationsidentityprovidersconfigsignspmetadata)|string||
|[spec.definition.organizations[].identityProviders[].config.signatureAlgorithm](#specdefinitionorganizationsidentityprovidersconfigsignaturealgorithm)|string||
|[spec.definition.organizations[].identityProviders[].config.signingCertificate](#specdefinitionorganizationsidentityprovidersconfigsigningcertificate)|string||
|[spec.definition.organizations[].identityProviders[].config.singleSignOnServiceUrl](#specdefinitionorganizationsidentityprovidersconfigsinglesignonserviceurl)|string||
|[spec.definition.organizations[].identityProviders[].config.syncMode](#specdefinitionorganizationsidentityprovidersconfigsyncmode)|string||
|[spec.definition.organizations[].identityProviders[].config.validateSignature](#specdefinitionorganizationsidentityprovidersconfigvalidatesignature)|string||
|[spec.definition.organizations[].identityProviders[].config.wantAssertionsEncrypted](#specdefinitionorganizationsidentityprovidersconfigwantassertionsencrypted)|string||
|[spec.definition.organizations[].identityProviders[].config.wantAssertionsSigned](#specdefinitionorganizationsidentityprovidersconfigwantassertionssigned)|string||
|[spec.definition.organizations[].identityProviders[].config.wantAuthnRequestsSigned](#specdefinitionorganizationsidentityprovidersconfigwantauthnrequestssigned)|string||
|[spec.definition.organizations[].identityProviders[].config.xmlSigKeyInfoKeyNameTransformer](#specdefinitionorganizationsidentityprovidersconfigxmlsigkeyinfokeynametransformer)|string||
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
|[spec.definition.smtpServer.auth](#specdefinitionsmtpserverauth)|string||
|[spec.definition.smtpServer.envelopeFrom](#specdefinitionsmtpserverenvelopefrom)|string||
|[spec.definition.smtpServer.from](#specdefinitionsmtpserverfrom)|string||
|[spec.definition.smtpServer.fromDisplayName](#specdefinitionsmtpserverfromdisplayname)|string||
|[spec.definition.smtpServer.host](#specdefinitionsmtpserverhost)|string||
|[spec.definition.smtpServer.password](#specdefinitionsmtpserverpassword)|string||
|[spec.definition.smtpServer.port](#specdefinitionsmtpserverport)|string||
|[spec.definition.smtpServer.replyTo](#specdefinitionsmtpserverreplyto)|string||
|[spec.definition.smtpServer.replyToDisplayName](#specdefinitionsmtpserverreplytodisplayname)|string||
|[spec.definition.smtpServer.ssl](#specdefinitionsmtpserverssl)|string||
|[spec.definition.smtpServer.starttls](#specdefinitionsmtpserverstarttls)|string||
|[spec.definition.smtpServer.user](#specdefinitionsmtpserveruser)|string||
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
|[spec.definition.verifiableCredentialsEnabled](#specdefinitionverifiablecredentialsenabled)|boolean||
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
|[adminPermissionsEnabled](#specdefinitionadminpermissionsenabled)|boolean||
|[adminTheme](#specdefinitionadmintheme)|string||
|[applicationScopeMappings](#specdefinitionapplicationscopemappings)|object||
|[attributes](#specdefinitionattributes)|object||
|[authenticationFlows[]](#specdefinitionauthenticationflows)|object||
|[authenticatorConfig[]](#specdefinitionauthenticatorconfig)|object||
|[browserFlow](#specdefinitionbrowserflow)|string||
|[browserSecurityHeaders](#specdefinitionbrowsersecurityheaders)|object||
|[bruteForceDetection](#specdefinitionbruteforcedetection)|object||
|[bruteForceProtected](#specdefinitionbruteforceprotected)|boolean||
|[bruteForceStrategy](#specdefinitionbruteforcestrategy)|string||
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
|[verifiableCredentialsEnabled](#specdefinitionverifiablecredentialsenabled)|boolean||
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

RealmRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "accessCodeLifespan": { "title": "Client Login Timeout", "description": "Max time a client has to finish the access token protocol. This should normally be 1 minute.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "accessCodeLifespanLogin": { "title": "Login timeout", "description": "Max time a user has to complete a login. This is recommended to be relatively long, such as 30 minutes or more.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "accessCodeLifespanUserAction": { "title": "Login action timeout", "description": "Max time a user has to complete login related actions like update password or configure totp. This is recommended to be relatively long, such as 5 minutes or more.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "accessTokenLifespan": { "title": "Access Token Lifespan", "description": "Max time before an access token is expired. This value is recommended to be short relative to the SSO timeout.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "accessTokenLifespanForImplicitFlow": { "title": "Access Token Lifespan For Implicit Flow", "description": "Max time before an access token issued during OpenID Connect Implicit Flow is expired. This value is recommended to be shorter than the SSO timeout. There is no possibility to refresh token during implicit flow, that's why there is a separate timeout different to 'Access Token Lifespan'.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "accountTheme": { "title": "Account theme", "description": "Select theme for login, OTP, grant, registration and forgot password pages.", "type": "string" }, "actionTokenGeneratedByAdminLifespan": { "title": "Default Admin-Initiated Action Lifespan", "description": "Maximum time before an action permit sent to a user by administrator is expired. This value is recommended to be long to allow administrators to send e-mails for users that are currently offline. The default timeout can be overridden immediately before issuing the token.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "actionTokenGeneratedByUserLifespan": { "title": "User-Initiated Action Lifespan", "description": "Maximum time before an action permit sent by a user (such as a forgot password e-mail) is expired. This value is recommended to be short because it's expected that the user would react to self-created action quickly.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "adminEventsDetailsEnabled": { "title": "Include representation", "description": "Include JSON representation for create and update requests.", "type": "boolean" }, "adminEventsEnabled": { "title": "Save events", "description": "If enabled, admin events are saved to the database, which makes events available to the Admin UI.", "type": "boolean" }, "adminPermissionsClient": { "$ref": "#/$defs/ClientRepresentation" }, "adminPermissionsEnabled": { "type": "boolean" }, "adminTheme": { "title": "Admin theme", "type": "string" }, "applicationScopeMappings": { "type": "object", "additionalProperties": { "type": "array", "items": { "$ref": "#/$defs/ScopeMappingRepresentation" } } }, "applications": { "type": "array", "items": { "$ref": "#/$defs/ApplicationRepresentation" } }, "attributes": { "type": "object", "properties": { "adminEventsExpiration": { "title": "Expiration", "description": "Sets the expiration for events. Expired events are periodically deleted from the database.", "type": "string", "pattern": "^[0-9]*$" }, "cibaAuthRequestedUserHint": { "title": "Authentication Requested User Hint", "description": "The way of identifying the end-user for whom authentication is being requested. Currently only \"login_hint\" is supported.", "type": "string", "enum": [ "login_hint" ] }, "cibaBackchannelTokenDeliveryMode": { "title": "Backchannel Token Delivery Mode", "description": "Specifies how the CD (Consumption Device) gets the authentication result and related tokens. This mode will be used by default for the CIBA clients, which do not have other mode explicitly set.", "type": "string", "enum": [ "ping", "poll" ] }, "cibaExpiresIn": { "title": "Expires In", "description": "The expiration time of the \"auth_req_id\" in seconds since the authentication request was received.", "type": "string", "pattern": "^[0-9]*$" }, "cibaInterval": { "title": "Interval", "description": "The minimum amount of time in seconds that the CD (Consumption Device) must wait between polling requests to the token endpoint. If set to 0, the CD must use 5 as the default value according to the CIBA specification.", "type": "string", "pattern": "^[0-9]*$" }, "frontendUrl": { "title": "Frontend URL", "description": "Set the frontend URL for the realm. Use in combination with the default hostname provider to override the base URL for frontend requests for a specific realm.", "type": "string" } }, "additionalProperties": { "type": "string" } }, "authenticationFlows": { "type": "array", "items": { "$ref": "#/$defs/AuthenticationFlowRepresentation" } }, "authenticatorConfig": { "type": "array", "items": { "$ref": "#/$defs/AuthenticatorConfigRepresentation" } }, "browserFlow": { "type": "string" }, "browserSecurityHeaders": { "type": "object", "properties": { "contentSecurityPolicy": { "title": "Content-Security-Policy", "description": "Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>", "type": "string" }, "contentSecurityPolicyReportOnly": { "title": "Content-Security-Policy-Report-Only", "description": "For testing Content Security Policies <1>Learn more</1>", "type": "string" }, "strictTransportSecurity": { "title": "HTTP Strict Transport Security (HSTS)", "description": "The Strict-Transport-Security HTTP header tells browsers to always use HTTPS. Once a browser sees this header, it will only visit the site over HTTPS for the time specified (1 year) at max-age, including the subdomains. <1>Learn more</1>", "type": "string" }, "xContentTypeOptions": { "title": "X-Content-Type-Options", "description": "The default value prevents Internet Explorer and Google Chrome from MIME-sniffing a response away from the declared content-type. <1>Learn more</1>", "type": "string" }, "xFrameOptions": { "title": "X-Frame-Options", "description": "Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>", "type": "string" }, "xRobotsTag": { "title": "X-Robots-Tag", "description": "Prevent pages from appearing in search engines. <1>Learn more</1>", "type": "string" }, "xXSSProtection": { "title": "X-XSS-Protection", "description": "This header configures the Cross-site scripting (XSS) filter in your browser. Using the default behaviour, the browser will prevent rendering of the page when a XSS attack is detected. <1>Learn more</1>", "type": "string" } }, "additionalProperties": false }, "bruteForceDetection": { "title": "Brute force detection" }, "bruteForceProtected": { "type": "boolean" }, "bruteForceStrategy": { "title": "Strategy to increase wait time", "description": "Multiple means wait time will be increased only when number of failures are multiples of '{{failureFactor}}'. Linear means each new failure starting at '{{failureFactor}}' will increase wait time.", "$ref": "#/$defs/BruteForceStrategy" }, "certificate": { "type": "string" }, "clientAuthenticationFlow": { "type": "string" }, "clientOfflineSessionIdleTimeout": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "clientOfflineSessionMaxLifespan": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "clientPolicies": { "$ref": "#/$defs/ClientPoliciesRepresentation" }, "clientProfiles": { "$ref": "#/$defs/ClientProfilesRepresentation" }, "clientScopeMappings": { "type": "object", "additionalProperties": { "type": "array", "items": { "$ref": "#/$defs/ScopeMappingRepresentation" } } }, "clientScopes": { "type": "array", "items": { "$ref": "#/$defs/ClientScopeRepresentation" } }, "clientSessionIdleTimeout": { "title": "Client Session Idle", "description": "Time a client session is allowed to be idle before it expires. Tokens are invalidated when a client session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Idle value.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "clientSessionMaxLifespan": { "title": "Client Session Max", "description": "Max time before a client session is expired. Tokens are invalidated when a session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Max value.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "clientTemplates": { "type": "array", "items": { "$ref": "#/$defs/ClientTemplateRepresentation" } }, "clients": { "type": "array", "items": { "$ref": "#/$defs/ClientRepresentation" } }, "codeSecret": { "type": "string" }, "components": { "$ref": "#/$defs/MultivaluedHashMapStringComponentExportRepresentation" }, "defaultDefaultClientScopes": { "type": "array", "items": { "type": "string" } }, "defaultGroups": { "type": "array", "items": { "type": "string" } }, "defaultLocale": { "title": "Default locale", "type": "string" }, "defaultOptionalClientScopes": { "type": "array", "items": { "type": "string" } }, "defaultRole": { "$ref": "#/$defs/RoleRepresentation" }, "defaultRoles": { "type": "array", "items": { "type": "string" } }, "defaultSignatureAlgorithm": { "title": "Default Signature Algorithm", "description": "Default algorithm used to sign tokens for the realm", "type": "string", "enum": [ "EdDSA", "ES256", "ES384", "ES512", "HS256", "HS384", "HS512", "PS256", "PS384", "PS512", "RS256", "RS384", "RS512" ] }, "directGrantFlow": { "type": "string" }, "displayName": { "title": "Display name", "type": "string" }, "displayNameHtml": { "title": "HTML Display name", "type": "string" }, "dockerAuthenticationFlow": { "type": "string" }, "duplicateEmailsAllowed": { "title": "Duplicate emails", "description": "Allow multiple users to have the same email address. Changing this setting will also clear the user's cache. It is recommended to manually update email constraints of existing users in the database after switching off support for duplicate email addresses.", "type": "boolean" }, "editUsernameAllowed": { "title": "Edit username", "description": "If enabled, the username field is editable, readonly otherwise.", "type": "boolean" }, "emailTheme": { "title": "Email theme", "description": "Select a theme for emails that are sent by the server.", "type": "string" }, "enabled": { "type": "boolean" }, "enabledEventTypes": { "type": "array", "items": { "type": "string" } }, "eventsEnabled": { "title": "Save events", "description": "If enabled, user events are saved to the database, which makes events available to the admin and account management UIs.", "type": "boolean" }, "eventsExpiration": { "title": "Expiration", "description": "Sets the expiration for events. Expired events are periodically deleted from the database.", "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "eventsListeners": { "title": "Event listeners", "description": "Configure what listeners receive events for the realm.", "type": "array", "items": { "type": "string" } }, "failureFactor": { "title": "Max login failures", "description": "Max login failures", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "federatedUsers": { "type": "array", "items": { "$ref": "#/$defs/UserRepresentation" } }, "firstBrokerLoginFlow": { "type": "string" }, "groups": { "type": "array", "items": { "$ref": "#/$defs/GroupRepresentation" } }, "id": { "type": "string" }, "identityProviderMappers": { "type": "array", "items": { "$ref": "#/$defs/IdentityProviderMapperRepresentation" } }, "identityProviders": { "type": "array", "items": { "$ref": "#/$defs/IdentityProviderRepresentation" } }, "internationalizationEnabled": { "title": "Internationalization", "description": "If enabled, you can choose which locales you support for this realm and which locale is the default.", "type": "boolean" }, "keycloakVersion": { "type": "string" }, "localizationTexts": { "type": "object", "additionalProperties": { "type": "object", "additionalProperties": { "type": "string" } } }, "loginTheme": { "title": "Login theme", "description": "Select theme for login, OTP, grant, registration and forgot password pages.", "type": "string" }, "loginWithEmailAllowed": { "title": "Login with email", "description": "Allow users to log in with their email address.", "type": "boolean" }, "maxDeltaTimeSeconds": { "title": "Failure reset time", "description": "When will failure count be reset?", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "maxFailureWaitSeconds": { "title": "Max wait", "description": "Max time a user will be locked out.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "maxTemporaryLockouts": { "title": "Maximum temporary lockouts", "description": "The number of temporary lockouts permitted before the user is permanently locked out.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "minimumQuickLoginWaitSeconds": { "title": "Minimum quick login wait", "description": "How long to wait after a quick login failure.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "notBefore": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "oAuth2DeviceCodeLifespan": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "oAuth2DevicePollingInterval": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "oauth2DeviceCodeLifespan": { "title": "OAuth 2.0 Device Code Lifespan", "description": "Max time before the device code and user code are expired. This value needs to be a long enough lifetime to be usable (allowing the user to retrieve their secondary device, navigate to the verification URI, login, etc.), but should be sufficiently short to limit the usability of a code obtained for phishing.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "oauth2DevicePollingInterval": { "title": "OAuth 2.0 Device Polling Interval", "description": "The minimum amount of time in seconds that the client should wait between polling requests to the token endpoint.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "oauthClients": { "type": "array", "items": { "$ref": "#/$defs/OAuthClientRepresentation" } }, "offlineSessionIdleTimeout": { "title": "Offline Session Idle", "description": "Time an offline session is allowed to be idle before it expires. You need to use offline token to refresh at least once within this period; otherwise offline session will expire.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "offlineSessionMaxLifespan": { "title": "Offline Session Max", "description": "Max time before an offline session is expired regardless of activity.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "offlineSessionMaxLifespanEnabled": { "title": "Offline Session Max Limited", "description": "Enable offline session maximum lifetime", "type": "boolean" }, "organizations": { "type": "array", "items": { "$ref": "#/$defs/OrganizationRepresentation" } }, "organizationsEnabled": { "type": "boolean" }, "otpPolicyAlgorithm": { "title": "OTP hash algorithm", "description": "What hashing algorithm should be used to generate the OTP.", "type": "string" }, "otpPolicyCodeReusable": { "title": "Reusable token", "description": "Possibility to use the same OTP code again after successful authentication.", "type": "boolean" }, "otpPolicyDigits": { "title": "Number of digits", "description": "How many digits should the OTP have?", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "otpPolicyInitialCounter": { "title": "Initial counter", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "otpPolicyLookAheadWindow": { "title": "Look around window", "description": "How far around (extra token periods or counts) should the server look just in case the token generator and server are out of time sync or counter sync?", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "otpPolicyPeriod": { "title": "OTP Token period", "description": "How many seconds should an OTP token be valid? Defaults to 30 seconds.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "otpPolicyType": { "title": "OTP type", "description": "totp is Time-Based One Time Password. 'hotp' is a counter base one time password in which the server keeps a counter to hash against.", "type": "string", "enum": [ "totp", "hotp" ] }, "otpSupportedApplications": { "type": "array", "items": { "type": "string" } }, "passwordCredentialGrantAllowed": { "type": "boolean" }, "passwordPolicy": { "type": "string" }, "permanentLockout": { "title": "Permanent lockout", "type": "boolean" }, "privateKey": { "type": "string" }, "protocolMappers": { "type": "array", "items": { "$ref": "#/$defs/ProtocolMapperRepresentation" } }, "publicKey": { "type": "string" }, "quickLoginCheckMilliSeconds": { "title": "Quick login check milliseconds", "description": "If a failure happens concurrently too quickly, lock out the user.", "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "realm": { "title": "Realm ID", "type": "string" }, "realmCacheEnabled": { "type": "boolean" }, "refreshTokenMaxReuse": { "title": "Refresh Token Max Reuse", "description": "Maximum number of times a refresh token can be reused. When a different token is used, revocation is immediate.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "registrationAllowed": { "title": "User registration", "description": "Enable/disable the registration page. A link for registration will show on login page too.", "type": "boolean" }, "registrationEmailAsUsername": { "title": "Email as username", "description": "Allow users to set email as username.", "type": "boolean" }, "registrationFlow": { "type": "string" }, "rememberMe": { "title": "Remember me", "description": "Show checkbox on login page to allow user to remain logged in between browser restarts until session expires.", "type": "boolean" }, "requiredActions": { "type": "array", "items": { "$ref": "#/$defs/RequiredActionProviderRepresentation" } }, "requiredCredentials": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resetCredentialsFlow": { "type": "string" }, "resetPasswordAllowed": { "title": "Specifies independent timeout for forgot password.", "description": "Show a link on login page for user to click when they have forgotten their credentials.", "type": "boolean" }, "revokeRefreshToken": { "title": "Revoke Refresh Token", "description": "If enabled a refresh token can only be used up to 'Refresh Token Max Reuse' and is revoked when a different token is used. Otherwise refresh tokens are not revoked when used and can be used multiple times.", "type": "boolean" }, "roles": { "$ref": "#/$defs/RolesRepresentation" }, "scopeMappings": { "type": "array", "items": { "$ref": "#/$defs/ScopeMappingRepresentation" } }, "smtpServer": { "type": "object", "properties": { "auth": { "title": "Authentication", "type": "string", "enum": [ "true", "false", "" ] }, "envelopeFrom": { "title": "Envelope from", "description": "An email address used for bounces (optional).", "type": "string" }, "from": { "title": "From", "type": "string" }, "fromDisplayName": { "title": "From display name", "description": "A user-friendly name for the 'From' address (optional).", "type": "string" }, "host": { "title": "Host", "type": "string" }, "password": { "title": "Password", "description": "SMTP password. This field is able to obtain its value from vault, use ${vault.ID} format.", "type": "string" }, "port": { "title": "Port", "type": "string" }, "replyTo": { "title": "Reply to", "type": "string" }, "replyToDisplayName": { "title": "Reply to display name", "description": "A user-friendly name for the 'Reply-To' address (optional).", "type": "string" }, "ssl": { "title": "Enable SSL", "type": "string", "enum": [ "true", "false", "" ] }, "starttls": { "title": "Enable StartTLS", "type": "string", "enum": [ "true", "false", "" ] }, "user": { "title": "Username", "type": "string" } }, "additionalProperties": false }, "social": { "type": "boolean" }, "socialProviders": { "type": "object", "additionalProperties": { "type": "string" } }, "sslRequired": { "title": "Require SSL", "description": "Is HTTPS required? 'None' means HTTPS is not required for any client IP address. 'External requests' means localhost and private IP addresses can access without HTTPS. 'All requests' means HTTPS is required for all IP addresses.", "type": "string", "enum": [ "all", "external", "none" ] }, "ssoSessionIdleTimeout": { "title": "SSO Session Idle", "description": "Time a session is allowed to be idle before it expires. Tokens and browser sessions are invalidated when a session is expired.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "ssoSessionIdleTimeoutRememberMe": { "title": "SSO Session Idle Remember Me", "description": "Time a remember me session is allowed to be idle before it expires. Tokens and browser sessions are invalidated when a session is expired. If not set it uses the standard SSO Session Idle value.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "ssoSessionMaxLifespan": { "title": "SSO Session Max", "description": "Max time before a session is expired. Tokens and browser sessions are invalidated when a session is expired.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "ssoSessionMaxLifespanRememberMe": { "title": "SSO Session Max Remember Me", "description": "Max time before a session is expired when a user has set the remember me option. Tokens and browser sessions are invalidated when a session is expired. If not set it uses the standard SSO Session Max value.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "supportedLocales": { "title": "Supported locales", "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "updateProfileOnInitialSocialLogin": { "type": "boolean" }, "userCacheEnabled": { "type": "boolean" }, "userFederationMappers": { "type": "array", "items": { "$ref": "#/$defs/UserFederationMapperRepresentation" } }, "userFederationProviders": { "type": "array", "items": { "$ref": "#/$defs/UserFederationProviderRepresentation" } }, "userManagedAccessAllowed": { "title": "User-managed access", "description": "If enabled, users are allowed to manage their resources and permissions using the Account Management UI.", "type": "boolean" }, "users": { "type": "array", "items": { "$ref": "#/$defs/UserRepresentation" } }, "verifiableCredentialsEnabled": { "type": "boolean" }, "verifyEmail": { "title": "Verify email", "description": "Require user to verify their email address after initial login or after address changes are submitted.", "type": "boolean" }, "waitIncrementSeconds": { "title": "Wait increment", "description": "When failure threshold has been met, how much time should the user be locked out?", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "webAuthnPolicyAcceptableAaguids": { "title": "Acceptable AAGUIDs", "description": "The list of allowed AAGUIDs of which an authenticator can be registered. An AAGUID is a 128-bit identifier indicating the authenticator's type (e.g., make and model).", "type": "array", "items": { "type": "string" } }, "webAuthnPolicyAttestationConveyancePreference": { "title": "Attestation conveyance preference", "description": "Communicates to an authenticator the preference of how to generate an attestation statement.", "type": "string", "enum": [ "not specified", "none", "indirect", "direct" ] }, "webAuthnPolicyAuthenticatorAttachment": { "title": "Authenticator Attachment", "description": "Communicates to an authenticator an acceptable attachment pattern.", "type": "string", "enum": [ "not specified", "platform", "cross-platform" ] }, "webAuthnPolicyAvoidSameAuthenticatorRegister": { "title": "Avoid same authenticator registration", "description": "Avoid registering an authenticator that has already been registered.", "type": "boolean" }, "webAuthnPolicyCreateTimeout": { "title": "Timeout", "description": "The timeout value for creating the user's public key credential in seconds. If set to 0, this timeout option is not adapted.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "webAuthnPolicyExtraOrigins": { "type": "array", "items": { "type": "string" } }, "webAuthnPolicyPasswordlessAcceptableAaguids": { "title": "Acceptable AAGUIDs", "description": "The list of allowed AAGUIDs of which an authenticator can be registered. An AAGUID is a 128-bit identifier indicating the authenticator's type (e.g., make and model).", "type": "array", "items": { "type": "string" } }, "webAuthnPolicyPasswordlessAttestationConveyancePreference": { "title": "Attestation conveyance preference", "description": "Communicates to an authenticator the preference of how to generate an attestation statement.", "type": "string", "enum": [ "not specified", "none", "indirect", "direct" ] }, "webAuthnPolicyPasswordlessAuthenticatorAttachment": { "title": "Authenticator Attachment", "description": "Communicates to an authenticator an acceptable attachment pattern.", "type": "string", "enum": [ "not specified", "platform", "cross-platform" ] }, "webAuthnPolicyPasswordlessAvoidSameAuthenticatorRegister": { "title": "Avoid same authenticator registration", "description": "Avoid registering an authenticator that has already been registered.", "type": "boolean" }, "webAuthnPolicyPasswordlessCreateTimeout": { "title": "Timeout", "description": "The timeout value for creating the user's public key credential in seconds. If set to 0, this timeout option is not adapted.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "webAuthnPolicyPasswordlessExtraOrigins": { "type": "array", "items": { "type": "string" } }, "webAuthnPolicyPasswordlessRequireResidentKey": { "title": "Require discoverable credential", "description": "It tells an authenticator whether to create a public key credential as a Discoverable Credential.", "type": "string", "enum": [ "not specified", "Yes", "No" ] }, "webAuthnPolicyPasswordlessRpEntityName": { "title": "Relying party entity name", "description": "Human-readable server name as WebAuthn Relying Party", "type": "string" }, "webAuthnPolicyPasswordlessRpId": { "title": "Relying party ID", "description": "The WebAuthn Relying Party ID (RpID). It must be the origin's effective domain, e.g. 'company.com' or 'auth.company.com'.", "type": "string" }, "webAuthnPolicyPasswordlessSignatureAlgorithms": { "title": "Signature algorithms", "description": "The signature algorithms that should be used for the Authentication Assertion.", "type": "array", "items": { "type": "string", "enum": [ "Ed25519", "ES256", "ES384", "ES512", "RS256", "RS384", "RS512", "RS1" ] } }, "webAuthnPolicyPasswordlessUserVerificationRequirement": { "title": "User verification requirement", "description": "Communicates to an authenticator whether to require to verify a user.", "type": "string", "enum": [ "not specified", "required", "preferred", "discouraged" ] }, "webAuthnPolicyRequireResidentKey": { "title": "Require discoverable credential", "description": "It tells an authenticator whether to create a public key credential as a Discoverable Credential.", "type": "string", "enum": [ "not specified", "Yes", "No" ] }, "webAuthnPolicyRpEntityName": { "title": "Relying party entity name", "description": "Human-readable server name as WebAuthn Relying Party", "type": "string" }, "webAuthnPolicyRpId": { "title": "Relying party ID", "description": "The WebAuthn Relying Party ID (RpID). It must be the origin's effective domain, e.g. 'company.com' or 'auth.company.com'.", "type": "string" }, "webAuthnPolicySignatureAlgorithms": { "title": "Signature algorithms", "description": "The signature algorithms that should be used for the Authentication Assertion.", "type": "array", "items": { "type": "string", "enum": [ "Ed25519", "ES256", "ES384", "ES512", "RS256", "RS384", "RS512", "RS1" ] } }, "webAuthnPolicyUserVerificationRequirement": { "title": "User verification requirement", "description": "Communicates to an authenticator whether to require to verify a user.", "type": "string", "enum": [ "not specified", "required", "preferred", "discouraged" ] } }, "additionalProperties": false } ``` </details>

---

### spec.definition.accessCodeLifespan

Type: integer

Max time a client has to finish the access token protocol. This should normally be 1 minute.

---

### spec.definition.accessCodeLifespanLogin

Type: integer

Max time a user has to complete a login. This is recommended to be relatively long, such as 30 minutes or more.

---

### spec.definition.accessCodeLifespanUserAction

Type: integer

Max time a user has to complete login related actions like update password or configure totp. This is recommended to be relatively long, such as 5 minutes or more.

---

### spec.definition.accessTokenLifespan

Type: integer

Max time before an access token is expired. This value is recommended to be short relative to the SSO timeout.

---

### spec.definition.accessTokenLifespanForImplicitFlow

Type: integer

Max time before an access token issued during OpenID Connect Implicit Flow is expired. This value is recommended to be shorter than the SSO timeout. There is no possibility to refresh token during implicit flow, that's why there is a separate timeout different to 'Access Token Lifespan'.

---

### spec.definition.accountTheme

Type: string

Select theme for login, OTP, grant, registration and forgot password pages.

---

### spec.definition.actionTokenGeneratedByAdminLifespan

Type: integer

Maximum time before an action permit sent to a user by administrator is expired. This value is recommended to be long to allow administrators to send e-mails for users that are currently offline. The default timeout can be overridden immediately before issuing the token.

---

### spec.definition.actionTokenGeneratedByUserLifespan

Type: integer

Maximum time before an action permit sent by a user (such as a forgot password e-mail) is expired. This value is recommended to be short because it's expected that the user would react to self-created action quickly.

---

### spec.definition.adminEventsDetailsEnabled

Type: boolean

Include JSON representation for create and update requests.

---

### spec.definition.adminEventsEnabled

Type: boolean

If enabled, admin events are saved to the database, which makes events available to the Admin UI.

---

### spec.definition.adminPermissionsEnabled

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

|Property|Type|Required|
|:-------|:---|:------:|
|[adminEventsExpiration](#specdefinitionattributesadmineventsexpiration)|string||
|[cibaAuthRequestedUserHint](#specdefinitionattributescibaauthrequesteduserhint)|string||
|[cibaBackchannelTokenDeliveryMode](#specdefinitionattributescibabackchanneltokendeliverymode)|string||
|[cibaExpiresIn](#specdefinitionattributescibaexpiresin)|string||
|[cibaInterval](#specdefinitionattributescibainterval)|string||
|[frontendUrl](#specdefinitionattributesfrontendurl)|string||

RealmRepresentationAttributes

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "adminEventsExpiration": { "title": "Expiration", "description": "Sets the expiration for events. Expired events are periodically deleted from the database.", "type": "string", "pattern": "^[0-9]*$" }, "cibaAuthRequestedUserHint": { "title": "Authentication Requested User Hint", "description": "The way of identifying the end-user for whom authentication is being requested. Currently only \"login_hint\" is supported.", "type": "string", "enum": [ "login_hint" ] }, "cibaBackchannelTokenDeliveryMode": { "title": "Backchannel Token Delivery Mode", "description": "Specifies how the CD (Consumption Device) gets the authentication result and related tokens. This mode will be used by default for the CIBA clients, which do not have other mode explicitly set.", "type": "string", "enum": [ "ping", "poll" ] }, "cibaExpiresIn": { "title": "Expires In", "description": "The expiration time of the \"auth_req_id\" in seconds since the authentication request was received.", "type": "string", "pattern": "^[0-9]*$" }, "cibaInterval": { "title": "Interval", "description": "The minimum amount of time in seconds that the CD (Consumption Device) must wait between polling requests to the token endpoint. If set to 0, the CD must use 5 as the default value according to the CIBA specification.", "type": "string", "pattern": "^[0-9]*$" }, "frontendUrl": { "title": "Frontend URL", "description": "Set the frontend URL for the realm. Use in combination with the default hostname provider to override the base URL for frontend requests for a specific realm.", "type": "string" } }, "additionalProperties": { "type": "string" } } ``` </details>

---

### spec.definition.attributes.adminEventsExpiration

Type: string

Sets the expiration for events. Expired events are periodically deleted from the database.

---

### spec.definition.attributes.cibaAuthRequestedUserHint

Type: string

The way of identifying the end-user for whom authentication is being requested. Currently only "login_hint" is supported.

---

### spec.definition.attributes.cibaBackchannelTokenDeliveryMode

Type: string

Specifies how the CD (Consumption Device) gets the authentication result and related tokens. This mode will be used by default for the CIBA clients, which do not have other mode explicitly set.

---

### spec.definition.attributes.cibaExpiresIn

Type: string

The expiration time of the "auth_req_id" in seconds since the authentication request was received.

---

### spec.definition.attributes.cibaInterval

Type: string

The minimum amount of time in seconds that the CD (Consumption Device) must wait between polling requests to the token endpoint. If set to 0, the CD must use 5 as the default value according to the CIBA specification.

---

### spec.definition.attributes.frontendUrl

Type: string

Set the frontend URL for the realm. Use in combination with the default hostname provider to override the base URL for frontend requests for a specific realm.

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

AuthenticationFlowRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "alias": { "type": "string" }, "authenticationExecutions": { "type": "array", "items": { "$ref": "#/$defs/AuthenticationExecutionExportRepresentation" } }, "builtIn": { "type": "boolean" }, "description": { "type": "string" }, "id": { "type": "string" }, "providerId": { "type": "string" }, "topLevel": { "type": "boolean" } }, "additionalProperties": false } ``` </details>

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

AuthenticationExecutionExportRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "authenticator": { "type": "string" }, "authenticatorConfig": { "type": "string" }, "authenticatorFlow": { "type": "boolean" }, "autheticatorFlow": { "type": "boolean" }, "flowAlias": { "type": "string" }, "priority": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "requirement": { "type": "string", "enum": [ "ALTERNATIVE", "CONDITIONAL", "DISABLED", "REQUIRED" ] }, "userSetupAllowed": { "type": "boolean" } }, "additionalProperties": false } ``` </details>

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

AuthenticationExecutionExportRepresentationRequirement

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "ALTERNATIVE", "CONDITIONAL", "DISABLED", "REQUIRED" ] } ``` </details>

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

AuthenticatorConfigRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "alias": { "type": "string" }, "config": { "type": "object", "additionalProperties": { "type": "string" } }, "id": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

|Property|Type|Required|
|:-------|:---|:------:|
|[contentSecurityPolicy](#specdefinitionbrowsersecurityheaderscontentsecuritypolicy)|string||
|[contentSecurityPolicyReportOnly](#specdefinitionbrowsersecurityheaderscontentsecuritypolicyreportonly)|string||
|[strictTransportSecurity](#specdefinitionbrowsersecurityheadersstricttransportsecurity)|string||
|[xContentTypeOptions](#specdefinitionbrowsersecurityheadersxcontenttypeoptions)|string||
|[xFrameOptions](#specdefinitionbrowsersecurityheadersxframeoptions)|string||
|[xRobotsTag](#specdefinitionbrowsersecurityheadersxrobotstag)|string||
|[xXSSProtection](#specdefinitionbrowsersecurityheadersxxssprotection)|string||

RealmRepresentationBrowserSecurityHeaders

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "contentSecurityPolicy": { "title": "Content-Security-Policy", "description": "Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>", "type": "string" }, "contentSecurityPolicyReportOnly": { "title": "Content-Security-Policy-Report-Only", "description": "For testing Content Security Policies <1>Learn more</1>", "type": "string" }, "strictTransportSecurity": { "title": "HTTP Strict Transport Security (HSTS)", "description": "The Strict-Transport-Security HTTP header tells browsers to always use HTTPS. Once a browser sees this header, it will only visit the site over HTTPS for the time specified (1 year) at max-age, including the subdomains. <1>Learn more</1>", "type": "string" }, "xContentTypeOptions": { "title": "X-Content-Type-Options", "description": "The default value prevents Internet Explorer and Google Chrome from MIME-sniffing a response away from the declared content-type. <1>Learn more</1>", "type": "string" }, "xFrameOptions": { "title": "X-Frame-Options", "description": "Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>", "type": "string" }, "xRobotsTag": { "title": "X-Robots-Tag", "description": "Prevent pages from appearing in search engines. <1>Learn more</1>", "type": "string" }, "xXSSProtection": { "title": "X-XSS-Protection", "description": "This header configures the Cross-site scripting (XSS) filter in your browser. Using the default behaviour, the browser will prevent rendering of the page when a XSS attack is detected. <1>Learn more</1>", "type": "string" } }, "additionalProperties": false } ``` </details>

---

### spec.definition.browserSecurityHeaders.contentSecurityPolicy

Type: string

Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>

---

### spec.definition.browserSecurityHeaders.contentSecurityPolicyReportOnly

Type: string

For testing Content Security Policies <1>Learn more</1>

---

### spec.definition.browserSecurityHeaders.strictTransportSecurity

Type: string

The Strict-Transport-Security HTTP header tells browsers to always use HTTPS. Once a browser sees this header, it will only visit the site over HTTPS for the time specified (1 year) at max-age, including the subdomains. <1>Learn more</1>

---

### spec.definition.browserSecurityHeaders.xContentTypeOptions

Type: string

The default value prevents Internet Explorer and Google Chrome from MIME-sniffing a response away from the declared content-type. <1>Learn more</1>

---

### spec.definition.browserSecurityHeaders.xFrameOptions

Type: string

Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>

---

### spec.definition.browserSecurityHeaders.xRobotsTag

Type: string

Prevent pages from appearing in search engines. <1>Learn more</1>

---

### spec.definition.browserSecurityHeaders.xXSSProtection

Type: string

This header configures the Cross-site scripting (XSS) filter in your browser. Using the default behaviour, the browser will prevent rendering of the page when a XSS attack is detected. <1>Learn more</1>

---

### spec.definition.bruteForceDetection

Type: object

*missing*

---

### spec.definition.bruteForceProtected

Type: boolean

*missing*

---

### spec.definition.bruteForceStrategy

Type: string

Multiple means wait time will be increased only when number of failures are multiples of '{{failureFactor}}'. Linear means each new failure starting at '{{failureFactor}}' will increase wait time.

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

ClientPoliciesRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "globalPolicies": { "type": "array", "items": { "$ref": "#/$defs/ClientPolicyRepresentation" } }, "policies": { "type": "array", "items": { "$ref": "#/$defs/ClientPolicyRepresentation" } } }, "additionalProperties": false } ``` </details>

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

ClientPolicyRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "conditions": { "type": "array", "items": { "$ref": "#/$defs/ClientPolicyConditionRepresentation" } }, "description": { "type": "string" }, "enabled": { "type": "boolean" }, "name": { "type": "string" }, "profiles": { "type": "array", "items": { "type": "string" } } }, "additionalProperties": false } ``` </details>

---

### spec.definition.clientPolicies.globalPolicies[].conditions[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[condition](#specdefinitionclientpoliciesglobalpoliciesconditionscondition)|string||
|[configuration](#specdefinitionclientpoliciesglobalpoliciesconditionsconfiguration)|object||

ClientPolicyConditionRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "condition": { "type": "string" }, "configuration": { "type": "object" } }, "additionalProperties": false } ``` </details>

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

ClientPolicyRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "conditions": { "type": "array", "items": { "$ref": "#/$defs/ClientPolicyConditionRepresentation" } }, "description": { "type": "string" }, "enabled": { "type": "boolean" }, "name": { "type": "string" }, "profiles": { "type": "array", "items": { "type": "string" } } }, "additionalProperties": false } ``` </details>

---

### spec.definition.clientPolicies.policies[].conditions[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[condition](#specdefinitionclientpoliciespoliciesconditionscondition)|string||
|[configuration](#specdefinitionclientpoliciespoliciesconditionsconfiguration)|object||

ClientPolicyConditionRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "condition": { "type": "string" }, "configuration": { "type": "object" } }, "additionalProperties": false } ``` </details>

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

ClientProfilesRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "globalProfiles": { "type": "array", "items": { "$ref": "#/$defs/ClientProfileRepresentation" } }, "profiles": { "type": "array", "items": { "$ref": "#/$defs/ClientProfileRepresentation" } } }, "additionalProperties": false } ``` </details>

---

### spec.definition.clientProfiles.globalProfiles[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[description](#specdefinitionclientprofilesglobalprofilesdescription)|string||
|[executors[]](#specdefinitionclientprofilesglobalprofilesexecutors)|object||
|[name](#specdefinitionclientprofilesglobalprofilesname)|string||

ClientProfileRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "description": { "type": "string" }, "executors": { "type": "array", "items": { "$ref": "#/$defs/ClientPolicyExecutorRepresentation" } }, "name": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

ClientPolicyExecutorRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "configuration": { "type": "object" }, "executor": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

ClientProfileRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "description": { "type": "string" }, "executors": { "type": "array", "items": { "$ref": "#/$defs/ClientPolicyExecutorRepresentation" } }, "name": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

ClientPolicyExecutorRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "configuration": { "type": "object" }, "executor": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

ClientScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "attributes": { "type": "object", "additionalProperties": { "type": "string" } }, "description": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "protocol": { "type": "string", "enum": [ "openid-connect", "saml" ] }, "protocolMappers": { "type": "array", "items": { "$ref": "#/$defs/ProtocolMapperRepresentation" } } }, "additionalProperties": false } ``` </details>

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

ClientScopeRepresentationProtocol

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "openid-connect", "saml" ] } ``` </details>

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

ProtocolMapperRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "consentRequired": { "type": "boolean" }, "consentText": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "protocol": { "type": "string", "enum": [ "openid-connect", "saml" ] }, "protocolMapper": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

ProtocolMapperRepresentationProtocol

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "openid-connect", "saml" ] } ``` </details>

---

### spec.definition.clientScopes[].protocolMappers[].protocolMapper

Type: string

*missing*

---

### spec.definition.clientSessionIdleTimeout

Type: integer

Time a client session is allowed to be idle before it expires. Tokens are invalidated when a client session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Idle value.

---

### spec.definition.clientSessionMaxLifespan

Type: integer

Max time before a client session is expired. Tokens are invalidated when a session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Max value.

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

ClientTemplateRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "attributes": { "type": "object", "additionalProperties": { "type": "string" } }, "bearerOnly": { "type": "boolean" }, "consentRequired": { "type": "boolean" }, "description": { "type": "string" }, "directAccessGrantsEnabled": { "type": "boolean" }, "frontchannelLogout": { "type": "boolean" }, "fullScopeAllowed": { "type": "boolean" }, "id": { "type": "string" }, "implicitFlowEnabled": { "type": "boolean" }, "name": { "type": "string" }, "protocol": { "type": "string" }, "protocolMappers": { "type": "array", "items": { "$ref": "#/$defs/ProtocolMapperRepresentation" } }, "publicClient": { "type": "boolean" }, "serviceAccountsEnabled": { "type": "boolean" }, "standardFlowEnabled": { "type": "boolean" } }, "additionalProperties": false } ``` </details>

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

ProtocolMapperRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "consentRequired": { "type": "boolean" }, "consentText": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "protocol": { "type": "string", "enum": [ "openid-connect", "saml" ] }, "protocolMapper": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

ProtocolMapperRepresentationProtocol

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "openid-connect", "saml" ] } ``` </details>

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

RoleRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "clientRole": { "type": "boolean" }, "composite": { "type": "boolean" }, "composites": { "$ref": "#/$defs/Composites" }, "containerId": { "type": "string" }, "description": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "scopeParamRequired": { "type": "boolean" } }, "additionalProperties": false } ``` </details>

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

Composites

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "application": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "client": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "realm": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } }, "additionalProperties": false } ``` </details>

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

Default algorithm used to sign tokens for the realm

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

Allow multiple users to have the same email address. Changing this setting will also clear the user's cache. It is recommended to manually update email constraints of existing users in the database after switching off support for duplicate email addresses.

---

### spec.definition.editUsernameAllowed

Type: boolean

If enabled, the username field is editable, readonly otherwise.

---

### spec.definition.emailTheme

Type: string

Select a theme for emails that are sent by the server.

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

If enabled, user events are saved to the database, which makes events available to the admin and account management UIs.

---

### spec.definition.eventsExpiration

Type: integer

Sets the expiration for events. Expired events are periodically deleted from the database.

---

### spec.definition.eventsListeners[]

Type: string

*missing*

---

### spec.definition.failureFactor

Type: integer

Max login failures

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

UserRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "access": { "type": "object", "additionalProperties": { "type": "boolean" } }, "applicationRoles": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "clientConsents": { "type": "array", "items": { "$ref": "#/$defs/UserConsentRepresentation" } }, "clientRoles": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "createdTimestamp": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "credentials": { "type": "array", "items": { "$ref": "#/$defs/CredentialRepresentation" } }, "disableableCredentialTypes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "email": { "type": "string" }, "emailVerified": { "type": "boolean" }, "enabled": { "type": "boolean" }, "federatedIdentities": { "type": "array", "items": { "$ref": "#/$defs/FederatedIdentityRepresentation" } }, "federationLink": { "type": "string" }, "firstName": { "type": "string" }, "groups": { "type": "array", "items": { "type": "string" } }, "id": { "type": "string" }, "lastName": { "type": "string" }, "notBefore": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "origin": { "type": "string" }, "realmRoles": { "type": "array", "items": { "type": "string" } }, "requiredActions": { "type": "array", "items": { "type": "string" } }, "self": { "type": "string" }, "serviceAccountClientId": { "type": "string" }, "socialLinks": { "type": "array", "items": { "$ref": "#/$defs/SocialLinkRepresentation" } }, "totp": { "type": "boolean" }, "userProfileMetadata": { "$ref": "#/$defs/UserProfileMetadata" }, "username": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

UserConsentRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "clientId": { "type": "string" }, "createdDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "grantedClientScopes": { "type": "array", "items": { "type": "string" } }, "grantedRealmRoles": { "type": "array", "items": { "type": "string" } }, "lastUpdatedDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 } }, "additionalProperties": false } ``` </details>

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

CredentialRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "algorithm": { "type": "string" }, "config": { "type": "object", "allOf": [ { "$ref": "#/$defs/MultivaluedHashMapStringString" } ] }, "counter": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "createdDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "credentialData": { "type": "string" }, "device": { "type": "string" }, "digits": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "hashIterations": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "hashedSaltedValue": { "type": "string" }, "id": { "type": "string" }, "period": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "priority": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "salt": { "type": "string" }, "secretData": { "type": "string" }, "temporary": { "type": "boolean" }, "type": { "type": "string" }, "userLabel": { "type": "string" }, "value": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

FederatedIdentityRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "identityProvider": { "type": "string" }, "userId": { "type": "string" }, "userName": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

SocialLinkRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "socialProvider": { "type": "string" }, "socialUserId": { "type": "string" }, "socialUsername": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

UserProfileMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "attributes": { "type": "array", "items": { "$ref": "#/$defs/UserProfileAttributeMetadata" } }, "groups": { "type": "array", "items": { "$ref": "#/$defs/UserProfileAttributeGroupMetadata" } } }, "additionalProperties": false } ``` </details>

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

UserProfileAttributeMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "annotations": { "type": "object", "additionalProperties": {} }, "displayName": { "type": "string" }, "group": { "type": "string" }, "multivalued": { "type": "boolean" }, "name": { "type": "string" }, "readOnly": { "type": "boolean" }, "required": { "type": "boolean" }, "validators": { "type": "object", "additionalProperties": { "type": "object", "additionalProperties": {} } } }, "additionalProperties": false } ``` </details>

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

UserProfileAttributeGroupMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "annotations": { "type": "object", "additionalProperties": {} }, "displayDescription": { "type": "string" }, "displayHeader": { "type": "string" }, "name": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

IdentityProviderMapperRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "properties": { "attribute.friendly.name": { "title": "Friendly name", "description": "Friendly name of attribute to search for in assertion. You can leave this blank and specify a name instead.", "type": "string" }, "attribute.name.format": { "type": "string", "enum": [ "ATTRIBUTE_FORMAT_BASIC", "ATTRIBUTE_FORMAT_URI", "ATTRIBUTE_FORMAT_UNSPECIFIED" ] }, "syncMode": { "title": "Sync mode override", "description": "Overrides the default sync mode of the IDP for this mapper. Values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider and 'inherit' to use the sync mode defined in the identity provider for this mapper.", "type": "string", "enum": [ "INHERIT", "IMPORT", "LEGACY", "FORCE" ] }, "user.attribute": { "title": "User Attribute Name", "description": "Name of user attribute you want to hardcode", "type": "string" } }, "additionalProperties": { "type": "string" } }, "id": { "type": "string" }, "identityProviderAlias": { "type": "string" }, "identityProviderMapper": { "type": "string" }, "name": { "title": "Name", "description": "Name of the mapper.", "type": "string" } }, "additionalProperties": false } ``` </details>

---

### spec.definition.identityProviderMappers[].config

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[attribute.friendly.name](#specdefinitionidentityprovidermappersconfigattributefriendlyname)|string||
|[attribute.name.format](#specdefinitionidentityprovidermappersconfigattributenameformat)|string||
|[syncMode](#specdefinitionidentityprovidermappersconfigsyncmode)|string||
|[user.attribute](#specdefinitionidentityprovidermappersconfiguserattribute)|string||

IdentityProviderMapperRepresentationConfig

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "attribute.friendly.name": { "title": "Friendly name", "description": "Friendly name of attribute to search for in assertion. You can leave this blank and specify a name instead.", "type": "string" }, "attribute.name.format": { "type": "string", "enum": [ "ATTRIBUTE_FORMAT_BASIC", "ATTRIBUTE_FORMAT_URI", "ATTRIBUTE_FORMAT_UNSPECIFIED" ] }, "syncMode": { "title": "Sync mode override", "description": "Overrides the default sync mode of the IDP for this mapper. Values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider and 'inherit' to use the sync mode defined in the identity provider for this mapper.", "type": "string", "enum": [ "INHERIT", "IMPORT", "LEGACY", "FORCE" ] }, "user.attribute": { "title": "User Attribute Name", "description": "Name of user attribute you want to hardcode", "type": "string" } }, "additionalProperties": { "type": "string" } } ``` </details>

---

### spec.definition.identityProviderMappers[].config.attribute.friendly.name

Type: string

Friendly name of attribute to search for in assertion. You can leave this blank and specify a name instead.

---

### spec.definition.identityProviderMappers[].config.attribute.name.format

Type: string

IdentityProviderMapperRepresentationConfigAttributeNameFormat

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "ATTRIBUTE_FORMAT_BASIC", "ATTRIBUTE_FORMAT_URI", "ATTRIBUTE_FORMAT_UNSPECIFIED" ] } ``` </details>

---

### spec.definition.identityProviderMappers[].config.syncMode

Type: string

Overrides the default sync mode of the IDP for this mapper. Values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider and 'inherit' to use the sync mode defined in the identity provider for this mapper.

---

### spec.definition.identityProviderMappers[].config.user.attribute

Type: string

Name of user attribute you want to hardcode

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

Name of the mapper.

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

```json { "type": "object", "properties": { "addReadTokenRoleOnCreate": { "title": "Stored tokens readable", "description": "Enable/disable if new users can read any stored tokens. This assigns the broker.read-token role.", "type": "boolean" }, "alias": { "title": "Alias", "description": "The alias uniquely identifies an identity provider and it is also used to build the redirect uri.", "type": "string" }, "authenticateByDefault": { "type": "boolean" }, "config": { "type": "object", "properties": { "allowCreate": { "title": "Allow create", "description": "Allow the external identity provider to create a new identifier to represent the principal.", "type": "string", "enum": [ "true", "false", "" ] }, "allowedClockSkew": { "title": "Allowed clock skew", "description": "Clock skew in seconds that is tolerated when validating identity provider tokens. Default value is zero.", "type": "string", "pattern": "^[0-9]*$" }, "attributeConsumingServiceIndex": { "title": "Attribute Consuming Service Index", "description": "Index of the Attribute Consuming Service profile to request during authentication.", "type": "string", "pattern": "^[0-9]*$" }, "authnContextClassRefs": { "title": "AuthnContext ClassRefs", "description": "Ordered list of requested AuthnContext ClassRefs.", "type": "string" }, "authnContextComparisonType": { "title": "Comparison", "description": "Specifies the comparison method used to evaluate the requested context classes or statements. The default is \"Exact\".", "type": "string", "enum": [ "exact", "minimum", "maximum", "better" ] }, "authnContextDeclRefs": { "title": "AuthnContext DeclRefs", "description": "Ordered list of requested AuthnContext DeclRefs.", "type": "string" }, "backchannelSupported": { "title": "Backchannel logout", "description": "Does the external IDP support backchannel logout?", "type": "string", "enum": [ "true", "false", "" ] }, "encryptionAlgorithm": { "title": "Encryption Algorithm", "description": "Encryption algorithm, which is used by SAML IDP for encryption of SAML documents, assertions or IDs. The corresponding decryption key for decrypt SAML document parts will be chosen based on this configured algorithm and should be available in realm keys for the encryption (ENC) usage. If algorithm is not configured, then any supported algorithm is allowed and decryption key will be chosen based on the algorithm configured in SAML document itself.", "type": "string" }, "entityId": { "title": "Service provider entity ID", "description": "The Entity ID that will be used to uniquely identify this SAML Service Provider.", "type": "string" }, "forceAuthn": { "title": "Force authentication", "description": "Indicates whether the identity provider must authenticate the presenter directly rather than rely on a previous security context.", "type": "string", "enum": [ "true", "false", "" ] }, "hideOnLoginPage": { "title": "Hide on login page", "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.", "type": "string", "enum": [ "true", "false", "" ] }, "idpEntityId": { "title": "Identity provider entity ID", "description": "The Entity ID used to validate the Issuer for received SAML assertions. If empty, no Issuer validation is performed.", "type": "string" }, "loginHint": { "title": "Pass subject", "description": "During login phase, forward an optional login_hint query parameter to SAML AuthnRequest's Subject.", "type": "string", "enum": [ "true", "false", "" ] }, "nameIDPolicyFormat": { "title": "NameID policy format", "description": "Specifies the URI reference corresponding to a name identifier format.", "type": "string" }, "postBindingAuthnRequest": { "title": "HTTP-POST binding for AuthnRequest", "description": "Indicates whether the AuthnRequest must be sent using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "postBindingLogout": { "title": "HTTP-POST binding logout", "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "postBindingResponse": { "title": "HTTP-POST binding response", "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "principalAttribute": { "title": "Principal attribute", "description": "Name or Friendly Name of the attribute used to identify external users.", "type": "string" }, "principalType": { "title": "Principal type", "description": "Way to identify and track external users from the assertion. Default is using Subject NameID, alternatively you can set up identifying attribute.", "type": "string", "enum": [ "SUBJECT", "ATTRIBUTE", "FRIENDLY_ATTRIBUTE" ] }, "signSpMetadata": { "title": "Sign service provider metadata", "description": "Enable/disable signature of the provider SAML metadata.", "type": "string", "enum": [ "true", "false", "" ] }, "signatureAlgorithm": { "title": "Signature algorithm", "description": "The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.", "type": "string" }, "signingCertificate": { "title": "Validating X509 certificates", "description": "The public certificates Keycloak uses to validate the signatures of SAML requests and responses from the external IDP when Use metadata descriptor URL is OFF. Multiple certificates can be entered separated by comma (,). The certificates can be re-imported from the Metadata descriptor URL clicking the Import Keys action in the identity provider page. The action downloads the current certificates in the metadata endpoint and assigns them to the config in this same option. You need to click Save to definitely store the re-imported certificates.", "type": "string" }, "singleSignOnServiceUrl": { "title": "Single Sign-On service URL", "description": "The Url that must be used to send authentication requests (SAML AuthnRequest).", "type": "string" }, "syncMode": { "title": "Sync mode", "description": "Default sync mode for all mappers. The sync mode determines when user data will be synced using the mappers. Possible values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider.", "type": "string", "enum": [ "IMPORT", "LEGACY", "FORCE" ] }, "validateSignature": { "title": "Validate Signatures", "description": "Enable/disable signature validation of external IDP signatures.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAssertionsEncrypted": { "title": "Want Assertions encrypted", "description": "Indicates whether this service provider expects an encrypted Assertion.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAssertionsSigned": { "title": "Want Assertions signed", "description": "Indicates whether this service provider expects a signed Assertion.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAuthnRequestsSigned": { "title": "Want AuthnRequests signed", "description": "Indicates whether the identity provider expects a signed AuthnRequest.", "type": "string", "enum": [ "true", "false", "" ] }, "xmlSigKeyInfoKeyNameTransformer": { "title": "SAML signature key name", "description": "Signed SAML documents contain identification of signing key in KeyName element. For Keycloak / RH-SSO counter-party, use KEY_ID, for MS AD FS use CERT_SUBJECT, for others check and use NONE if no other option works.", "type": "string", "enum": [ "NONE", "KEY_ID", "CERT_SUBJECT" ] } }, "additionalProperties": { "type": "string" } }, "displayName": { "title": "Display name", "description": "Friendly name for Identity Providers.", "type": "string" }, "enabled": { "title": "Enabled", "type": "boolean" }, "firstBrokerLoginFlowAlias": { "title": "First login flow override", "description": "Alias of authentication flow, which is triggered after first login with this identity provider. Term 'First Login' means that no Keycloak account is currently linked to the authenticated identity provider account.", "type": "string" }, "hideOnLogin": { "title": "Hide on login page", "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.", "type": "boolean" }, "internalId": { "type": "string" }, "linkOnly": { "title": "Account linking only", "description": "If true, users cannot log in through this provider.  They can only link to this provider.  This is useful if you don't want to allow login from the provider, but want to integrate with a provider.", "type": "boolean" }, "organizationId": { "type": "string" }, "postBrokerLoginFlowAlias": { "title": "Post login flow", "description": "Alias of authentication flow, which is triggered after each login with this identity provider. Useful if you want additional verification of each user authenticated with this identity provider (for example OTP). Leave this to \"None\" if you need no any additional authenticators to be triggered after login with this identity provider. Also note that authenticator implementations must assume that user is already set in ClientSession as identity provider already set it.", "type": "string" }, "providerId": { "type": "string" }, "storeToken": { "title": "Store tokens", "description": "Enable/disable if tokens must be stored after authenticating users.", "type": "boolean" }, "trustEmail": { "title": "Trust Email", "description": "If enabled, email provided by this provider is not verified even if verification is enabled for the realm.", "type": "boolean" }, "updateProfileFirstLogin": { "type": "boolean" }, "updateProfileFirstLoginMode": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

|Property|Type|Required|
|:-------|:---|:------:|
|[allowCreate](#specdefinitionidentityprovidersconfigallowcreate)|string||
|[allowedClockSkew](#specdefinitionidentityprovidersconfigallowedclockskew)|string||
|[attributeConsumingServiceIndex](#specdefinitionidentityprovidersconfigattributeconsumingserviceindex)|string||
|[authnContextClassRefs](#specdefinitionidentityprovidersconfigauthncontextclassrefs)|string||
|[authnContextComparisonType](#specdefinitionidentityprovidersconfigauthncontextcomparisontype)|string||
|[authnContextDeclRefs](#specdefinitionidentityprovidersconfigauthncontextdeclrefs)|string||
|[backchannelSupported](#specdefinitionidentityprovidersconfigbackchannelsupported)|string||
|[encryptionAlgorithm](#specdefinitionidentityprovidersconfigencryptionalgorithm)|string||
|[entityId](#specdefinitionidentityprovidersconfigentityid)|string||
|[forceAuthn](#specdefinitionidentityprovidersconfigforceauthn)|string||
|[hideOnLoginPage](#specdefinitionidentityprovidersconfighideonloginpage)|string||
|[idpEntityId](#specdefinitionidentityprovidersconfigidpentityid)|string||
|[loginHint](#specdefinitionidentityprovidersconfigloginhint)|string||
|[nameIDPolicyFormat](#specdefinitionidentityprovidersconfignameidpolicyformat)|string||
|[postBindingAuthnRequest](#specdefinitionidentityprovidersconfigpostbindingauthnrequest)|string||
|[postBindingLogout](#specdefinitionidentityprovidersconfigpostbindinglogout)|string||
|[postBindingResponse](#specdefinitionidentityprovidersconfigpostbindingresponse)|string||
|[principalAttribute](#specdefinitionidentityprovidersconfigprincipalattribute)|string||
|[principalType](#specdefinitionidentityprovidersconfigprincipaltype)|string||
|[signSpMetadata](#specdefinitionidentityprovidersconfigsignspmetadata)|string||
|[signatureAlgorithm](#specdefinitionidentityprovidersconfigsignaturealgorithm)|string||
|[signingCertificate](#specdefinitionidentityprovidersconfigsigningcertificate)|string||
|[singleSignOnServiceUrl](#specdefinitionidentityprovidersconfigsinglesignonserviceurl)|string||
|[syncMode](#specdefinitionidentityprovidersconfigsyncmode)|string||
|[validateSignature](#specdefinitionidentityprovidersconfigvalidatesignature)|string||
|[wantAssertionsEncrypted](#specdefinitionidentityprovidersconfigwantassertionsencrypted)|string||
|[wantAssertionsSigned](#specdefinitionidentityprovidersconfigwantassertionssigned)|string||
|[wantAuthnRequestsSigned](#specdefinitionidentityprovidersconfigwantauthnrequestssigned)|string||
|[xmlSigKeyInfoKeyNameTransformer](#specdefinitionidentityprovidersconfigxmlsigkeyinfokeynametransformer)|string||

IdentityProviderRepresentationConfig

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "allowCreate": { "title": "Allow create", "description": "Allow the external identity provider to create a new identifier to represent the principal.", "type": "string", "enum": [ "true", "false", "" ] }, "allowedClockSkew": { "title": "Allowed clock skew", "description": "Clock skew in seconds that is tolerated when validating identity provider tokens. Default value is zero.", "type": "string", "pattern": "^[0-9]*$" }, "attributeConsumingServiceIndex": { "title": "Attribute Consuming Service Index", "description": "Index of the Attribute Consuming Service profile to request during authentication.", "type": "string", "pattern": "^[0-9]*$" }, "authnContextClassRefs": { "title": "AuthnContext ClassRefs", "description": "Ordered list of requested AuthnContext ClassRefs.", "type": "string" }, "authnContextComparisonType": { "title": "Comparison", "description": "Specifies the comparison method used to evaluate the requested context classes or statements. The default is \"Exact\".", "type": "string", "enum": [ "exact", "minimum", "maximum", "better" ] }, "authnContextDeclRefs": { "title": "AuthnContext DeclRefs", "description": "Ordered list of requested AuthnContext DeclRefs.", "type": "string" }, "backchannelSupported": { "title": "Backchannel logout", "description": "Does the external IDP support backchannel logout?", "type": "string", "enum": [ "true", "false", "" ] }, "encryptionAlgorithm": { "title": "Encryption Algorithm", "description": "Encryption algorithm, which is used by SAML IDP for encryption of SAML documents, assertions or IDs. The corresponding decryption key for decrypt SAML document parts will be chosen based on this configured algorithm and should be available in realm keys for the encryption (ENC) usage. If algorithm is not configured, then any supported algorithm is allowed and decryption key will be chosen based on the algorithm configured in SAML document itself.", "type": "string" }, "entityId": { "title": "Service provider entity ID", "description": "The Entity ID that will be used to uniquely identify this SAML Service Provider.", "type": "string" }, "forceAuthn": { "title": "Force authentication", "description": "Indicates whether the identity provider must authenticate the presenter directly rather than rely on a previous security context.", "type": "string", "enum": [ "true", "false", "" ] }, "hideOnLoginPage": { "title": "Hide on login page", "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.", "type": "string", "enum": [ "true", "false", "" ] }, "idpEntityId": { "title": "Identity provider entity ID", "description": "The Entity ID used to validate the Issuer for received SAML assertions. If empty, no Issuer validation is performed.", "type": "string" }, "loginHint": { "title": "Pass subject", "description": "During login phase, forward an optional login_hint query parameter to SAML AuthnRequest's Subject.", "type": "string", "enum": [ "true", "false", "" ] }, "nameIDPolicyFormat": { "title": "NameID policy format", "description": "Specifies the URI reference corresponding to a name identifier format.", "type": "string" }, "postBindingAuthnRequest": { "title": "HTTP-POST binding for AuthnRequest", "description": "Indicates whether the AuthnRequest must be sent using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "postBindingLogout": { "title": "HTTP-POST binding logout", "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "postBindingResponse": { "title": "HTTP-POST binding response", "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "principalAttribute": { "title": "Principal attribute", "description": "Name or Friendly Name of the attribute used to identify external users.", "type": "string" }, "principalType": { "title": "Principal type", "description": "Way to identify and track external users from the assertion. Default is using Subject NameID, alternatively you can set up identifying attribute.", "type": "string", "enum": [ "SUBJECT", "ATTRIBUTE", "FRIENDLY_ATTRIBUTE" ] }, "signSpMetadata": { "title": "Sign service provider metadata", "description": "Enable/disable signature of the provider SAML metadata.", "type": "string", "enum": [ "true", "false", "" ] }, "signatureAlgorithm": { "title": "Signature algorithm", "description": "The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.", "type": "string" }, "signingCertificate": { "title": "Validating X509 certificates", "description": "The public certificates Keycloak uses to validate the signatures of SAML requests and responses from the external IDP when Use metadata descriptor URL is OFF. Multiple certificates can be entered separated by comma (,). The certificates can be re-imported from the Metadata descriptor URL clicking the Import Keys action in the identity provider page. The action downloads the current certificates in the metadata endpoint and assigns them to the config in this same option. You need to click Save to definitely store the re-imported certificates.", "type": "string" }, "singleSignOnServiceUrl": { "title": "Single Sign-On service URL", "description": "The Url that must be used to send authentication requests (SAML AuthnRequest).", "type": "string" }, "syncMode": { "title": "Sync mode", "description": "Default sync mode for all mappers. The sync mode determines when user data will be synced using the mappers. Possible values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider.", "type": "string", "enum": [ "IMPORT", "LEGACY", "FORCE" ] }, "validateSignature": { "title": "Validate Signatures", "description": "Enable/disable signature validation of external IDP signatures.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAssertionsEncrypted": { "title": "Want Assertions encrypted", "description": "Indicates whether this service provider expects an encrypted Assertion.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAssertionsSigned": { "title": "Want Assertions signed", "description": "Indicates whether this service provider expects a signed Assertion.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAuthnRequestsSigned": { "title": "Want AuthnRequests signed", "description": "Indicates whether the identity provider expects a signed AuthnRequest.", "type": "string", "enum": [ "true", "false", "" ] }, "xmlSigKeyInfoKeyNameTransformer": { "title": "SAML signature key name", "description": "Signed SAML documents contain identification of signing key in KeyName element. For Keycloak / RH-SSO counter-party, use KEY_ID, for MS AD FS use CERT_SUBJECT, for others check and use NONE if no other option works.", "type": "string", "enum": [ "NONE", "KEY_ID", "CERT_SUBJECT" ] } }, "additionalProperties": { "type": "string" } } ``` </details>

---

### spec.definition.identityProviders[].config.allowCreate

Type: string

Allow the external identity provider to create a new identifier to represent the principal.

---

### spec.definition.identityProviders[].config.allowedClockSkew

Type: string

Clock skew in seconds that is tolerated when validating identity provider tokens. Default value is zero.

---

### spec.definition.identityProviders[].config.attributeConsumingServiceIndex

Type: string

Index of the Attribute Consuming Service profile to request during authentication.

---

### spec.definition.identityProviders[].config.authnContextClassRefs

Type: string

Ordered list of requested AuthnContext ClassRefs.

---

### spec.definition.identityProviders[].config.authnContextComparisonType

Type: string

Specifies the comparison method used to evaluate the requested context classes or statements. The default is "Exact".

---

### spec.definition.identityProviders[].config.authnContextDeclRefs

Type: string

Ordered list of requested AuthnContext DeclRefs.

---

### spec.definition.identityProviders[].config.backchannelSupported

Type: string

Does the external IDP support backchannel logout?

---

### spec.definition.identityProviders[].config.encryptionAlgorithm

Type: string

Encryption algorithm, which is used by SAML IDP for encryption of SAML documents, assertions or IDs. The corresponding decryption key for decrypt SAML document parts will be chosen based on this configured algorithm and should be available in realm keys for the encryption (ENC) usage. If algorithm is not configured, then any supported algorithm is allowed and decryption key will be chosen based on the algorithm configured in SAML document itself.

---

### spec.definition.identityProviders[].config.entityId

Type: string

The Entity ID that will be used to uniquely identify this SAML Service Provider.

---

### spec.definition.identityProviders[].config.forceAuthn

Type: string

Indicates whether the identity provider must authenticate the presenter directly rather than rely on a previous security context.

---

### spec.definition.identityProviders[].config.hideOnLoginPage

Type: string

If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.

---

### spec.definition.identityProviders[].config.idpEntityId

Type: string

The Entity ID used to validate the Issuer for received SAML assertions. If empty, no Issuer validation is performed.

---

### spec.definition.identityProviders[].config.loginHint

Type: string

During login phase, forward an optional login_hint query parameter to SAML AuthnRequest's Subject.

---

### spec.definition.identityProviders[].config.nameIDPolicyFormat

Type: string

Specifies the URI reference corresponding to a name identifier format.

---

### spec.definition.identityProviders[].config.postBindingAuthnRequest

Type: string

Indicates whether the AuthnRequest must be sent using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.

---

### spec.definition.identityProviders[].config.postBindingLogout

Type: string

Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.

---

### spec.definition.identityProviders[].config.postBindingResponse

Type: string

Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.

---

### spec.definition.identityProviders[].config.principalAttribute

Type: string

Name or Friendly Name of the attribute used to identify external users.

---

### spec.definition.identityProviders[].config.principalType

Type: string

Way to identify and track external users from the assertion. Default is using Subject NameID, alternatively you can set up identifying attribute.

---

### spec.definition.identityProviders[].config.signSpMetadata

Type: string

Enable/disable signature of the provider SAML metadata.

---

### spec.definition.identityProviders[].config.signatureAlgorithm

Type: string

The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.

---

### spec.definition.identityProviders[].config.signingCertificate

Type: string

The public certificates Keycloak uses to validate the signatures of SAML requests and responses from the external IDP when Use metadata descriptor URL is OFF. Multiple certificates can be entered separated by comma (,). The certificates can be re-imported from the Metadata descriptor URL clicking the Import Keys action in the identity provider page. The action downloads the current certificates in the metadata endpoint and assigns them to the config in this same option. You need to click Save to definitely store the re-imported certificates.

---

### spec.definition.identityProviders[].config.singleSignOnServiceUrl

Type: string

The Url that must be used to send authentication requests (SAML AuthnRequest).

---

### spec.definition.identityProviders[].config.syncMode

Type: string

Default sync mode for all mappers. The sync mode determines when user data will be synced using the mappers. Possible values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider.

---

### spec.definition.identityProviders[].config.validateSignature

Type: string

Enable/disable signature validation of external IDP signatures.

---

### spec.definition.identityProviders[].config.wantAssertionsEncrypted

Type: string

Indicates whether this service provider expects an encrypted Assertion.

---

### spec.definition.identityProviders[].config.wantAssertionsSigned

Type: string

Indicates whether this service provider expects a signed Assertion.

---

### spec.definition.identityProviders[].config.wantAuthnRequestsSigned

Type: string

Indicates whether the identity provider expects a signed AuthnRequest.

---

### spec.definition.identityProviders[].config.xmlSigKeyInfoKeyNameTransformer

Type: string

Signed SAML documents contain identification of signing key in KeyName element. For Keycloak / RH-SSO counter-party, use KEY_ID, for MS AD FS use CERT_SUBJECT, for others check and use NONE if no other option works.

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

### spec.definition.internationalizationEnabled

Type: boolean

If enabled, you can choose which locales you support for this realm and which locale is the default.

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

Select theme for login, OTP, grant, registration and forgot password pages.

---

### spec.definition.loginWithEmailAllowed

Type: boolean

Allow users to log in with their email address.

---

### spec.definition.maxDeltaTimeSeconds

Type: integer

When will failure count be reset?

---

### spec.definition.maxFailureWaitSeconds

Type: integer

Max time a user will be locked out.

---

### spec.definition.maxTemporaryLockouts

Type: integer

The number of temporary lockouts permitted before the user is permanently locked out.

---

### spec.definition.minimumQuickLoginWaitSeconds

Type: integer

How long to wait after a quick login failure.

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

Max time before the device code and user code are expired. This value needs to be a long enough lifetime to be usable (allowing the user to retrieve their secondary device, navigate to the verification URI, login, etc.), but should be sufficiently short to limit the usability of a code obtained for phishing.

---

### spec.definition.oauth2DevicePollingInterval

Type: integer

The minimum amount of time in seconds that the client should wait between polling requests to the token endpoint.

---

### spec.definition.offlineSessionIdleTimeout

Type: integer

Time an offline session is allowed to be idle before it expires. You need to use offline token to refresh at least once within this period; otherwise offline session will expire.

---

### spec.definition.offlineSessionMaxLifespan

Type: integer

Max time before an offline session is expired regardless of activity.

---

### spec.definition.offlineSessionMaxLifespanEnabled

Type: boolean

Enable offline session maximum lifetime

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

OrganizationRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "alias": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "description": { "type": "string" }, "domains": { "type": "array", "items": { "$ref": "#/$defs/OrganizationDomainRepresentation" }, "uniqueItems": true }, "enabled": { "type": "boolean" }, "id": { "type": "string" }, "identityProviders": { "type": "array", "items": { "$ref": "#/$defs/IdentityProviderRepresentation" } }, "members": { "type": "array", "items": { "$ref": "#/$defs/MemberRepresentation" } }, "name": { "type": "string" }, "redirectUrl": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

OrganizationDomainRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "name": { "type": "string" }, "verified": { "type": "boolean" } }, "additionalProperties": false } ``` </details>

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

IdentityProviderRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "addReadTokenRoleOnCreate": { "title": "Stored tokens readable", "description": "Enable/disable if new users can read any stored tokens. This assigns the broker.read-token role.", "type": "boolean" }, "alias": { "title": "Alias", "description": "The alias uniquely identifies an identity provider and it is also used to build the redirect uri.", "type": "string" }, "authenticateByDefault": { "type": "boolean" }, "config": { "type": "object", "properties": { "allowCreate": { "title": "Allow create", "description": "Allow the external identity provider to create a new identifier to represent the principal.", "type": "string", "enum": [ "true", "false", "" ] }, "allowedClockSkew": { "title": "Allowed clock skew", "description": "Clock skew in seconds that is tolerated when validating identity provider tokens. Default value is zero.", "type": "string", "pattern": "^[0-9]*$" }, "attributeConsumingServiceIndex": { "title": "Attribute Consuming Service Index", "description": "Index of the Attribute Consuming Service profile to request during authentication.", "type": "string", "pattern": "^[0-9]*$" }, "authnContextClassRefs": { "title": "AuthnContext ClassRefs", "description": "Ordered list of requested AuthnContext ClassRefs.", "type": "string" }, "authnContextComparisonType": { "title": "Comparison", "description": "Specifies the comparison method used to evaluate the requested context classes or statements. The default is \"Exact\".", "type": "string", "enum": [ "exact", "minimum", "maximum", "better" ] }, "authnContextDeclRefs": { "title": "AuthnContext DeclRefs", "description": "Ordered list of requested AuthnContext DeclRefs.", "type": "string" }, "backchannelSupported": { "title": "Backchannel logout", "description": "Does the external IDP support backchannel logout?", "type": "string", "enum": [ "true", "false", "" ] }, "encryptionAlgorithm": { "title": "Encryption Algorithm", "description": "Encryption algorithm, which is used by SAML IDP for encryption of SAML documents, assertions or IDs. The corresponding decryption key for decrypt SAML document parts will be chosen based on this configured algorithm and should be available in realm keys for the encryption (ENC) usage. If algorithm is not configured, then any supported algorithm is allowed and decryption key will be chosen based on the algorithm configured in SAML document itself.", "type": "string" }, "entityId": { "title": "Service provider entity ID", "description": "The Entity ID that will be used to uniquely identify this SAML Service Provider.", "type": "string" }, "forceAuthn": { "title": "Force authentication", "description": "Indicates whether the identity provider must authenticate the presenter directly rather than rely on a previous security context.", "type": "string", "enum": [ "true", "false", "" ] }, "hideOnLoginPage": { "title": "Hide on login page", "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.", "type": "string", "enum": [ "true", "false", "" ] }, "idpEntityId": { "title": "Identity provider entity ID", "description": "The Entity ID used to validate the Issuer for received SAML assertions. If empty, no Issuer validation is performed.", "type": "string" }, "loginHint": { "title": "Pass subject", "description": "During login phase, forward an optional login_hint query parameter to SAML AuthnRequest's Subject.", "type": "string", "enum": [ "true", "false", "" ] }, "nameIDPolicyFormat": { "title": "NameID policy format", "description": "Specifies the URI reference corresponding to a name identifier format.", "type": "string" }, "postBindingAuthnRequest": { "title": "HTTP-POST binding for AuthnRequest", "description": "Indicates whether the AuthnRequest must be sent using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "postBindingLogout": { "title": "HTTP-POST binding logout", "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "postBindingResponse": { "title": "HTTP-POST binding response", "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "principalAttribute": { "title": "Principal attribute", "description": "Name or Friendly Name of the attribute used to identify external users.", "type": "string" }, "principalType": { "title": "Principal type", "description": "Way to identify and track external users from the assertion. Default is using Subject NameID, alternatively you can set up identifying attribute.", "type": "string", "enum": [ "SUBJECT", "ATTRIBUTE", "FRIENDLY_ATTRIBUTE" ] }, "signSpMetadata": { "title": "Sign service provider metadata", "description": "Enable/disable signature of the provider SAML metadata.", "type": "string", "enum": [ "true", "false", "" ] }, "signatureAlgorithm": { "title": "Signature algorithm", "description": "The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.", "type": "string" }, "signingCertificate": { "title": "Validating X509 certificates", "description": "The public certificates Keycloak uses to validate the signatures of SAML requests and responses from the external IDP when Use metadata descriptor URL is OFF. Multiple certificates can be entered separated by comma (,). The certificates can be re-imported from the Metadata descriptor URL clicking the Import Keys action in the identity provider page. The action downloads the current certificates in the metadata endpoint and assigns them to the config in this same option. You need to click Save to definitely store the re-imported certificates.", "type": "string" }, "singleSignOnServiceUrl": { "title": "Single Sign-On service URL", "description": "The Url that must be used to send authentication requests (SAML AuthnRequest).", "type": "string" }, "syncMode": { "title": "Sync mode", "description": "Default sync mode for all mappers. The sync mode determines when user data will be synced using the mappers. Possible values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider.", "type": "string", "enum": [ "IMPORT", "LEGACY", "FORCE" ] }, "validateSignature": { "title": "Validate Signatures", "description": "Enable/disable signature validation of external IDP signatures.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAssertionsEncrypted": { "title": "Want Assertions encrypted", "description": "Indicates whether this service provider expects an encrypted Assertion.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAssertionsSigned": { "title": "Want Assertions signed", "description": "Indicates whether this service provider expects a signed Assertion.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAuthnRequestsSigned": { "title": "Want AuthnRequests signed", "description": "Indicates whether the identity provider expects a signed AuthnRequest.", "type": "string", "enum": [ "true", "false", "" ] }, "xmlSigKeyInfoKeyNameTransformer": { "title": "SAML signature key name", "description": "Signed SAML documents contain identification of signing key in KeyName element. For Keycloak / RH-SSO counter-party, use KEY_ID, for MS AD FS use CERT_SUBJECT, for others check and use NONE if no other option works.", "type": "string", "enum": [ "NONE", "KEY_ID", "CERT_SUBJECT" ] } }, "additionalProperties": { "type": "string" } }, "displayName": { "title": "Display name", "description": "Friendly name for Identity Providers.", "type": "string" }, "enabled": { "title": "Enabled", "type": "boolean" }, "firstBrokerLoginFlowAlias": { "title": "First login flow override", "description": "Alias of authentication flow, which is triggered after first login with this identity provider. Term 'First Login' means that no Keycloak account is currently linked to the authenticated identity provider account.", "type": "string" }, "hideOnLogin": { "title": "Hide on login page", "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.", "type": "boolean" }, "internalId": { "type": "string" }, "linkOnly": { "title": "Account linking only", "description": "If true, users cannot log in through this provider.  They can only link to this provider.  This is useful if you don't want to allow login from the provider, but want to integrate with a provider.", "type": "boolean" }, "organizationId": { "type": "string" }, "postBrokerLoginFlowAlias": { "title": "Post login flow", "description": "Alias of authentication flow, which is triggered after each login with this identity provider. Useful if you want additional verification of each user authenticated with this identity provider (for example OTP). Leave this to \"None\" if you need no any additional authenticators to be triggered after login with this identity provider. Also note that authenticator implementations must assume that user is already set in ClientSession as identity provider already set it.", "type": "string" }, "providerId": { "type": "string" }, "storeToken": { "title": "Store tokens", "description": "Enable/disable if tokens must be stored after authenticating users.", "type": "boolean" }, "trustEmail": { "title": "Trust Email", "description": "If enabled, email provided by this provider is not verified even if verification is enabled for the realm.", "type": "boolean" }, "updateProfileFirstLogin": { "type": "boolean" }, "updateProfileFirstLoginMode": { "type": "string" } }, "additionalProperties": false } ``` </details>

---

### spec.definition.organizations[].identityProviders[].addReadTokenRoleOnCreate

Type: boolean

Enable/disable if new users can read any stored tokens. This assigns the broker.read-token role.

---

### spec.definition.organizations[].identityProviders[].alias

Type: string

The alias uniquely identifies an identity provider and it is also used to build the redirect uri.

---

### spec.definition.organizations[].identityProviders[].authenticateByDefault

Type: boolean

*missing*

---

### spec.definition.organizations[].identityProviders[].config

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[allowCreate](#specdefinitionorganizationsidentityprovidersconfigallowcreate)|string||
|[allowedClockSkew](#specdefinitionorganizationsidentityprovidersconfigallowedclockskew)|string||
|[attributeConsumingServiceIndex](#specdefinitionorganizationsidentityprovidersconfigattributeconsumingserviceindex)|string||
|[authnContextClassRefs](#specdefinitionorganizationsidentityprovidersconfigauthncontextclassrefs)|string||
|[authnContextComparisonType](#specdefinitionorganizationsidentityprovidersconfigauthncontextcomparisontype)|string||
|[authnContextDeclRefs](#specdefinitionorganizationsidentityprovidersconfigauthncontextdeclrefs)|string||
|[backchannelSupported](#specdefinitionorganizationsidentityprovidersconfigbackchannelsupported)|string||
|[encryptionAlgorithm](#specdefinitionorganizationsidentityprovidersconfigencryptionalgorithm)|string||
|[entityId](#specdefinitionorganizationsidentityprovidersconfigentityid)|string||
|[forceAuthn](#specdefinitionorganizationsidentityprovidersconfigforceauthn)|string||
|[hideOnLoginPage](#specdefinitionorganizationsidentityprovidersconfighideonloginpage)|string||
|[idpEntityId](#specdefinitionorganizationsidentityprovidersconfigidpentityid)|string||
|[loginHint](#specdefinitionorganizationsidentityprovidersconfigloginhint)|string||
|[nameIDPolicyFormat](#specdefinitionorganizationsidentityprovidersconfignameidpolicyformat)|string||
|[postBindingAuthnRequest](#specdefinitionorganizationsidentityprovidersconfigpostbindingauthnrequest)|string||
|[postBindingLogout](#specdefinitionorganizationsidentityprovidersconfigpostbindinglogout)|string||
|[postBindingResponse](#specdefinitionorganizationsidentityprovidersconfigpostbindingresponse)|string||
|[principalAttribute](#specdefinitionorganizationsidentityprovidersconfigprincipalattribute)|string||
|[principalType](#specdefinitionorganizationsidentityprovidersconfigprincipaltype)|string||
|[signSpMetadata](#specdefinitionorganizationsidentityprovidersconfigsignspmetadata)|string||
|[signatureAlgorithm](#specdefinitionorganizationsidentityprovidersconfigsignaturealgorithm)|string||
|[signingCertificate](#specdefinitionorganizationsidentityprovidersconfigsigningcertificate)|string||
|[singleSignOnServiceUrl](#specdefinitionorganizationsidentityprovidersconfigsinglesignonserviceurl)|string||
|[syncMode](#specdefinitionorganizationsidentityprovidersconfigsyncmode)|string||
|[validateSignature](#specdefinitionorganizationsidentityprovidersconfigvalidatesignature)|string||
|[wantAssertionsEncrypted](#specdefinitionorganizationsidentityprovidersconfigwantassertionsencrypted)|string||
|[wantAssertionsSigned](#specdefinitionorganizationsidentityprovidersconfigwantassertionssigned)|string||
|[wantAuthnRequestsSigned](#specdefinitionorganizationsidentityprovidersconfigwantauthnrequestssigned)|string||
|[xmlSigKeyInfoKeyNameTransformer](#specdefinitionorganizationsidentityprovidersconfigxmlsigkeyinfokeynametransformer)|string||

IdentityProviderRepresentationConfig

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "allowCreate": { "title": "Allow create", "description": "Allow the external identity provider to create a new identifier to represent the principal.", "type": "string", "enum": [ "true", "false", "" ] }, "allowedClockSkew": { "title": "Allowed clock skew", "description": "Clock skew in seconds that is tolerated when validating identity provider tokens. Default value is zero.", "type": "string", "pattern": "^[0-9]*$" }, "attributeConsumingServiceIndex": { "title": "Attribute Consuming Service Index", "description": "Index of the Attribute Consuming Service profile to request during authentication.", "type": "string", "pattern": "^[0-9]*$" }, "authnContextClassRefs": { "title": "AuthnContext ClassRefs", "description": "Ordered list of requested AuthnContext ClassRefs.", "type": "string" }, "authnContextComparisonType": { "title": "Comparison", "description": "Specifies the comparison method used to evaluate the requested context classes or statements. The default is \"Exact\".", "type": "string", "enum": [ "exact", "minimum", "maximum", "better" ] }, "authnContextDeclRefs": { "title": "AuthnContext DeclRefs", "description": "Ordered list of requested AuthnContext DeclRefs.", "type": "string" }, "backchannelSupported": { "title": "Backchannel logout", "description": "Does the external IDP support backchannel logout?", "type": "string", "enum": [ "true", "false", "" ] }, "encryptionAlgorithm": { "title": "Encryption Algorithm", "description": "Encryption algorithm, which is used by SAML IDP for encryption of SAML documents, assertions or IDs. The corresponding decryption key for decrypt SAML document parts will be chosen based on this configured algorithm and should be available in realm keys for the encryption (ENC) usage. If algorithm is not configured, then any supported algorithm is allowed and decryption key will be chosen based on the algorithm configured in SAML document itself.", "type": "string" }, "entityId": { "title": "Service provider entity ID", "description": "The Entity ID that will be used to uniquely identify this SAML Service Provider.", "type": "string" }, "forceAuthn": { "title": "Force authentication", "description": "Indicates whether the identity provider must authenticate the presenter directly rather than rely on a previous security context.", "type": "string", "enum": [ "true", "false", "" ] }, "hideOnLoginPage": { "title": "Hide on login page", "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.", "type": "string", "enum": [ "true", "false", "" ] }, "idpEntityId": { "title": "Identity provider entity ID", "description": "The Entity ID used to validate the Issuer for received SAML assertions. If empty, no Issuer validation is performed.", "type": "string" }, "loginHint": { "title": "Pass subject", "description": "During login phase, forward an optional login_hint query parameter to SAML AuthnRequest's Subject.", "type": "string", "enum": [ "true", "false", "" ] }, "nameIDPolicyFormat": { "title": "NameID policy format", "description": "Specifies the URI reference corresponding to a name identifier format.", "type": "string" }, "postBindingAuthnRequest": { "title": "HTTP-POST binding for AuthnRequest", "description": "Indicates whether the AuthnRequest must be sent using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "postBindingLogout": { "title": "HTTP-POST binding logout", "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "postBindingResponse": { "title": "HTTP-POST binding response", "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "principalAttribute": { "title": "Principal attribute", "description": "Name or Friendly Name of the attribute used to identify external users.", "type": "string" }, "principalType": { "title": "Principal type", "description": "Way to identify and track external users from the assertion. Default is using Subject NameID, alternatively you can set up identifying attribute.", "type": "string", "enum": [ "SUBJECT", "ATTRIBUTE", "FRIENDLY_ATTRIBUTE" ] }, "signSpMetadata": { "title": "Sign service provider metadata", "description": "Enable/disable signature of the provider SAML metadata.", "type": "string", "enum": [ "true", "false", "" ] }, "signatureAlgorithm": { "title": "Signature algorithm", "description": "The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.", "type": "string" }, "signingCertificate": { "title": "Validating X509 certificates", "description": "The public certificates Keycloak uses to validate the signatures of SAML requests and responses from the external IDP when Use metadata descriptor URL is OFF. Multiple certificates can be entered separated by comma (,). The certificates can be re-imported from the Metadata descriptor URL clicking the Import Keys action in the identity provider page. The action downloads the current certificates in the metadata endpoint and assigns them to the config in this same option. You need to click Save to definitely store the re-imported certificates.", "type": "string" }, "singleSignOnServiceUrl": { "title": "Single Sign-On service URL", "description": "The Url that must be used to send authentication requests (SAML AuthnRequest).", "type": "string" }, "syncMode": { "title": "Sync mode", "description": "Default sync mode for all mappers. The sync mode determines when user data will be synced using the mappers. Possible values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider.", "type": "string", "enum": [ "IMPORT", "LEGACY", "FORCE" ] }, "validateSignature": { "title": "Validate Signatures", "description": "Enable/disable signature validation of external IDP signatures.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAssertionsEncrypted": { "title": "Want Assertions encrypted", "description": "Indicates whether this service provider expects an encrypted Assertion.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAssertionsSigned": { "title": "Want Assertions signed", "description": "Indicates whether this service provider expects a signed Assertion.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAuthnRequestsSigned": { "title": "Want AuthnRequests signed", "description": "Indicates whether the identity provider expects a signed AuthnRequest.", "type": "string", "enum": [ "true", "false", "" ] }, "xmlSigKeyInfoKeyNameTransformer": { "title": "SAML signature key name", "description": "Signed SAML documents contain identification of signing key in KeyName element. For Keycloak / RH-SSO counter-party, use KEY_ID, for MS AD FS use CERT_SUBJECT, for others check and use NONE if no other option works.", "type": "string", "enum": [ "NONE", "KEY_ID", "CERT_SUBJECT" ] } }, "additionalProperties": { "type": "string" } } ``` </details>

---

### spec.definition.organizations[].identityProviders[].config.allowCreate

Type: string

Allow the external identity provider to create a new identifier to represent the principal.

---

### spec.definition.organizations[].identityProviders[].config.allowedClockSkew

Type: string

Clock skew in seconds that is tolerated when validating identity provider tokens. Default value is zero.

---

### spec.definition.organizations[].identityProviders[].config.attributeConsumingServiceIndex

Type: string

Index of the Attribute Consuming Service profile to request during authentication.

---

### spec.definition.organizations[].identityProviders[].config.authnContextClassRefs

Type: string

Ordered list of requested AuthnContext ClassRefs.

---

### spec.definition.organizations[].identityProviders[].config.authnContextComparisonType

Type: string

Specifies the comparison method used to evaluate the requested context classes or statements. The default is "Exact".

---

### spec.definition.organizations[].identityProviders[].config.authnContextDeclRefs

Type: string

Ordered list of requested AuthnContext DeclRefs.

---

### spec.definition.organizations[].identityProviders[].config.backchannelSupported

Type: string

Does the external IDP support backchannel logout?

---

### spec.definition.organizations[].identityProviders[].config.encryptionAlgorithm

Type: string

Encryption algorithm, which is used by SAML IDP for encryption of SAML documents, assertions or IDs. The corresponding decryption key for decrypt SAML document parts will be chosen based on this configured algorithm and should be available in realm keys for the encryption (ENC) usage. If algorithm is not configured, then any supported algorithm is allowed and decryption key will be chosen based on the algorithm configured in SAML document itself.

---

### spec.definition.organizations[].identityProviders[].config.entityId

Type: string

The Entity ID that will be used to uniquely identify this SAML Service Provider.

---

### spec.definition.organizations[].identityProviders[].config.forceAuthn

Type: string

Indicates whether the identity provider must authenticate the presenter directly rather than rely on a previous security context.

---

### spec.definition.organizations[].identityProviders[].config.hideOnLoginPage

Type: string

If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.

---

### spec.definition.organizations[].identityProviders[].config.idpEntityId

Type: string

The Entity ID used to validate the Issuer for received SAML assertions. If empty, no Issuer validation is performed.

---

### spec.definition.organizations[].identityProviders[].config.loginHint

Type: string

During login phase, forward an optional login_hint query parameter to SAML AuthnRequest's Subject.

---

### spec.definition.organizations[].identityProviders[].config.nameIDPolicyFormat

Type: string

Specifies the URI reference corresponding to a name identifier format.

---

### spec.definition.organizations[].identityProviders[].config.postBindingAuthnRequest

Type: string

Indicates whether the AuthnRequest must be sent using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.

---

### spec.definition.organizations[].identityProviders[].config.postBindingLogout

Type: string

Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.

---

### spec.definition.organizations[].identityProviders[].config.postBindingResponse

Type: string

Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.

---

### spec.definition.organizations[].identityProviders[].config.principalAttribute

Type: string

Name or Friendly Name of the attribute used to identify external users.

---

### spec.definition.organizations[].identityProviders[].config.principalType

Type: string

Way to identify and track external users from the assertion. Default is using Subject NameID, alternatively you can set up identifying attribute.

---

### spec.definition.organizations[].identityProviders[].config.signSpMetadata

Type: string

Enable/disable signature of the provider SAML metadata.

---

### spec.definition.organizations[].identityProviders[].config.signatureAlgorithm

Type: string

The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.

---

### spec.definition.organizations[].identityProviders[].config.signingCertificate

Type: string

The public certificates Keycloak uses to validate the signatures of SAML requests and responses from the external IDP when Use metadata descriptor URL is OFF. Multiple certificates can be entered separated by comma (,). The certificates can be re-imported from the Metadata descriptor URL clicking the Import Keys action in the identity provider page. The action downloads the current certificates in the metadata endpoint and assigns them to the config in this same option. You need to click Save to definitely store the re-imported certificates.

---

### spec.definition.organizations[].identityProviders[].config.singleSignOnServiceUrl

Type: string

The Url that must be used to send authentication requests (SAML AuthnRequest).

---

### spec.definition.organizations[].identityProviders[].config.syncMode

Type: string

Default sync mode for all mappers. The sync mode determines when user data will be synced using the mappers. Possible values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider.

---

### spec.definition.organizations[].identityProviders[].config.validateSignature

Type: string

Enable/disable signature validation of external IDP signatures.

---

### spec.definition.organizations[].identityProviders[].config.wantAssertionsEncrypted

Type: string

Indicates whether this service provider expects an encrypted Assertion.

---

### spec.definition.organizations[].identityProviders[].config.wantAssertionsSigned

Type: string

Indicates whether this service provider expects a signed Assertion.

---

### spec.definition.organizations[].identityProviders[].config.wantAuthnRequestsSigned

Type: string

Indicates whether the identity provider expects a signed AuthnRequest.

---

### spec.definition.organizations[].identityProviders[].config.xmlSigKeyInfoKeyNameTransformer

Type: string

Signed SAML documents contain identification of signing key in KeyName element. For Keycloak / RH-SSO counter-party, use KEY_ID, for MS AD FS use CERT_SUBJECT, for others check and use NONE if no other option works.

---

### spec.definition.organizations[].identityProviders[].displayName

Type: string

Friendly name for Identity Providers.

---

### spec.definition.organizations[].identityProviders[].enabled

Type: boolean

*missing*

---

### spec.definition.organizations[].identityProviders[].firstBrokerLoginFlowAlias

Type: string

Alias of authentication flow, which is triggered after first login with this identity provider. Term 'First Login' means that no Keycloak account is currently linked to the authenticated identity provider account.

---

### spec.definition.organizations[].identityProviders[].hideOnLogin

Type: boolean

If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.

---

### spec.definition.organizations[].identityProviders[].internalId

Type: string

*missing*

---

### spec.definition.organizations[].identityProviders[].linkOnly

Type: boolean

If true, users cannot log in through this provider.  They can only link to this provider.  This is useful if you don't want to allow login from the provider, but want to integrate with a provider.

---

### spec.definition.organizations[].identityProviders[].organizationId

Type: string

*missing*

---

### spec.definition.organizations[].identityProviders[].postBrokerLoginFlowAlias

Type: string

Alias of authentication flow, which is triggered after each login with this identity provider. Useful if you want additional verification of each user authenticated with this identity provider (for example OTP). Leave this to "None" if you need no any additional authenticators to be triggered after login with this identity provider. Also note that authenticator implementations must assume that user is already set in ClientSession as identity provider already set it.

---

### spec.definition.organizations[].identityProviders[].providerId

Type: string

*missing*

---

### spec.definition.organizations[].identityProviders[].storeToken

Type: boolean

Enable/disable if tokens must be stored after authenticating users.

---

### spec.definition.organizations[].identityProviders[].trustEmail

Type: boolean

If enabled, email provided by this provider is not verified even if verification is enabled for the realm.

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

MemberRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "access": { "type": "object", "additionalProperties": { "type": "boolean" } }, "applicationRoles": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "clientConsents": { "type": "array", "items": { "$ref": "#/$defs/UserConsentRepresentation" } }, "clientRoles": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "createdTimestamp": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "credentials": { "type": "array", "items": { "$ref": "#/$defs/CredentialRepresentation" } }, "disableableCredentialTypes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "email": { "type": "string" }, "emailVerified": { "type": "boolean" }, "enabled": { "type": "boolean" }, "federatedIdentities": { "type": "array", "items": { "$ref": "#/$defs/FederatedIdentityRepresentation" } }, "federationLink": { "type": "string" }, "firstName": { "type": "string" }, "groups": { "type": "array", "items": { "type": "string" } }, "id": { "type": "string" }, "lastName": { "type": "string" }, "membershipType": { "$ref": "#/$defs/MembershipType" }, "notBefore": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "origin": { "type": "string" }, "realmRoles": { "type": "array", "items": { "type": "string" } }, "requiredActions": { "type": "array", "items": { "type": "string" } }, "self": { "type": "string" }, "serviceAccountClientId": { "type": "string" }, "socialLinks": { "type": "array", "items": { "$ref": "#/$defs/SocialLinkRepresentation" } }, "totp": { "type": "boolean" }, "userProfileMetadata": { "$ref": "#/$defs/UserProfileMetadata" }, "username": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

UserConsentRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "clientId": { "type": "string" }, "createdDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "grantedClientScopes": { "type": "array", "items": { "type": "string" } }, "grantedRealmRoles": { "type": "array", "items": { "type": "string" } }, "lastUpdatedDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 } }, "additionalProperties": false } ``` </details>

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

CredentialRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "algorithm": { "type": "string" }, "config": { "type": "object", "allOf": [ { "$ref": "#/$defs/MultivaluedHashMapStringString" } ] }, "counter": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "createdDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "credentialData": { "type": "string" }, "device": { "type": "string" }, "digits": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "hashIterations": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "hashedSaltedValue": { "type": "string" }, "id": { "type": "string" }, "period": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "priority": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "salt": { "type": "string" }, "secretData": { "type": "string" }, "temporary": { "type": "boolean" }, "type": { "type": "string" }, "userLabel": { "type": "string" }, "value": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

FederatedIdentityRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "identityProvider": { "type": "string" }, "userId": { "type": "string" }, "userName": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

MembershipType

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "UNMANAGED", "MANAGED" ] } ``` </details>

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

SocialLinkRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "socialProvider": { "type": "string" }, "socialUserId": { "type": "string" }, "socialUsername": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

UserProfileMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "attributes": { "type": "array", "items": { "$ref": "#/$defs/UserProfileAttributeMetadata" } }, "groups": { "type": "array", "items": { "$ref": "#/$defs/UserProfileAttributeGroupMetadata" } } }, "additionalProperties": false } ``` </details>

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

UserProfileAttributeMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "annotations": { "type": "object", "additionalProperties": {} }, "displayName": { "type": "string" }, "group": { "type": "string" }, "multivalued": { "type": "boolean" }, "name": { "type": "string" }, "readOnly": { "type": "boolean" }, "required": { "type": "boolean" }, "validators": { "type": "object", "additionalProperties": { "type": "object", "additionalProperties": {} } } }, "additionalProperties": false } ``` </details>

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

UserProfileAttributeGroupMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "annotations": { "type": "object", "additionalProperties": {} }, "displayDescription": { "type": "string" }, "displayHeader": { "type": "string" }, "name": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

What hashing algorithm should be used to generate the OTP.

---

### spec.definition.otpPolicyCodeReusable

Type: boolean

Possibility to use the same OTP code again after successful authentication.

---

### spec.definition.otpPolicyDigits

Type: integer

How many digits should the OTP have?

---

### spec.definition.otpPolicyInitialCounter

Type: integer

*missing*

---

### spec.definition.otpPolicyLookAheadWindow

Type: integer

How far around (extra token periods or counts) should the server look just in case the token generator and server are out of time sync or counter sync?

---

### spec.definition.otpPolicyPeriod

Type: integer

How many seconds should an OTP token be valid? Defaults to 30 seconds.

---

### spec.definition.otpPolicyType

Type: string

totp is Time-Based One Time Password. 'hotp' is a counter base one time password in which the server keeps a counter to hash against.

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

ProtocolMapperRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "consentRequired": { "type": "boolean" }, "consentText": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "protocol": { "type": "string", "enum": [ "openid-connect", "saml" ] }, "protocolMapper": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

ProtocolMapperRepresentationProtocol

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "openid-connect", "saml" ] } ``` </details>

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

If a failure happens concurrently too quickly, lock out the user.

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

Maximum number of times a refresh token can be reused. When a different token is used, revocation is immediate.

---

### spec.definition.registrationAllowed

Type: boolean

Enable/disable the registration page. A link for registration will show on login page too.

---

### spec.definition.registrationEmailAsUsername

Type: boolean

Allow users to set email as username.

---

### spec.definition.registrationFlow

Type: string

*missing*

---

### spec.definition.rememberMe

Type: boolean

Show checkbox on login page to allow user to remain logged in between browser restarts until session expires.

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

RequiredActionProviderRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "alias": { "type": "string" }, "config": { "type": "object", "additionalProperties": { "type": "string" } }, "defaultAction": { "type": "boolean" }, "enabled": { "type": "boolean" }, "name": { "type": "string" }, "priority": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "providerId": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

Show a link on login page for user to click when they have forgotten their credentials.

---

### spec.definition.revokeRefreshToken

Type: boolean

If enabled a refresh token can only be used up to 'Refresh Token Max Reuse' and is revoked when a different token is used. Otherwise refresh tokens are not revoked when used and can be used multiple times.

---

### spec.definition.roles

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[application](#specdefinitionrolesapplication)|object||
|[client](#specdefinitionrolesclient)|object||
|[realm[]](#specdefinitionrolesrealm)|object||

RolesRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "application": { "type": "object", "additionalProperties": { "type": "array", "items": { "$ref": "#/$defs/RoleRepresentation" } } }, "client": { "type": "object", "additionalProperties": { "type": "array", "items": { "$ref": "#/$defs/RoleRepresentation" } } }, "realm": { "type": "array", "items": { "$ref": "#/$defs/RoleRepresentation" } } }, "additionalProperties": false } ``` </details>

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

RoleRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "clientRole": { "type": "boolean" }, "composite": { "type": "boolean" }, "composites": { "$ref": "#/$defs/Composites" }, "containerId": { "type": "string" }, "description": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "scopeParamRequired": { "type": "boolean" } }, "additionalProperties": false } ``` </details>

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

Composites

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "application": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "client": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "realm": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } }, "additionalProperties": false } ``` </details>

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

ScopeMappingRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "client": { "type": "string" }, "clientScope": { "type": "string" }, "clientTemplate": { "type": "string" }, "roles": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "self": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

|Property|Type|Required|
|:-------|:---|:------:|
|[auth](#specdefinitionsmtpserverauth)|string||
|[envelopeFrom](#specdefinitionsmtpserverenvelopefrom)|string||
|[from](#specdefinitionsmtpserverfrom)|string||
|[fromDisplayName](#specdefinitionsmtpserverfromdisplayname)|string||
|[host](#specdefinitionsmtpserverhost)|string||
|[password](#specdefinitionsmtpserverpassword)|string||
|[port](#specdefinitionsmtpserverport)|string||
|[replyTo](#specdefinitionsmtpserverreplyto)|string||
|[replyToDisplayName](#specdefinitionsmtpserverreplytodisplayname)|string||
|[ssl](#specdefinitionsmtpserverssl)|string||
|[starttls](#specdefinitionsmtpserverstarttls)|string||
|[user](#specdefinitionsmtpserveruser)|string||

RealmRepresentationSmtpServer

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "auth": { "title": "Authentication", "type": "string", "enum": [ "true", "false", "" ] }, "envelopeFrom": { "title": "Envelope from", "description": "An email address used for bounces (optional).", "type": "string" }, "from": { "title": "From", "type": "string" }, "fromDisplayName": { "title": "From display name", "description": "A user-friendly name for the 'From' address (optional).", "type": "string" }, "host": { "title": "Host", "type": "string" }, "password": { "title": "Password", "description": "SMTP password. This field is able to obtain its value from vault, use ${vault.ID} format.", "type": "string" }, "port": { "title": "Port", "type": "string" }, "replyTo": { "title": "Reply to", "type": "string" }, "replyToDisplayName": { "title": "Reply to display name", "description": "A user-friendly name for the 'Reply-To' address (optional).", "type": "string" }, "ssl": { "title": "Enable SSL", "type": "string", "enum": [ "true", "false", "" ] }, "starttls": { "title": "Enable StartTLS", "type": "string", "enum": [ "true", "false", "" ] }, "user": { "title": "Username", "type": "string" } }, "additionalProperties": false } ``` </details>

---

### spec.definition.smtpServer.auth

Type: string

Authentication

<details><summary>JSON schema</summary>

```json { "title": "Authentication", "type": "string", "enum": [ "true", "false", "" ] } ``` </details>

---

### spec.definition.smtpServer.envelopeFrom

Type: string

An email address used for bounces (optional).

---

### spec.definition.smtpServer.from

Type: string

*missing*

---

### spec.definition.smtpServer.fromDisplayName

Type: string

A user-friendly name for the 'From' address (optional).

---

### spec.definition.smtpServer.host

Type: string

*missing*

---

### spec.definition.smtpServer.password

Type: string

SMTP password. This field is able to obtain its value from vault, use ${vault.ID} format.

---

### spec.definition.smtpServer.port

Type: string

*missing*

---

### spec.definition.smtpServer.replyTo

Type: string

*missing*

---

### spec.definition.smtpServer.replyToDisplayName

Type: string

A user-friendly name for the 'Reply-To' address (optional).

---

### spec.definition.smtpServer.ssl

Type: string

EnableSsl

<details><summary>JSON schema</summary>

```json { "title": "Enable SSL", "type": "string", "enum": [ "true", "false", "" ] } ``` </details>

---

### spec.definition.smtpServer.starttls

Type: string

EnableStartTls

<details><summary>JSON schema</summary>

```json { "title": "Enable StartTLS", "type": "string", "enum": [ "true", "false", "" ] } ``` </details>

---

### spec.definition.smtpServer.user

Type: string

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

Is HTTPS required? 'None' means HTTPS is not required for any client IP address. 'External requests' means localhost and private IP addresses can access without HTTPS. 'All requests' means HTTPS is required for all IP addresses.

---

### spec.definition.ssoSessionIdleTimeout

Type: integer

Time a session is allowed to be idle before it expires. Tokens and browser sessions are invalidated when a session is expired.

---

### spec.definition.ssoSessionIdleTimeoutRememberMe

Type: integer

Time a remember me session is allowed to be idle before it expires. Tokens and browser sessions are invalidated when a session is expired. If not set it uses the standard SSO Session Idle value.

---

### spec.definition.ssoSessionMaxLifespan

Type: integer

Max time before a session is expired. Tokens and browser sessions are invalidated when a session is expired.

---

### spec.definition.ssoSessionMaxLifespanRememberMe

Type: integer

Max time before a session is expired when a user has set the remember me option. Tokens and browser sessions are invalidated when a session is expired. If not set it uses the standard SSO Session Max value.

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

UserFederationMapperRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "federationMapperType": { "type": "string" }, "federationProviderDisplayName": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

UserFederationProviderRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "changedSyncPeriod": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "config": { "type": "object", "additionalProperties": { "type": "string" } }, "displayName": { "type": "string" }, "fullSyncPeriod": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "id": { "type": "string" }, "lastSync": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "priority": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "providerName": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

If enabled, users are allowed to manage their resources and permissions using the Account Management UI.

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

UserRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "access": { "type": "object", "additionalProperties": { "type": "boolean" } }, "applicationRoles": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "clientConsents": { "type": "array", "items": { "$ref": "#/$defs/UserConsentRepresentation" } }, "clientRoles": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "createdTimestamp": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "credentials": { "type": "array", "items": { "$ref": "#/$defs/CredentialRepresentation" } }, "disableableCredentialTypes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "email": { "type": "string" }, "emailVerified": { "type": "boolean" }, "enabled": { "type": "boolean" }, "federatedIdentities": { "type": "array", "items": { "$ref": "#/$defs/FederatedIdentityRepresentation" } }, "federationLink": { "type": "string" }, "firstName": { "type": "string" }, "groups": { "type": "array", "items": { "type": "string" } }, "id": { "type": "string" }, "lastName": { "type": "string" }, "notBefore": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "origin": { "type": "string" }, "realmRoles": { "type": "array", "items": { "type": "string" } }, "requiredActions": { "type": "array", "items": { "type": "string" } }, "self": { "type": "string" }, "serviceAccountClientId": { "type": "string" }, "socialLinks": { "type": "array", "items": { "$ref": "#/$defs/SocialLinkRepresentation" } }, "totp": { "type": "boolean" }, "userProfileMetadata": { "$ref": "#/$defs/UserProfileMetadata" }, "username": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

UserConsentRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "clientId": { "type": "string" }, "createdDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "grantedClientScopes": { "type": "array", "items": { "type": "string" } }, "grantedRealmRoles": { "type": "array", "items": { "type": "string" } }, "lastUpdatedDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 } }, "additionalProperties": false } ``` </details>

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

CredentialRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "algorithm": { "type": "string" }, "config": { "type": "object", "allOf": [ { "$ref": "#/$defs/MultivaluedHashMapStringString" } ] }, "counter": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "createdDate": { "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "credentialData": { "type": "string" }, "device": { "type": "string" }, "digits": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "hashIterations": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "hashedSaltedValue": { "type": "string" }, "id": { "type": "string" }, "period": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "priority": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "salt": { "type": "string" }, "secretData": { "type": "string" }, "temporary": { "type": "boolean" }, "type": { "type": "string" }, "userLabel": { "type": "string" }, "value": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

FederatedIdentityRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "identityProvider": { "type": "string" }, "userId": { "type": "string" }, "userName": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

SocialLinkRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "socialProvider": { "type": "string" }, "socialUserId": { "type": "string" }, "socialUsername": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

UserProfileMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "attributes": { "type": "array", "items": { "$ref": "#/$defs/UserProfileAttributeMetadata" } }, "groups": { "type": "array", "items": { "$ref": "#/$defs/UserProfileAttributeGroupMetadata" } } }, "additionalProperties": false } ``` </details>

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

UserProfileAttributeMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "annotations": { "type": "object", "additionalProperties": {} }, "displayName": { "type": "string" }, "group": { "type": "string" }, "multivalued": { "type": "boolean" }, "name": { "type": "string" }, "readOnly": { "type": "boolean" }, "required": { "type": "boolean" }, "validators": { "type": "object", "additionalProperties": { "type": "object", "additionalProperties": {} } } }, "additionalProperties": false } ``` </details>

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

UserProfileAttributeGroupMetadata

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "annotations": { "type": "object", "additionalProperties": {} }, "displayDescription": { "type": "string" }, "displayHeader": { "type": "string" }, "name": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

### spec.definition.verifiableCredentialsEnabled

Type: boolean

*missing*

---

### spec.definition.verifyEmail

Type: boolean

Require user to verify their email address after initial login or after address changes are submitted.

---

### spec.definition.waitIncrementSeconds

Type: integer

When failure threshold has been met, how much time should the user be locked out?

---

### spec.definition.webAuthnPolicyAcceptableAaguids[]

Type: string

*missing*

---

### spec.definition.webAuthnPolicyAttestationConveyancePreference

Type: string

Communicates to an authenticator the preference of how to generate an attestation statement.

---

### spec.definition.webAuthnPolicyAuthenticatorAttachment

Type: string

Communicates to an authenticator an acceptable attachment pattern.

---

### spec.definition.webAuthnPolicyAvoidSameAuthenticatorRegister

Type: boolean

Avoid registering an authenticator that has already been registered.

---

### spec.definition.webAuthnPolicyCreateTimeout

Type: integer

The timeout value for creating the user's public key credential in seconds. If set to 0, this timeout option is not adapted.

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

Communicates to an authenticator the preference of how to generate an attestation statement.

---

### spec.definition.webAuthnPolicyPasswordlessAuthenticatorAttachment

Type: string

Communicates to an authenticator an acceptable attachment pattern.

---

### spec.definition.webAuthnPolicyPasswordlessAvoidSameAuthenticatorRegister

Type: boolean

Avoid registering an authenticator that has already been registered.

---

### spec.definition.webAuthnPolicyPasswordlessCreateTimeout

Type: integer

The timeout value for creating the user's public key credential in seconds. If set to 0, this timeout option is not adapted.

---

### spec.definition.webAuthnPolicyPasswordlessExtraOrigins[]

Type: string

*missing*

---

### spec.definition.webAuthnPolicyPasswordlessRequireResidentKey

Type: string

It tells an authenticator whether to create a public key credential as a Discoverable Credential.

---

### spec.definition.webAuthnPolicyPasswordlessRpEntityName

Type: string

Human-readable server name as WebAuthn Relying Party

---

### spec.definition.webAuthnPolicyPasswordlessRpId

Type: string

The WebAuthn Relying Party ID (RpID). It must be the origin's effective domain, e.g. 'company.com' or 'auth.company.com'.

---

### spec.definition.webAuthnPolicyPasswordlessSignatureAlgorithms[]

Type: string

SignatureAlgorithmsItem

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "Ed25519", "ES256", "ES384", "ES512", "RS256", "RS384", "RS512", "RS1" ] } ``` </details>

---

### spec.definition.webAuthnPolicyPasswordlessUserVerificationRequirement

Type: string

Communicates to an authenticator whether to require to verify a user.

---

### spec.definition.webAuthnPolicyRequireResidentKey

Type: string

It tells an authenticator whether to create a public key credential as a Discoverable Credential.

---

### spec.definition.webAuthnPolicyRpEntityName

Type: string

Human-readable server name as WebAuthn Relying Party

---

### spec.definition.webAuthnPolicyRpId

Type: string

The WebAuthn Relying Party ID (RpID). It must be the origin's effective domain, e.g. 'company.com' or 'auth.company.com'.

---

### spec.definition.webAuthnPolicySignatureAlgorithms[]

Type: string

SignatureAlgorithmsItem

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "Ed25519", "ES256", "ES384", "ES512", "RS256", "RS384", "RS512", "RS1" ] } ``` </details>

---

### spec.definition.webAuthnPolicyUserVerificationRequirement

Type: string

Communicates to an authenticator whether to require to verify a user.

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