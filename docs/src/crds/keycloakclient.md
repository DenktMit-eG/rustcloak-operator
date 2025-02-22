# KeycloakClient

## v1

resource to define a Client within a [KeycloakRealm](./keycloakrealm.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clientSecret](#specclientsecret)|object||
|[spec.clientSecret.clientIdKey](#specclientsecretclientidkey)|string||
|[spec.clientSecret.clientSecretKey](#specclientsecretclientsecretkey)|string||
|[spec.clientSecret.secretName](#specclientsecretsecretname)|string|✅|
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.access](#specdefinitionaccess)|object||
|[spec.definition.adminUrl](#specdefinitionadminurl)|string||
|[spec.definition.alwaysDisplayInConsole](#specdefinitionalwaysdisplayinconsole)|boolean||
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.authenticationFlowBindingOverrides](#specdefinitionauthenticationflowbindingoverrides)|object||
|[spec.definition.authorizationServicesEnabled](#specdefinitionauthorizationservicesenabled)|boolean||
|[spec.definition.authorizationSettings](#specdefinitionauthorizationsettings)|object||
|[spec.definition.authorizationSettings.allowRemoteResourceManagement](#specdefinitionauthorizationsettingsallowremoteresourcemanagement)|boolean||
|[spec.definition.authorizationSettings.authorizationSchema](#specdefinitionauthorizationsettingsauthorizationschema)|object||
|[spec.definition.authorizationSettings.authorizationSchema.resourceTypes](#specdefinitionauthorizationsettingsauthorizationschemaresourcetypes)|object||
|[spec.definition.authorizationSettings.clientId](#specdefinitionauthorizationsettingsclientid)|string||
|[spec.definition.authorizationSettings.decisionStrategy](#specdefinitionauthorizationsettingsdecisionstrategy)|string||
|[spec.definition.authorizationSettings.id](#specdefinitionauthorizationsettingsid)|string||
|[spec.definition.authorizationSettings.name](#specdefinitionauthorizationsettingsname)|string||
|[spec.definition.authorizationSettings.policies[]](#specdefinitionauthorizationsettingspolicies)|object||
|[spec.definition.authorizationSettings.policies[].config](#specdefinitionauthorizationsettingspoliciesconfig)|object||
|[spec.definition.authorizationSettings.policies[].decisionStrategy](#specdefinitionauthorizationsettingspoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.policies[].description](#specdefinitionauthorizationsettingspoliciesdescription)|string||
|[spec.definition.authorizationSettings.policies[].id](#specdefinitionauthorizationsettingspoliciesid)|string||
|[spec.definition.authorizationSettings.policies[].logic](#specdefinitionauthorizationsettingspolicieslogic)|string||
|[spec.definition.authorizationSettings.policies[].name](#specdefinitionauthorizationsettingspoliciesname)|string||
|[spec.definition.authorizationSettings.policies[].owner](#specdefinitionauthorizationsettingspoliciesowner)|string||
|[spec.definition.authorizationSettings.policies[].policies[]](#specdefinitionauthorizationsettingspoliciespolicies)|string||
|[spec.definition.authorizationSettings.policies[].resourceType](#specdefinitionauthorizationsettingspoliciesresourcetype)|string||
|[spec.definition.authorizationSettings.policies[].resources[]](#specdefinitionauthorizationsettingspoliciesresources)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[]](#specdefinitionauthorizationsettingspoliciesresourcesdata)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[]._id](#specdefinitionauthorizationsettingspoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingspoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingspoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].name](#specdefinitionauthorizationsettingspoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner](#specdefinitionauthorizationsettingspoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingspoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingspoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].type](#specdefinitionauthorizationsettingspoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uri](#specdefinitionauthorizationsettingspoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.policies[].scopes[]](#specdefinitionauthorizationsettingspoliciesscopes)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[]](#specdefinitionauthorizationsettingspoliciesscopesdata)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdatadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].iconUri](#specdefinitionauthorizationsettingspoliciesscopesdataiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].id](#specdefinitionauthorizationsettingspoliciesscopesdataid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].name](#specdefinitionauthorizationsettingspoliciesscopesdataname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]](#specdefinitionauthorizationsettingspoliciesscopesdataresources)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].type](#specdefinitionauthorizationsettingspoliciestype)|string||
|[spec.definition.authorizationSettings.policyEnforcementMode](#specdefinitionauthorizationsettingspolicyenforcementmode)|string||
|[spec.definition.authorizationSettings.resources[]](#specdefinitionauthorizationsettingsresources)|object||
|[spec.definition.authorizationSettings.resources[]._id](#specdefinitionauthorizationsettingsresourcesid)|string||
|[spec.definition.authorizationSettings.resources[].attributes](#specdefinitionauthorizationsettingsresourcesattributes)|object||
|[spec.definition.authorizationSettings.resources[].displayName](#specdefinitionauthorizationsettingsresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.resources[].icon_uri](#specdefinitionauthorizationsettingsresourcesiconuri)|string||
|[spec.definition.authorizationSettings.resources[].name](#specdefinitionauthorizationsettingsresourcesname)|string||
|[spec.definition.authorizationSettings.resources[].owner](#specdefinitionauthorizationsettingsresourcesowner)|object||
|[spec.definition.authorizationSettings.resources[].owner.id](#specdefinitionauthorizationsettingsresourcesownerid)|string||
|[spec.definition.authorizationSettings.resources[].owner.name](#specdefinitionauthorizationsettingsresourcesownername)|string||
|[spec.definition.authorizationSettings.resources[].ownerManagedAccess](#specdefinitionauthorizationsettingsresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.resources[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopes)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].displayName](#specdefinitionauthorizationsettingsresourcesscopesdisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].id](#specdefinitionauthorizationsettingsresourcesscopesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].name](#specdefinitionauthorizationsettingsresourcesscopesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resourceType](#specdefinitionauthorizationsettingsresourcesscopespoliciesresourcetype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[]](#specdefinitionauthorizationsettingsresourcesscopesuma)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].displayName](#specdefinitionauthorizationsettingsresourcesscopesumadisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesumaiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].id](#specdefinitionauthorizationsettingsresourcesscopesumaid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].name](#specdefinitionauthorizationsettingsresourcesscopesumaname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resourceType](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresourcetype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].type](#specdefinitionauthorizationsettingsresourcestype)|string||
|[spec.definition.authorizationSettings.resources[].uri](#specdefinitionauthorizationsettingsresourcesuri)|string||
|[spec.definition.authorizationSettings.resources[].uris[]](#specdefinitionauthorizationsettingsresourcesuris)|string||
|[spec.definition.authorizationSettings.scopes[]](#specdefinitionauthorizationsettingsscopes)|object||
|[spec.definition.authorizationSettings.scopes[].displayName](#specdefinitionauthorizationsettingsscopesdisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].iconUri](#specdefinitionauthorizationsettingsscopesiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].id](#specdefinitionauthorizationsettingsscopesid)|string||
|[spec.definition.authorizationSettings.scopes[].name](#specdefinitionauthorizationsettingsscopesname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[]](#specdefinitionauthorizationsettingsscopespolicies)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].config](#specdefinitionauthorizationsettingsscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].description](#specdefinitionauthorizationsettingsscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].id](#specdefinitionauthorizationsettingsscopespoliciesid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].logic](#specdefinitionauthorizationsettingsscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].name](#specdefinitionauthorizationsettingsscopespoliciesname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].owner](#specdefinitionauthorizationsettingsscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourceType](#specdefinitionauthorizationsettingsscopespoliciesresourcetype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdata)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].type](#specdefinitionauthorizationsettingsscopespoliciestype)|string||
|[spec.definition.authorizationSettings.scopes[].resources[]](#specdefinitionauthorizationsettingsscopesresources)|object||
|[spec.definition.authorizationSettings.scopes[].resources[]._id](#specdefinitionauthorizationsettingsscopesresourcesid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].attributes](#specdefinitionauthorizationsettingsscopesresourcesattributes)|object||
|[spec.definition.authorizationSettings.scopes[].resources[].displayName](#specdefinitionauthorizationsettingsscopesresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].icon_uri](#specdefinitionauthorizationsettingsscopesresourcesiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].name](#specdefinitionauthorizationsettingsscopesresourcesname)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner](#specdefinitionauthorizationsettingsscopesresourcesowner)|object||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.id](#specdefinitionauthorizationsettingsscopesresourcesownerid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.name](#specdefinitionauthorizationsettingsscopesresourcesownername)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopesresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].resources[].type](#specdefinitionauthorizationsettingsscopesresourcestype)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uri](#specdefinitionauthorizationsettingsscopesresourcesuri)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uris[]](#specdefinitionauthorizationsettingsscopesresourcesuris)|string||
|[spec.definition.baseUrl](#specdefinitionbaseurl)|string||
|[spec.definition.bearerOnly](#specdefinitionbeareronly)|boolean||
|[spec.definition.clientAuthenticatorType](#specdefinitionclientauthenticatortype)|string||
|[spec.definition.clientId](#specdefinitionclientid)|string||
|[spec.definition.clientTemplate](#specdefinitionclienttemplate)|string||
|[spec.definition.consentRequired](#specdefinitionconsentrequired)|boolean||
|[spec.definition.defaultClientScopes[]](#specdefinitiondefaultclientscopes)|string||
|[spec.definition.defaultRoles[]](#specdefinitiondefaultroles)|string||
|[spec.definition.description](#specdefinitiondescription)|string||
|[spec.definition.directAccessGrantsEnabled](#specdefinitiondirectaccessgrantsenabled)|boolean||
|[spec.definition.directGrantsOnly](#specdefinitiondirectgrantsonly)|boolean||
|[spec.definition.enabled](#specdefinitionenabled)|boolean||
|[spec.definition.frontchannelLogout](#specdefinitionfrontchannellogout)|boolean||
|[spec.definition.fullScopeAllowed](#specdefinitionfullscopeallowed)|boolean||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.implicitFlowEnabled](#specdefinitionimplicitflowenabled)|boolean||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.definition.nodeReRegistrationTimeout](#specdefinitionnodereregistrationtimeout)|integer||
|[spec.definition.notBefore](#specdefinitionnotbefore)|integer||
|[spec.definition.optionalClientScopes[]](#specdefinitionoptionalclientscopes)|string||
|[spec.definition.origin](#specdefinitionorigin)|string||
|[spec.definition.protocol](#specdefinitionprotocol)|string||
|[spec.definition.protocolMappers[]](#specdefinitionprotocolmappers)|object||
|[spec.definition.protocolMappers[].config](#specdefinitionprotocolmappersconfig)|object||
|[spec.definition.protocolMappers[].consentRequired](#specdefinitionprotocolmappersconsentrequired)|boolean||
|[spec.definition.protocolMappers[].consentText](#specdefinitionprotocolmappersconsenttext)|string||
|[spec.definition.protocolMappers[].id](#specdefinitionprotocolmappersid)|string||
|[spec.definition.protocolMappers[].name](#specdefinitionprotocolmappersname)|string||
|[spec.definition.protocolMappers[].protocol](#specdefinitionprotocolmappersprotocol)|string||
|[spec.definition.protocolMappers[].protocolMapper](#specdefinitionprotocolmappersprotocolmapper)|string||
|[spec.definition.publicClient](#specdefinitionpublicclient)|boolean||
|[spec.definition.redirectUris[]](#specdefinitionredirecturis)|string||
|[spec.definition.registeredNodes](#specdefinitionregisterednodes)|object||
|[spec.definition.registrationAccessToken](#specdefinitionregistrationaccesstoken)|string||
|[spec.definition.rootUrl](#specdefinitionrooturl)|string||
|[spec.definition.secret](#specdefinitionsecret)|string||
|[spec.definition.serviceAccountsEnabled](#specdefinitionserviceaccountsenabled)|boolean||
|[spec.definition.standardFlowEnabled](#specdefinitionstandardflowenabled)|boolean||
|[spec.definition.surrogateAuthRequired](#specdefinitionsurrogateauthrequired)|boolean||
|[spec.definition.type](#specdefinitiontype)|string||
|[spec.definition.useTemplateConfig](#specdefinitionusetemplateconfig)|boolean||
|[spec.definition.useTemplateMappers](#specdefinitionusetemplatemappers)|boolean||
|[spec.definition.useTemplateScope](#specdefinitionusetemplatescope)|boolean||
|[spec.definition.webOrigins[]](#specdefinitionweborigins)|string||
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
|[clientSecret](#specclientsecret)|object||
|[definition](#specdefinition)|object|✅|
|[options](#specoptions)|object||
|[patchFrom](#specpatchfrom)|object||
|[realmRef](#specrealmref)|string|✅|

the KeycloakClient resource

---

### spec.clientSecret

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clientIdKey](#specclientsecretclientidkey)|string||
|[clientSecretKey](#specclientsecretclientsecretkey)|string||
|[secretName](#specclientsecretsecretname)|string|✅|

*missing*

---

### spec.clientSecret.clientIdKey

Type: string

*missing*

---

### spec.clientSecret.clientSecretKey

Type: string

*missing*

---

### spec.clientSecret.secretName

Type: string

*missing*

---

### spec.definition

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[access](#specdefinitionaccess)|object||
|[adminUrl](#specdefinitionadminurl)|string||
|[alwaysDisplayInConsole](#specdefinitionalwaysdisplayinconsole)|boolean||
|[attributes](#specdefinitionattributes)|object||
|[authenticationFlowBindingOverrides](#specdefinitionauthenticationflowbindingoverrides)|object||
|[authorizationServicesEnabled](#specdefinitionauthorizationservicesenabled)|boolean||
|[authorizationSettings](#specdefinitionauthorizationsettings)|object||
|[baseUrl](#specdefinitionbaseurl)|string||
|[bearerOnly](#specdefinitionbeareronly)|boolean||
|[clientAuthenticatorType](#specdefinitionclientauthenticatortype)|string||
|[clientId](#specdefinitionclientid)|string||
|[clientTemplate](#specdefinitionclienttemplate)|string||
|[consentRequired](#specdefinitionconsentrequired)|boolean||
|[defaultClientScopes[]](#specdefinitiondefaultclientscopes)|string||
|[defaultRoles[]](#specdefinitiondefaultroles)|string||
|[description](#specdefinitiondescription)|string||
|[directAccessGrantsEnabled](#specdefinitiondirectaccessgrantsenabled)|boolean||
|[directGrantsOnly](#specdefinitiondirectgrantsonly)|boolean||
|[enabled](#specdefinitionenabled)|boolean||
|[frontchannelLogout](#specdefinitionfrontchannellogout)|boolean||
|[fullScopeAllowed](#specdefinitionfullscopeallowed)|boolean||
|[id](#specdefinitionid)|string||
|[implicitFlowEnabled](#specdefinitionimplicitflowenabled)|boolean||
|[name](#specdefinitionname)|string||
|[nodeReRegistrationTimeout](#specdefinitionnodereregistrationtimeout)|integer||
|[notBefore](#specdefinitionnotbefore)|integer||
|[optionalClientScopes[]](#specdefinitionoptionalclientscopes)|string||
|[origin](#specdefinitionorigin)|string||
|[protocol](#specdefinitionprotocol)|string||
|[protocolMappers[]](#specdefinitionprotocolmappers)|object||
|[publicClient](#specdefinitionpublicclient)|boolean||
|[redirectUris[]](#specdefinitionredirecturis)|string||
|[registeredNodes](#specdefinitionregisterednodes)|object||
|[registrationAccessToken](#specdefinitionregistrationaccesstoken)|string||
|[rootUrl](#specdefinitionrooturl)|string||
|[secret](#specdefinitionsecret)|string||
|[serviceAccountsEnabled](#specdefinitionserviceaccountsenabled)|boolean||
|[standardFlowEnabled](#specdefinitionstandardflowenabled)|boolean||
|[surrogateAuthRequired](#specdefinitionsurrogateauthrequired)|boolean||
|[type](#specdefinitiontype)|string||
|[useTemplateConfig](#specdefinitionusetemplateconfig)|boolean||
|[useTemplateMappers](#specdefinitionusetemplatemappers)|boolean||
|[useTemplateScope](#specdefinitionusetemplatescope)|boolean||
|[webOrigins[]](#specdefinitionweborigins)|string||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

*missing*

---

### spec.definition.access

Type: object

*missing*

---

### spec.definition.adminUrl

Type: string

*missing*

---

### spec.definition.alwaysDisplayInConsole

Type: boolean

*missing*

---

### spec.definition.attributes

Type: object

*missing*

---

### spec.definition.authenticationFlowBindingOverrides

Type: object

*missing*

---

### spec.definition.authorizationServicesEnabled

Type: boolean

*missing*

---

### spec.definition.authorizationSettings

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[allowRemoteResourceManagement](#specdefinitionauthorizationsettingsallowremoteresourcemanagement)|boolean||
|[authorizationSchema](#specdefinitionauthorizationsettingsauthorizationschema)|object||
|[clientId](#specdefinitionauthorizationsettingsclientid)|string||
|[decisionStrategy](#specdefinitionauthorizationsettingsdecisionstrategy)|string||
|[id](#specdefinitionauthorizationsettingsid)|string||
|[name](#specdefinitionauthorizationsettingsname)|string||
|[policies[]](#specdefinitionauthorizationsettingspolicies)|object||
|[policyEnforcementMode](#specdefinitionauthorizationsettingspolicyenforcementmode)|string||
|[resources[]](#specdefinitionauthorizationsettingsresources)|object||
|[scopes[]](#specdefinitionauthorizationsettingsscopes)|object||

*missing*

---

### spec.definition.authorizationSettings.allowRemoteResourceManagement

Type: boolean

*missing*

---

### spec.definition.authorizationSettings.authorizationSchema

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[resourceTypes](#specdefinitionauthorizationsettingsauthorizationschemaresourcetypes)|object||

*missing*

---

### spec.definition.authorizationSettings.authorizationSchema.resourceTypes

Type: object

*missing*

---

### spec.definition.authorizationSettings.clientId

Type: string

*missing*

---

### spec.definition.authorizationSettings.decisionStrategy

Type: string

*missing*

---

### spec.definition.authorizationSettings.id

Type: string

*missing*

---

### spec.definition.authorizationSettings.name

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionauthorizationsettingspoliciesconfig)|object||
|[decisionStrategy](#specdefinitionauthorizationsettingspoliciesdecisionstrategy)|string||
|[description](#specdefinitionauthorizationsettingspoliciesdescription)|string||
|[id](#specdefinitionauthorizationsettingspoliciesid)|string||
|[logic](#specdefinitionauthorizationsettingspolicieslogic)|string||
|[name](#specdefinitionauthorizationsettingspoliciesname)|string||
|[owner](#specdefinitionauthorizationsettingspoliciesowner)|string||
|[policies[]](#specdefinitionauthorizationsettingspoliciespolicies)|string||
|[resourceType](#specdefinitionauthorizationsettingspoliciesresourcetype)|string||
|[resources[]](#specdefinitionauthorizationsettingspoliciesresources)|string||
|[resourcesData[]](#specdefinitionauthorizationsettingspoliciesresourcesdata)|object||
|[scopes[]](#specdefinitionauthorizationsettingspoliciesscopes)|string||
|[scopesData[]](#specdefinitionauthorizationsettingspoliciesscopesdata)|object||
|[type](#specdefinitionauthorizationsettingspoliciestype)|string||

*missing*

---

### spec.definition.authorizationSettings.policies[].config

Type: object

*missing*

---

### spec.definition.authorizationSettings.policies[].decisionStrategy

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].description

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].id

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].logic

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].owner

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].policies[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourceType

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resources[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[\_id](#specdefinitionauthorizationsettingspoliciesresourcesdataid)|string||
|[attributes](#specdefinitionauthorizationsettingspoliciesresourcesdataattributes)|object||
|[displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatadisplayname)|string||
|[icon_uri](#specdefinitionauthorizationsettingspoliciesresourcesdataiconuri)|string||
|[name](#specdefinitionauthorizationsettingspoliciesresourcesdataname)|string||
|[owner](#specdefinitionauthorizationsettingspoliciesresourcesdataowner)|object||
|[ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesresourcesdataownermanagedaccess)|boolean||
|[scopes[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|object||
|[scopesUma[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[type](#specdefinitionauthorizationsettingspoliciesresourcesdatatype)|string||
|[uri](#specdefinitionauthorizationsettingspoliciesresourcesdatauri)|string||
|[uris[]](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|string||

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[]._id

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].attributes

Type: object

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].displayName

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].icon_uri

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].owner

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specdefinitionauthorizationsettingspoliciesresourcesdataownerid)|string||
|[name](#specdefinitionauthorizationsettingspoliciesresourcesdataownername)|string||

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.id

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.name

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].type

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].uri

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].resourcesData[].uris[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].scopes[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[displayName](#specdefinitionauthorizationsettingspoliciesscopesdatadisplayname)|string||
|[iconUri](#specdefinitionauthorizationsettingspoliciesscopesdataiconuri)|string||
|[id](#specdefinitionauthorizationsettingspoliciesscopesdataid)|string||
|[name](#specdefinitionauthorizationsettingspoliciesscopesdataname)|string||
|[resources[]](#specdefinitionauthorizationsettingspoliciesscopesdataresources)|object||

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].displayName

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].iconUri

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].id

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[\_id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.policies[].type

Type: string

*missing*

---

### spec.definition.authorizationSettings.policyEnforcementMode

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[\_id](#specdefinitionauthorizationsettingsresourcesid)|string||
|[attributes](#specdefinitionauthorizationsettingsresourcesattributes)|object||
|[displayName](#specdefinitionauthorizationsettingsresourcesdisplayname)|string||
|[icon_uri](#specdefinitionauthorizationsettingsresourcesiconuri)|string||
|[name](#specdefinitionauthorizationsettingsresourcesname)|string||
|[owner](#specdefinitionauthorizationsettingsresourcesowner)|object||
|[ownerManagedAccess](#specdefinitionauthorizationsettingsresourcesownermanagedaccess)|boolean||
|[scopes[]](#specdefinitionauthorizationsettingsresourcesscopes)|object||
|[scopesUma[]](#specdefinitionauthorizationsettingsresourcesscopesuma)|object||
|[type](#specdefinitionauthorizationsettingsresourcestype)|string||
|[uri](#specdefinitionauthorizationsettingsresourcesuri)|string||
|[uris[]](#specdefinitionauthorizationsettingsresourcesuris)|string||

*missing*

---

### spec.definition.authorizationSettings.resources[]._id

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].attributes

Type: object

*missing*

---

### spec.definition.authorizationSettings.resources[].displayName

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].icon_uri

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].owner

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specdefinitionauthorizationsettingsresourcesownerid)|string||
|[name](#specdefinitionauthorizationsettingsresourcesownername)|string||

*missing*

---

### spec.definition.authorizationSettings.resources[].owner.id

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].owner.name

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].ownerManagedAccess

Type: boolean

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[displayName](#specdefinitionauthorizationsettingsresourcesscopesdisplayname)|string||
|[iconUri](#specdefinitionauthorizationsettingsresourcesscopesiconuri)|string||
|[id](#specdefinitionauthorizationsettingsresourcesscopesid)|string||
|[name](#specdefinitionauthorizationsettingsresourcesscopesname)|string||
|[policies[]](#specdefinitionauthorizationsettingsresourcesscopespolicies)|object||

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].displayName

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].iconUri

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].id

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[resourceType](#specdefinitionauthorizationsettingsresourcesscopespoliciesresourcetype)|string||
|[resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resourceType

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[displayName](#specdefinitionauthorizationsettingsresourcesscopesumadisplayname)|string||
|[iconUri](#specdefinitionauthorizationsettingsresourcesscopesumaiconuri)|string||
|[id](#specdefinitionauthorizationsettingsresourcesscopesumaid)|string||
|[name](#specdefinitionauthorizationsettingsresourcesscopesumaname)|string||
|[policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|object||

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].displayName

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].iconUri

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].id

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[resourceType](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresourcetype)|string||
|[resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resourceType

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].type

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].uri

Type: string

*missing*

---

### spec.definition.authorizationSettings.resources[].uris[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[displayName](#specdefinitionauthorizationsettingsscopesdisplayname)|string||
|[iconUri](#specdefinitionauthorizationsettingsscopesiconuri)|string||
|[id](#specdefinitionauthorizationsettingsscopesid)|string||
|[name](#specdefinitionauthorizationsettingsscopesname)|string||
|[policies[]](#specdefinitionauthorizationsettingsscopespolicies)|object||
|[resources[]](#specdefinitionauthorizationsettingsscopesresources)|object||

*missing*

---

### spec.definition.authorizationSettings.scopes[].displayName

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].iconUri

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].id

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionauthorizationsettingsscopespoliciesconfig)|object||
|[decisionStrategy](#specdefinitionauthorizationsettingsscopespoliciesdecisionstrategy)|string||
|[description](#specdefinitionauthorizationsettingsscopespoliciesdescription)|string||
|[id](#specdefinitionauthorizationsettingsscopespoliciesid)|string||
|[logic](#specdefinitionauthorizationsettingsscopespolicieslogic)|string||
|[name](#specdefinitionauthorizationsettingsscopespoliciesname)|string||
|[owner](#specdefinitionauthorizationsettingsscopespoliciesowner)|string||
|[policies[]](#specdefinitionauthorizationsettingsscopespoliciespolicies)|string||
|[resourceType](#specdefinitionauthorizationsettingsscopespoliciesresourcetype)|string||
|[resources[]](#specdefinitionauthorizationsettingsscopespoliciesresources)|string||
|[resourcesData[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdata)|object||
|[scopes[]](#specdefinitionauthorizationsettingsscopespoliciesscopes)|string||
|[type](#specdefinitionauthorizationsettingsscopespoliciestype)|string||

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].config

Type: object

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].description

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].id

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].logic

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].owner

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].policies[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resourceType

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resources[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[\_id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].scopes[]

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].type

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].resources[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[\_id](#specdefinitionauthorizationsettingsscopesresourcesid)|string||
|[attributes](#specdefinitionauthorizationsettingsscopesresourcesattributes)|object||
|[displayName](#specdefinitionauthorizationsettingsscopesresourcesdisplayname)|string||
|[icon_uri](#specdefinitionauthorizationsettingsscopesresourcesiconuri)|string||
|[name](#specdefinitionauthorizationsettingsscopesresourcesname)|string||
|[owner](#specdefinitionauthorizationsettingsscopesresourcesowner)|object||
|[ownerManagedAccess](#specdefinitionauthorizationsettingsscopesresourcesownermanagedaccess)|boolean||
|[type](#specdefinitionauthorizationsettingsscopesresourcestype)|string||
|[uri](#specdefinitionauthorizationsettingsscopesresourcesuri)|string||
|[uris[]](#specdefinitionauthorizationsettingsscopesresourcesuris)|string||

*missing*

---

### spec.definition.authorizationSettings.scopes[].resources[]._id

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].resources[].attributes

Type: object

*missing*

---

### spec.definition.authorizationSettings.scopes[].resources[].displayName

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].resources[].icon_uri

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].resources[].name

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].resources[].owner

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specdefinitionauthorizationsettingsscopesresourcesownerid)|string||
|[name](#specdefinitionauthorizationsettingsscopesresourcesownername)|string||

*missing*

---

### spec.definition.authorizationSettings.scopes[].resources[].owner.id

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].resources[].owner.name

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].resources[].ownerManagedAccess

Type: boolean

*missing*

---

### spec.definition.authorizationSettings.scopes[].resources[].type

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].resources[].uri

Type: string

*missing*

---

### spec.definition.authorizationSettings.scopes[].resources[].uris[]

Type: string

*missing*

---

### spec.definition.baseUrl

Type: string

*missing*

---

### spec.definition.bearerOnly

Type: boolean

*missing*

---

### spec.definition.clientAuthenticatorType

Type: string

*missing*

---

### spec.definition.clientId

Type: string

*missing*

---

### spec.definition.clientTemplate

Type: string

*missing*

---

### spec.definition.consentRequired

Type: boolean

*missing*

---

### spec.definition.defaultClientScopes[]

Type: string

*missing*

---

### spec.definition.defaultRoles[]

Type: string

*missing*

---

### spec.definition.description

Type: string

*missing*

---

### spec.definition.directAccessGrantsEnabled

Type: boolean

*missing*

---

### spec.definition.directGrantsOnly

Type: boolean

*missing*

---

### spec.definition.enabled

Type: boolean

*missing*

---

### spec.definition.frontchannelLogout

Type: boolean

*missing*

---

### spec.definition.fullScopeAllowed

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

### spec.definition.implicitFlowEnabled

Type: boolean

*missing*

---

### spec.definition.name

Type: string

*missing*

---

### spec.definition.nodeReRegistrationTimeout

Type: integer

*missing*

---

### spec.definition.notBefore

Type: integer

*missing*

---

### spec.definition.optionalClientScopes[]

Type: string

*missing*

---

### spec.definition.origin

Type: string

*missing*

---

### spec.definition.protocol

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

### spec.definition.publicClient

Type: boolean

*missing*

---

### spec.definition.redirectUris[]

Type: string

*missing*

---

### spec.definition.registeredNodes

Type: object

*missing*

---

### spec.definition.registrationAccessToken

Type: string

*missing*

---

### spec.definition.rootUrl

Type: string

*missing*

---

### spec.definition.secret

Type: string

*missing*

---

### spec.definition.serviceAccountsEnabled

Type: boolean

*missing*

---

### spec.definition.standardFlowEnabled

Type: boolean

*missing*

---

### spec.definition.surrogateAuthRequired

Type: boolean

*missing*

---

### spec.definition.type

Type: string

*missing*

---

### spec.definition.useTemplateConfig

Type: boolean

*missing*

---

### spec.definition.useTemplateMappers

Type: boolean

*missing*

---

### spec.definition.useTemplateScope

Type: boolean

*missing*

---

### spec.definition.webOrigins[]

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