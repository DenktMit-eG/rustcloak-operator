# ClusterKeycloakRealm

## v1beta1

resource to define an Realm within a [KeyclaokInstance](./keycloakinstance.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clusterInstanceRef](#specclusterinstanceref)|string||
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
|[spec.definition.adminPermissionsClient](#specdefinitionadminpermissionsclient)|object||
|[spec.definition.adminPermissionsClient.access](#specdefinitionadminpermissionsclientaccess)|object||
|[spec.definition.adminPermissionsClient.adminUrl](#specdefinitionadminpermissionsclientadminurl)|string||
|[spec.definition.adminPermissionsClient.alwaysDisplayInConsole](#specdefinitionadminpermissionsclientalwaysdisplayinconsole)|boolean||
|[spec.definition.adminPermissionsClient.attributes](#specdefinitionadminpermissionsclientattributes)|object||
|[spec.definition.adminPermissionsClient.authenticationFlowBindingOverrides](#specdefinitionadminpermissionsclientauthenticationflowbindingoverrides)|object||
|[spec.definition.adminPermissionsClient.authorizationServicesEnabled](#specdefinitionadminpermissionsclientauthorizationservicesenabled)|boolean||
|[spec.definition.adminPermissionsClient.authorizationSettings](#specdefinitionadminpermissionsclientauthorizationsettings)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.allowRemoteResourceManagement](#specdefinitionadminpermissionsclientauthorizationsettingsallowremoteresourcemanagement)|boolean||
|[spec.definition.adminPermissionsClient.authorizationSettings.authorizationSchema](#specdefinitionadminpermissionsclientauthorizationsettingsauthorizationschema)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.authorizationSchema.resourceTypes](#specdefinitionadminpermissionsclientauthorizationsettingsauthorizationschemaresourcetypes)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.clientId](#specdefinitionadminpermissionsclientauthorizationsettingsclientid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.decisionStrategy](#specdefinitionadminpermissionsclientauthorizationsettingsdecisionstrategy)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.id](#specdefinitionadminpermissionsclientauthorizationsettingsid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.name](#specdefinitionadminpermissionsclientauthorizationsettingsname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[]](#specdefinitionadminpermissionsclientauthorizationsettingspolicies)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].config](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesconfig)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].decisionStrategy](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesdecisionstrategy)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].description](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesdescription)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].logic](#specdefinitionadminpermissionsclientauthorizationsettingspolicieslogic)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].owner](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesowner)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].policies[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciespolicies)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourceType](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcetype)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resources[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresources)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdata)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[]._id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].attributes](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataattributes)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].displayName](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatadisplayname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].icon_uri](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataiconuri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].owner](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataowner)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].owner.id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataownerid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].owner.name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataownername)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].ownerManagedAccess](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopes[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopes)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopesUma[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].type](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatatype)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].uri](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatauri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].uris[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatauris)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopes[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopes)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdata)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].displayName](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdatadisplayname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].iconUri](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataiconuri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresources)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policies[].type](#specdefinitionadminpermissionsclientauthorizationsettingspoliciestype)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.policyEnforcementMode](#specdefinitionadminpermissionsclientauthorizationsettingspolicyenforcementmode)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[]](#specdefinitionadminpermissionsclientauthorizationsettingsresources)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[]._id](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].attributes](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesattributes)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].displayName](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesdisplayname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].icon_uri](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesiconuri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].name](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].owner](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesowner)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].owner.id](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesownerid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].owner.name](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesownername)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].ownerManagedAccess](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesownermanagedaccess)|boolean||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopes)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].displayName](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesdisplayname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].iconUri](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesiconuri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].id](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].name](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespolicies)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].resourceType](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesresourcetype)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesuma)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].displayName](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumadisplayname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].iconUri](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumaiconuri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].id](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumaid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].name](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumaname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapolicies)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].resourceType](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesresourcetype)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].type](#specdefinitionadminpermissionsclientauthorizationsettingsresourcestype)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].uri](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesuri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.resources[].uris[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesuris)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopes)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].displayName](#specdefinitionadminpermissionsclientauthorizationsettingsscopesdisplayname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].iconUri](#specdefinitionadminpermissionsclientauthorizationsettingsscopesiconuri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].id](#specdefinitionadminpermissionsclientauthorizationsettingsscopesid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].name](#specdefinitionadminpermissionsclientauthorizationsettingsscopesname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopespolicies)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].config](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesconfig)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].decisionStrategy](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesdecisionstrategy)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].description](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesdescription)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].id](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].logic](#specdefinitionadminpermissionsclientauthorizationsettingsscopespolicieslogic)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].name](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].owner](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesowner)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].policies[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciespolicies)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourceType](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcetype)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resources[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresources)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdata)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].scopes[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesscopes)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].type](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciestype)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresources)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[]._id](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].attributes](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesattributes)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].displayName](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesdisplayname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].icon_uri](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesiconuri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].name](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesname)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].owner](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesowner)|object||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].owner.id](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesownerid)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].owner.name](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesownername)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].ownerManagedAccess](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesownermanagedaccess)|boolean||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].type](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcestype)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].uri](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesuri)|string||
|[spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].uris[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesuris)|string||
|[spec.definition.adminPermissionsClient.baseUrl](#specdefinitionadminpermissionsclientbaseurl)|string||
|[spec.definition.adminPermissionsClient.bearerOnly](#specdefinitionadminpermissionsclientbeareronly)|boolean||
|[spec.definition.adminPermissionsClient.clientAuthenticatorType](#specdefinitionadminpermissionsclientclientauthenticatortype)|string||
|[spec.definition.adminPermissionsClient.clientId](#specdefinitionadminpermissionsclientclientid)|string||
|[spec.definition.adminPermissionsClient.clientTemplate](#specdefinitionadminpermissionsclientclienttemplate)|string||
|[spec.definition.adminPermissionsClient.consentRequired](#specdefinitionadminpermissionsclientconsentrequired)|boolean||
|[spec.definition.adminPermissionsClient.defaultClientScopes[]](#specdefinitionadminpermissionsclientdefaultclientscopes)|string||
|[spec.definition.adminPermissionsClient.defaultRoles[]](#specdefinitionadminpermissionsclientdefaultroles)|string||
|[spec.definition.adminPermissionsClient.description](#specdefinitionadminpermissionsclientdescription)|string||
|[spec.definition.adminPermissionsClient.directAccessGrantsEnabled](#specdefinitionadminpermissionsclientdirectaccessgrantsenabled)|boolean||
|[spec.definition.adminPermissionsClient.directGrantsOnly](#specdefinitionadminpermissionsclientdirectgrantsonly)|boolean||
|[spec.definition.adminPermissionsClient.enabled](#specdefinitionadminpermissionsclientenabled)|boolean||
|[spec.definition.adminPermissionsClient.frontchannelLogout](#specdefinitionadminpermissionsclientfrontchannellogout)|boolean||
|[spec.definition.adminPermissionsClient.fullScopeAllowed](#specdefinitionadminpermissionsclientfullscopeallowed)|boolean||
|[spec.definition.adminPermissionsClient.id](#specdefinitionadminpermissionsclientid)|string||
|[spec.definition.adminPermissionsClient.implicitFlowEnabled](#specdefinitionadminpermissionsclientimplicitflowenabled)|boolean||
|[spec.definition.adminPermissionsClient.name](#specdefinitionadminpermissionsclientname)|string||
|[spec.definition.adminPermissionsClient.nodeReRegistrationTimeout](#specdefinitionadminpermissionsclientnodereregistrationtimeout)|integer||
|[spec.definition.adminPermissionsClient.notBefore](#specdefinitionadminpermissionsclientnotbefore)|integer||
|[spec.definition.adminPermissionsClient.optionalClientScopes[]](#specdefinitionadminpermissionsclientoptionalclientscopes)|string||
|[spec.definition.adminPermissionsClient.origin](#specdefinitionadminpermissionsclientorigin)|string||
|[spec.definition.adminPermissionsClient.protocol](#specdefinitionadminpermissionsclientprotocol)|string||
|[spec.definition.adminPermissionsClient.protocolMappers[]](#specdefinitionadminpermissionsclientprotocolmappers)|object||
|[spec.definition.adminPermissionsClient.protocolMappers[].config](#specdefinitionadminpermissionsclientprotocolmappersconfig)|object||
|[spec.definition.adminPermissionsClient.protocolMappers[].consentRequired](#specdefinitionadminpermissionsclientprotocolmappersconsentrequired)|boolean||
|[spec.definition.adminPermissionsClient.protocolMappers[].consentText](#specdefinitionadminpermissionsclientprotocolmappersconsenttext)|string||
|[spec.definition.adminPermissionsClient.protocolMappers[].id](#specdefinitionadminpermissionsclientprotocolmappersid)|string||
|[spec.definition.adminPermissionsClient.protocolMappers[].name](#specdefinitionadminpermissionsclientprotocolmappersname)|string||
|[spec.definition.adminPermissionsClient.protocolMappers[].protocol](#specdefinitionadminpermissionsclientprotocolmappersprotocol)|string||
|[spec.definition.adminPermissionsClient.protocolMappers[].protocolMapper](#specdefinitionadminpermissionsclientprotocolmappersprotocolmapper)|string||
|[spec.definition.adminPermissionsClient.publicClient](#specdefinitionadminpermissionsclientpublicclient)|boolean||
|[spec.definition.adminPermissionsClient.redirectUris[]](#specdefinitionadminpermissionsclientredirecturis)|string||
|[spec.definition.adminPermissionsClient.registeredNodes](#specdefinitionadminpermissionsclientregisterednodes)|object||
|[spec.definition.adminPermissionsClient.registrationAccessToken](#specdefinitionadminpermissionsclientregistrationaccesstoken)|string||
|[spec.definition.adminPermissionsClient.rootUrl](#specdefinitionadminpermissionsclientrooturl)|string||
|[spec.definition.adminPermissionsClient.secret](#specdefinitionadminpermissionsclientsecret)|string||
|[spec.definition.adminPermissionsClient.serviceAccountsEnabled](#specdefinitionadminpermissionsclientserviceaccountsenabled)|boolean||
|[spec.definition.adminPermissionsClient.standardFlowEnabled](#specdefinitionadminpermissionsclientstandardflowenabled)|boolean||
|[spec.definition.adminPermissionsClient.surrogateAuthRequired](#specdefinitionadminpermissionsclientsurrogateauthrequired)|boolean||
|[spec.definition.adminPermissionsClient.type](#specdefinitionadminpermissionsclienttype)|string||
|[spec.definition.adminPermissionsClient.useTemplateConfig](#specdefinitionadminpermissionsclientusetemplateconfig)|boolean||
|[spec.definition.adminPermissionsClient.useTemplateMappers](#specdefinitionadminpermissionsclientusetemplatemappers)|boolean||
|[spec.definition.adminPermissionsClient.useTemplateScope](#specdefinitionadminpermissionsclientusetemplatescope)|boolean||
|[spec.definition.adminPermissionsClient.webOrigins[]](#specdefinitionadminpermissionsclientweborigins)|string||
|[spec.definition.adminPermissionsEnabled](#specdefinitionadminpermissionsenabled)|boolean||
|[spec.definition.adminTheme](#specdefinitionadmintheme)|string||
|[spec.definition.applicationScopeMappings](#specdefinitionapplicationscopemappings)|object||
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.browserFlow](#specdefinitionbrowserflow)|string||
|[spec.definition.browserSecurityHeaders](#specdefinitionbrowsersecurityheaders)|object||
|[spec.definition.browserSecurityHeaders.contentSecurityPolicy](#specdefinitionbrowsersecurityheaderscontentsecuritypolicy)|string||
|[spec.definition.browserSecurityHeaders.contentSecurityPolicyReportOnly](#specdefinitionbrowsersecurityheaderscontentsecuritypolicyreportonly)|string||
|[spec.definition.browserSecurityHeaders.strictTransportSecurity](#specdefinitionbrowsersecurityheadersstricttransportsecurity)|string||
|[spec.definition.browserSecurityHeaders.xContentTypeOptions](#specdefinitionbrowsersecurityheadersxcontenttypeoptions)|string||
|[spec.definition.browserSecurityHeaders.xFrameOptions](#specdefinitionbrowsersecurityheadersxframeoptions)|string||
|[spec.definition.browserSecurityHeaders.xRobotsTag](#specdefinitionbrowsersecurityheadersxrobotstag)|string||
|[spec.definition.browserSecurityHeaders.xXSSProtection](#specdefinitionbrowsersecurityheadersxxssprotection)|string||
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
|[spec.definition.publicKey](#specdefinitionpublickey)|string||
|[spec.definition.quickLoginCheckMilliSeconds](#specdefinitionquicklogincheckmilliseconds)|integer||
|[spec.definition.realm](#specdefinitionrealm)|string||
|[spec.definition.realmCacheEnabled](#specdefinitionrealmcacheenabled)|boolean||
|[spec.definition.refreshTokenMaxReuse](#specdefinitionrefreshtokenmaxreuse)|integer||
|[spec.definition.registrationAllowed](#specdefinitionregistrationallowed)|boolean||
|[spec.definition.registrationEmailAsUsername](#specdefinitionregistrationemailasusername)|boolean||
|[spec.definition.registrationFlow](#specdefinitionregistrationflow)|string||
|[spec.definition.rememberMe](#specdefinitionrememberme)|boolean||
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
|[spec.instanceRef](#specinstanceref)|string||
|[spec.patchFrom[]](#specpatchfrom)|object||
|[spec.patchFrom[].configMapKeyRef](#specpatchfromconfigmapkeyref)|object||
|[spec.patchFrom[].configMapKeyRef.key](#specpatchfromconfigmapkeyrefkey)|string|✅|
|[spec.patchFrom[].configMapKeyRef.name](#specpatchfromconfigmapkeyrefname)|string|✅|
|[spec.patchFrom[].configMapKeyRef.optional](#specpatchfromconfigmapkeyrefoptional)|boolean||
|[spec.patchFrom[].path](#specpatchfrompath)|string|✅|
|[spec.patchFrom[].secretKeyRef](#specpatchfromsecretkeyref)|object||
|[spec.patchFrom[].secretKeyRef.key](#specpatchfromsecretkeyrefkey)|string|✅|
|[spec.patchFrom[].secretKeyRef.name](#specpatchfromsecretkeyrefname)|string|✅|
|[spec.patchFrom[].secretKeyRef.optional](#specpatchfromsecretkeyrefoptional)|boolean||
|[spec.patchFrom[].value](#specpatchfromvalue)|string||
|[spec.patchFrom[].value_as](#specpatchfromvalueas)|string||
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
|[clusterInstanceRef](#specclusterinstanceref)|string||
|[definition](#specdefinition)|object|✅|
|[instanceRef](#specinstanceref)|string||
|[patchFrom[]](#specpatchfrom)|object||

the KeycloakRealm resource

---

### spec.clusterInstanceRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the cluster instance to which this object belongs to.

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
|[adminPermissionsClient](#specdefinitionadminpermissionsclient)|object||
|[adminPermissionsEnabled](#specdefinitionadminpermissionsenabled)|boolean||
|[adminTheme](#specdefinitionadmintheme)|string||
|[applicationScopeMappings](#specdefinitionapplicationscopemappings)|object||
|[attributes](#specdefinitionattributes)|object||
|[browserFlow](#specdefinitionbrowserflow)|string||
|[browserSecurityHeaders](#specdefinitionbrowsersecurityheaders)|object||
|[bruteForceProtected](#specdefinitionbruteforceprotected)|boolean||
|[bruteForceStrategy](#specdefinitionbruteforcestrategy)|string||
|[certificate](#specdefinitioncertificate)|string||
|[clientAuthenticationFlow](#specdefinitionclientauthenticationflow)|string||
|[clientOfflineSessionIdleTimeout](#specdefinitionclientofflinesessionidletimeout)|integer||
|[clientOfflineSessionMaxLifespan](#specdefinitionclientofflinesessionmaxlifespan)|integer||
|[clientPolicies](#specdefinitionclientpolicies)|object||
|[clientProfiles](#specdefinitionclientprofiles)|object||
|[clientScopeMappings](#specdefinitionclientscopemappings)|object||
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
|[publicKey](#specdefinitionpublickey)|string||
|[quickLoginCheckMilliSeconds](#specdefinitionquicklogincheckmilliseconds)|integer||
|[realm](#specdefinitionrealm)|string||
|[realmCacheEnabled](#specdefinitionrealmcacheenabled)|boolean||
|[refreshTokenMaxReuse](#specdefinitionrefreshtokenmaxreuse)|integer||
|[registrationAllowed](#specdefinitionregistrationallowed)|boolean||
|[registrationEmailAsUsername](#specdefinitionregistrationemailasusername)|boolean||
|[registrationFlow](#specdefinitionregistrationflow)|string||
|[rememberMe](#specdefinitionrememberme)|boolean||
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

```json { "type": "object", "properties": { "accessCodeLifespan": { "title": "Client Login Timeout", "description": "Max time a client has to finish the access token protocol. This should normally be 1 minute.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "accessCodeLifespanLogin": { "title": "Login timeout", "description": "Max time a user has to complete a login. This is recommended to be relatively long, such as 30 minutes or more.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "accessCodeLifespanUserAction": { "title": "Login action timeout", "description": "Max time a user has to complete login related actions like update password or configure totp. This is recommended to be relatively long, such as 5 minutes or more.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "accessTokenLifespan": { "title": "Access Token Lifespan", "description": "Max time before an access token is expired. This value is recommended to be short relative to the SSO timeout.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "accessTokenLifespanForImplicitFlow": { "title": "Access Token Lifespan For Implicit Flow", "description": "Max time before an access token issued during OpenID Connect Implicit Flow is expired. This value is recommended to be shorter than the SSO timeout. There is no possibility to refresh token during implicit flow, that's why there is a separate timeout different to 'Access Token Lifespan'.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "accountTheme": { "title": "Account theme", "description": "Select theme for login, OTP, grant, registration and forgot password pages.", "type": "string" }, "actionTokenGeneratedByAdminLifespan": { "title": "Default Admin-Initiated Action Lifespan", "description": "Maximum time before an action permit sent to a user by administrator is expired. This value is recommended to be long to allow administrators to send e-mails for users that are currently offline. The default timeout can be overridden immediately before issuing the token.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "actionTokenGeneratedByUserLifespan": { "title": "User-Initiated Action Lifespan", "description": "Maximum time before an action permit sent by a user (such as a forgot password e-mail) is expired. This value is recommended to be short because it's expected that the user would react to self-created action quickly.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "adminEventsDetailsEnabled": { "title": "Include representation", "description": "Include JSON representation for create and update requests.", "type": "boolean" }, "adminEventsEnabled": { "title": "Save events", "description": "If enabled, admin events are saved to the database, which makes events available to the Admin UI.", "type": "boolean" }, "adminPermissionsClient": { "$ref": "#/$defs/ClientRepresentation" }, "adminPermissionsEnabled": { "type": "boolean" }, "adminTheme": { "title": "Admin theme", "type": "string" }, "applicationScopeMappings": { "type": "object", "additionalProperties": { "type": "array", "items": { "$ref": "#/$defs/ScopeMappingRepresentation" } } }, "applications": { "type": "array", "items": { "$ref": "#/$defs/ApplicationRepresentation" } }, "attributes": { "type": "object", "additionalProperties": { "type": "string" } }, "authenticationFlows": { "type": "array", "items": { "$ref": "#/$defs/AuthenticationFlowRepresentation" } }, "authenticatorConfig": { "type": "array", "items": { "$ref": "#/$defs/AuthenticatorConfigRepresentation" } }, "browserFlow": { "type": "string" }, "browserSecurityHeaders": { "type": "object", "properties": { "contentSecurityPolicy": { "title": "Content-Security-Policy", "description": "Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>", "type": "string" }, "contentSecurityPolicyReportOnly": { "title": "Content-Security-Policy-Report-Only", "description": "For testing Content Security Policies <1>Learn more</1>", "type": "string" }, "strictTransportSecurity": { "title": "HTTP Strict Transport Security (HSTS)", "description": "The Strict-Transport-Security HTTP header tells browsers to always use HTTPS. Once a browser sees this header, it will only visit the site over HTTPS for the time specified (1 year) at max-age, including the subdomains. <1>Learn more</1>", "type": "string" }, "xContentTypeOptions": { "title": "X-Content-Type-Options", "description": "The default value prevents Internet Explorer and Google Chrome from MIME-sniffing a response away from the declared content-type. <1>Learn more</1>", "type": "string" }, "xFrameOptions": { "title": "X-Frame-Options", "description": "Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>", "type": "string" }, "xRobotsTag": { "title": "X-Robots-Tag", "description": "Prevent pages from appearing in search engines. <1>Learn more</1>", "type": "string" }, "xXSSProtection": { "title": "X-XSS-Protection", "description": "This header configures the Cross-site scripting (XSS) filter in your browser. Using the default behaviour, the browser will prevent rendering of the page when a XSS attack is detected. <1>Learn more</1>", "type": "string" } } }, "bruteForceProtected": { "type": "boolean" }, "bruteForceStrategy": { "title": "Strategy to increase wait time", "description": "Multiple means wait time will be increased only when number of failures are multiples of '{{failureFactor}}'. Linear means each new failure starting at '{{failureFactor}}' will increase wait time.", "$ref": "#/$defs/BruteForceStrategy" }, "certificate": { "type": "string" }, "clientAuthenticationFlow": { "type": "string" }, "clientOfflineSessionIdleTimeout": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "clientOfflineSessionMaxLifespan": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "clientPolicies": { "$ref": "#/$defs/ClientPoliciesRepresentation" }, "clientProfiles": { "$ref": "#/$defs/ClientProfilesRepresentation" }, "clientScopeMappings": { "type": "object", "additionalProperties": { "type": "array", "items": { "$ref": "#/$defs/ScopeMappingRepresentation" } } }, "clientScopes": { "type": "array", "items": { "$ref": "#/$defs/ClientScopeRepresentation" } }, "clientSessionIdleTimeout": { "title": "Client Session Idle", "description": "Time a client session is allowed to be idle before it expires. Tokens are invalidated when a client session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Idle value.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "clientSessionMaxLifespan": { "title": "Client Session Max", "description": "Max time before a client session is expired. Tokens are invalidated when a session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Max value.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "clientTemplates": { "type": "array", "items": { "$ref": "#/$defs/ClientTemplateRepresentation" } }, "clients": { "type": "array", "items": { "$ref": "#/$defs/ClientRepresentation" } }, "codeSecret": { "type": "string" }, "components": { "$ref": "#/$defs/MultivaluedHashMapStringComponentExportRepresentation" }, "defaultDefaultClientScopes": { "type": "array", "items": { "type": "string" } }, "defaultGroups": { "type": "array", "items": { "type": "string" } }, "defaultLocale": { "title": "Default locale", "type": "string" }, "defaultOptionalClientScopes": { "type": "array", "items": { "type": "string" } }, "defaultRole": { "$ref": "#/$defs/RoleRepresentation" }, "defaultRoles": { "type": "array", "items": { "type": "string" } }, "defaultSignatureAlgorithm": { "title": "Default Signature Algorithm", "description": "Default algorithm used to sign tokens for the realm", "type": "string", "enum": [ "EdDSA", "ES256", "ES384", "ES512", "HS256", "HS384", "HS512", "PS256", "PS384", "PS512", "RS256", "RS384", "RS512" ] }, "directGrantFlow": { "type": "string" }, "displayName": { "title": "Display name", "type": "string" }, "displayNameHtml": { "title": "HTML Display name", "type": "string" }, "dockerAuthenticationFlow": { "type": "string" }, "duplicateEmailsAllowed": { "title": "Duplicate emails", "description": "Allow multiple users to have the same email address. Changing this setting will also clear the user's cache. It is recommended to manually update email constraints of existing users in the database after switching off support for duplicate email addresses.", "type": "boolean" }, "editUsernameAllowed": { "title": "Edit username", "description": "If enabled, the username field is editable, readonly otherwise.", "type": "boolean" }, "emailTheme": { "title": "Email theme", "description": "Select a theme for emails that are sent by the server.", "type": "string" }, "enabled": { "type": "boolean" }, "enabledEventTypes": { "type": "array", "items": { "type": "string" } }, "eventsEnabled": { "title": "Save events", "description": "If enabled, user events are saved to the database, which makes events available to the admin and account management UIs.", "type": "boolean" }, "eventsExpiration": { "title": "Expiration", "description": "Sets the expiration for events. Expired events are periodically deleted from the database.", "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "eventsListeners": { "title": "Event listeners", "description": "Configure what listeners receive events for the realm.", "type": "array", "items": { "type": "string" } }, "failureFactor": { "title": "Max login failures", "description": "Max login failures", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "federatedUsers": { "type": "array", "items": { "$ref": "#/$defs/UserRepresentation" } }, "firstBrokerLoginFlow": { "type": "string" }, "groups": { "type": "array", "items": { "$ref": "#/$defs/GroupRepresentation" } }, "id": { "type": "string" }, "identityProviderMappers": { "type": "array", "items": { "$ref": "#/$defs/IdentityProviderMapperRepresentation" } }, "identityProviders": { "type": "array", "items": { "$ref": "#/$defs/IdentityProviderRepresentation" } }, "internationalizationEnabled": { "title": "Internationalization", "description": "If enabled, you can choose which locales you support for this realm and which locale is the default.", "type": "boolean" }, "keycloakVersion": { "type": "string" }, "localizationTexts": { "type": "object", "additionalProperties": { "type": "object", "additionalProperties": { "type": "string" } } }, "loginTheme": { "title": "Login theme", "description": "Select theme for login, OTP, grant, registration and forgot password pages.", "type": "string" }, "loginWithEmailAllowed": { "title": "Login with email", "description": "Allow users to log in with their email address.", "type": "boolean" }, "maxDeltaTimeSeconds": { "title": "Failure reset time", "description": "When will failure count be reset?", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "maxFailureWaitSeconds": { "title": "Max wait", "description": "Max time a user will be locked out.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "maxTemporaryLockouts": { "title": "Maximum temporary lockouts", "description": "The number of temporary lockouts permitted before the user is permanently locked out.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "minimumQuickLoginWaitSeconds": { "title": "Minimum quick login wait", "description": "How long to wait after a quick login failure.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "notBefore": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "oAuth2DeviceCodeLifespan": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "oAuth2DevicePollingInterval": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "oauth2DeviceCodeLifespan": { "title": "OAuth 2.0 Device Code Lifespan", "description": "Max time before the device code and user code are expired. This value needs to be a long enough lifetime to be usable (allowing the user to retrieve their secondary device, navigate to the verification URI, login, etc.), but should be sufficiently short to limit the usability of a code obtained for phishing.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "oauth2DevicePollingInterval": { "title": "OAuth 2.0 Device Polling Interval", "description": "The minimum amount of time in seconds that the client should wait between polling requests to the token endpoint.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "oauthClients": { "type": "array", "items": { "$ref": "#/$defs/OAuthClientRepresentation" } }, "offlineSessionIdleTimeout": { "title": "Offline Session Idle", "description": "Time an offline session is allowed to be idle before it expires. You need to use offline token to refresh at least once within this period; otherwise offline session will expire.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "offlineSessionMaxLifespan": { "title": "Offline Session Max", "description": "Max time before an offline session is expired regardless of activity.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "offlineSessionMaxLifespanEnabled": { "title": "Offline Session Max Limited", "description": "Enable offline session maximum lifetime", "type": "boolean" }, "organizations": { "type": "array", "items": { "$ref": "#/$defs/OrganizationRepresentation" } }, "organizationsEnabled": { "type": "boolean" }, "otpPolicyAlgorithm": { "title": "OTP hash algorithm", "description": "What hashing algorithm should be used to generate the OTP.", "type": "string" }, "otpPolicyCodeReusable": { "title": "Reusable token", "description": "Possibility to use the same OTP code again after successful authentication.", "type": "boolean" }, "otpPolicyDigits": { "title": "Number of digits", "description": "How many digits should the OTP have?", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "otpPolicyInitialCounter": { "title": "Initial counter", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "otpPolicyLookAheadWindow": { "title": "Look around window", "description": "How far around (extra token periods or counts) should the server look just in case the token generator and server are out of time sync or counter sync?", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "otpPolicyPeriod": { "title": "OTP Token period", "description": "How many seconds should an OTP token be valid? Defaults to 30 seconds.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "otpPolicyType": { "title": "OTP type", "description": "totp is Time-Based One Time Password. 'hotp' is a counter base one time password in which the server keeps a counter to hash against.", "type": "string", "enum": [ "totp", "hotp" ] }, "otpSupportedApplications": { "type": "array", "items": { "type": "string" } }, "passwordCredentialGrantAllowed": { "type": "boolean" }, "passwordPolicy": { "type": "string" }, "permanentLockout": { "title": "Permanent lockout", "type": "boolean" }, "privateKey": { "type": "string" }, "protocolMappers": { "type": "array", "items": { "$ref": "#/$defs/ProtocolMapperRepresentation" } }, "publicKey": { "type": "string" }, "quickLoginCheckMilliSeconds": { "title": "Quick login check milliseconds", "description": "If a failure happens concurrently too quickly, lock out the user.", "type": "integer", "format": "int64", "maximum": 9.223372036854776e18, "minimum": -9.223372036854776e18 }, "realm": { "title": "Realm ID", "type": "string" }, "realmCacheEnabled": { "type": "boolean" }, "refreshTokenMaxReuse": { "title": "Refresh Token Max Reuse", "description": "Maximum number of times a refresh token can be reused. When a different token is used, revocation is immediate.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "registrationAllowed": { "title": "User registration", "description": "Enable/disable the registration page. A link for registration will show on login page too.", "type": "boolean" }, "registrationEmailAsUsername": { "title": "Email as username", "description": "Allow users to set email as username.", "type": "boolean" }, "registrationFlow": { "type": "string" }, "rememberMe": { "title": "Remember me", "description": "Show checkbox on login page to allow user to remain logged in between browser restarts until session expires.", "type": "boolean" }, "requiredActions": { "type": "array", "items": { "$ref": "#/$defs/RequiredActionProviderRepresentation" } }, "requiredCredentials": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resetCredentialsFlow": { "type": "string" }, "resetPasswordAllowed": { "title": "Specifies independent timeout for forgot password.", "description": "Show a link on login page for user to click when they have forgotten their credentials.", "type": "boolean" }, "revokeRefreshToken": { "title": "Revoke Refresh Token", "description": "If enabled a refresh token can only be used up to 'Refresh Token Max Reuse' and is revoked when a different token is used. Otherwise refresh tokens are not revoked when used and can be used multiple times.", "type": "boolean" }, "roles": { "$ref": "#/$defs/RolesRepresentation" }, "scopeMappings": { "type": "array", "items": { "$ref": "#/$defs/ScopeMappingRepresentation" } }, "smtpServer": { "type": "object", "properties": { "auth": { "title": "Authentication", "type": "string", "enum": [ "true", "false", "" ] }, "envelopeFrom": { "title": "Envelope from", "description": "An email address used for bounces (optional).", "type": "string" }, "from": { "title": "From", "type": "string" }, "fromDisplayName": { "title": "From display name", "description": "A user-friendly name for the 'From' address (optional).", "type": "string" }, "host": { "title": "Host", "type": "string" }, "password": { "title": "Password", "description": "SMTP password. This field is able to obtain its value from vault, use ${vault.ID} format.", "type": "string" }, "port": { "title": "Port", "type": "string" }, "replyTo": { "title": "Reply to", "type": "string" }, "replyToDisplayName": { "title": "Reply to display name", "description": "A user-friendly name for the 'Reply-To' address (optional).", "type": "string" }, "ssl": { "title": "Enable SSL", "type": "string", "enum": [ "true", "false", "" ] }, "starttls": { "title": "Enable StartTLS", "type": "string", "enum": [ "true", "false", "" ] }, "user": { "title": "Username", "type": "string" } } }, "social": { "type": "boolean" }, "socialProviders": { "type": "object", "additionalProperties": { "type": "string" } }, "sslRequired": { "title": "Require SSL", "description": "Is HTTPS required? 'None' means HTTPS is not required for any client IP address. 'External requests' means localhost and private IP addresses can access without HTTPS. 'All requests' means HTTPS is required for all IP addresses.", "type": "string", "enum": [ "all", "external", "none" ] }, "ssoSessionIdleTimeout": { "title": "SSO Session Idle", "description": "Time a session is allowed to be idle before it expires. Tokens and browser sessions are invalidated when a session is expired.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "ssoSessionIdleTimeoutRememberMe": { "title": "SSO Session Idle Remember Me", "description": "Time a remember me session is allowed to be idle before it expires. Tokens and browser sessions are invalidated when a session is expired. If not set it uses the standard SSO Session Idle value.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "ssoSessionMaxLifespan": { "title": "SSO Session Max", "description": "Max time before a session is expired. Tokens and browser sessions are invalidated when a session is expired.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "ssoSessionMaxLifespanRememberMe": { "title": "SSO Session Max Remember Me", "description": "Max time before a session is expired when a user has set the remember me option. Tokens and browser sessions are invalidated when a session is expired. If not set it uses the standard SSO Session Max value.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "supportedLocales": { "title": "Supported locales", "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "updateProfileOnInitialSocialLogin": { "type": "boolean" }, "userCacheEnabled": { "type": "boolean" }, "userFederationMappers": { "type": "array", "items": { "$ref": "#/$defs/UserFederationMapperRepresentation" } }, "userFederationProviders": { "type": "array", "items": { "$ref": "#/$defs/UserFederationProviderRepresentation" } }, "userManagedAccessAllowed": { "title": "User-managed access", "description": "If enabled, users are allowed to manage their resources and permissions using the Account Management UI.", "type": "boolean" }, "users": { "type": "array", "items": { "$ref": "#/$defs/UserRepresentation" } }, "verifiableCredentialsEnabled": { "type": "boolean" }, "verifyEmail": { "title": "Verify email", "description": "Require user to verify their email address after initial login or after address changes are submitted.", "type": "boolean" }, "waitIncrementSeconds": { "title": "Wait increment", "description": "When failure threshold has been met, how much time should the user be locked out?", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "webAuthnPolicyAcceptableAaguids": { "title": "Acceptable AAGUIDs", "description": "The list of allowed AAGUIDs of which an authenticator can be registered. An AAGUID is a 128-bit identifier indicating the authenticator's type (e.g., make and model).", "type": "array", "items": { "type": "string" } }, "webAuthnPolicyAttestationConveyancePreference": { "title": "Attestation conveyance preference", "description": "Communicates to an authenticator the preference of how to generate an attestation statement.", "type": "string", "enum": [ "not specified", "none", "indirect", "direct" ] }, "webAuthnPolicyAuthenticatorAttachment": { "title": "Authenticator Attachment", "description": "Communicates to an authenticator an acceptable attachment pattern.", "type": "string", "enum": [ "not specified", "platform", "cross-platform" ] }, "webAuthnPolicyAvoidSameAuthenticatorRegister": { "title": "Avoid same authenticator registration", "description": "Avoid registering an authenticator that has already been registered.", "type": "boolean" }, "webAuthnPolicyCreateTimeout": { "title": "Timeout", "description": "The timeout value for creating the user's public key credential in seconds. If set to 0, this timeout option is not adapted.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "webAuthnPolicyExtraOrigins": { "type": "array", "items": { "type": "string" } }, "webAuthnPolicyPasswordlessAcceptableAaguids": { "title": "Acceptable AAGUIDs", "description": "The list of allowed AAGUIDs of which an authenticator can be registered. An AAGUID is a 128-bit identifier indicating the authenticator's type (e.g., make and model).", "type": "array", "items": { "type": "string" } }, "webAuthnPolicyPasswordlessAttestationConveyancePreference": { "title": "Attestation conveyance preference", "description": "Communicates to an authenticator the preference of how to generate an attestation statement.", "type": "string", "enum": [ "not specified", "none", "indirect", "direct" ] }, "webAuthnPolicyPasswordlessAuthenticatorAttachment": { "title": "Authenticator Attachment", "description": "Communicates to an authenticator an acceptable attachment pattern.", "type": "string", "enum": [ "not specified", "platform", "cross-platform" ] }, "webAuthnPolicyPasswordlessAvoidSameAuthenticatorRegister": { "title": "Avoid same authenticator registration", "description": "Avoid registering an authenticator that has already been registered.", "type": "boolean" }, "webAuthnPolicyPasswordlessCreateTimeout": { "title": "Timeout", "description": "The timeout value for creating the user's public key credential in seconds. If set to 0, this timeout option is not adapted.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "webAuthnPolicyPasswordlessExtraOrigins": { "type": "array", "items": { "type": "string" } }, "webAuthnPolicyPasswordlessRequireResidentKey": { "title": "Require discoverable credential", "description": "It tells an authenticator whether to create a public key credential as a Discoverable Credential.", "type": "string", "enum": [ "not specified", "Yes", "No" ] }, "webAuthnPolicyPasswordlessRpEntityName": { "title": "Relying party entity name", "description": "Human-readable server name as WebAuthn Relying Party", "type": "string" }, "webAuthnPolicyPasswordlessRpId": { "title": "Relying party ID", "description": "The WebAuthn Relying Party ID (RpID). It must be the origin's effective domain, e.g. 'company.com' or 'auth.company.com'.", "type": "string" }, "webAuthnPolicyPasswordlessSignatureAlgorithms": { "title": "Signature algorithms", "description": "The signature algorithms that should be used for the Authentication Assertion.", "type": "array", "items": { "type": "string", "enum": [ "Ed25519", "ES256", "ES384", "ES512", "RS256", "RS384", "RS512", "RS1" ] } }, "webAuthnPolicyPasswordlessUserVerificationRequirement": { "title": "User verification requirement", "description": "Communicates to an authenticator whether to require to verify a user.", "type": "string", "enum": [ "not specified", "required", "preferred", "discouraged" ] }, "webAuthnPolicyRequireResidentKey": { "title": "Require discoverable credential", "description": "It tells an authenticator whether to create a public key credential as a Discoverable Credential.", "type": "string", "enum": [ "not specified", "Yes", "No" ] }, "webAuthnPolicyRpEntityName": { "title": "Relying party entity name", "description": "Human-readable server name as WebAuthn Relying Party", "type": "string" }, "webAuthnPolicyRpId": { "title": "Relying party ID", "description": "The WebAuthn Relying Party ID (RpID). It must be the origin's effective domain, e.g. 'company.com' or 'auth.company.com'.", "type": "string" }, "webAuthnPolicySignatureAlgorithms": { "title": "Signature algorithms", "description": "The signature algorithms that should be used for the Authentication Assertion.", "type": "array", "items": { "type": "string", "enum": [ "Ed25519", "ES256", "ES384", "ES512", "RS256", "RS384", "RS512", "RS1" ] } }, "webAuthnPolicyUserVerificationRequirement": { "title": "User verification requirement", "description": "Communicates to an authenticator whether to require to verify a user.", "type": "string", "enum": [ "not specified", "required", "preferred", "discouraged" ] } } } ``` </details>

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

### spec.definition.adminPermissionsClient

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[access](#specdefinitionadminpermissionsclientaccess)|object||
|[adminUrl](#specdefinitionadminpermissionsclientadminurl)|string||
|[alwaysDisplayInConsole](#specdefinitionadminpermissionsclientalwaysdisplayinconsole)|boolean||
|[attributes](#specdefinitionadminpermissionsclientattributes)|object||
|[authenticationFlowBindingOverrides](#specdefinitionadminpermissionsclientauthenticationflowbindingoverrides)|object||
|[authorizationServicesEnabled](#specdefinitionadminpermissionsclientauthorizationservicesenabled)|boolean||
|[authorizationSettings](#specdefinitionadminpermissionsclientauthorizationsettings)|object||
|[baseUrl](#specdefinitionadminpermissionsclientbaseurl)|string||
|[bearerOnly](#specdefinitionadminpermissionsclientbeareronly)|boolean||
|[clientAuthenticatorType](#specdefinitionadminpermissionsclientclientauthenticatortype)|string||
|[clientId](#specdefinitionadminpermissionsclientclientid)|string||
|[clientTemplate](#specdefinitionadminpermissionsclientclienttemplate)|string||
|[consentRequired](#specdefinitionadminpermissionsclientconsentrequired)|boolean||
|[defaultClientScopes[]](#specdefinitionadminpermissionsclientdefaultclientscopes)|string||
|[defaultRoles[]](#specdefinitionadminpermissionsclientdefaultroles)|string||
|[description](#specdefinitionadminpermissionsclientdescription)|string||
|[directAccessGrantsEnabled](#specdefinitionadminpermissionsclientdirectaccessgrantsenabled)|boolean||
|[directGrantsOnly](#specdefinitionadminpermissionsclientdirectgrantsonly)|boolean||
|[enabled](#specdefinitionadminpermissionsclientenabled)|boolean||
|[frontchannelLogout](#specdefinitionadminpermissionsclientfrontchannellogout)|boolean||
|[fullScopeAllowed](#specdefinitionadminpermissionsclientfullscopeallowed)|boolean||
|[id](#specdefinitionadminpermissionsclientid)|string||
|[implicitFlowEnabled](#specdefinitionadminpermissionsclientimplicitflowenabled)|boolean||
|[name](#specdefinitionadminpermissionsclientname)|string||
|[nodeReRegistrationTimeout](#specdefinitionadminpermissionsclientnodereregistrationtimeout)|integer||
|[notBefore](#specdefinitionadminpermissionsclientnotbefore)|integer||
|[optionalClientScopes[]](#specdefinitionadminpermissionsclientoptionalclientscopes)|string||
|[origin](#specdefinitionadminpermissionsclientorigin)|string||
|[protocol](#specdefinitionadminpermissionsclientprotocol)|string||
|[protocolMappers[]](#specdefinitionadminpermissionsclientprotocolmappers)|object||
|[publicClient](#specdefinitionadminpermissionsclientpublicclient)|boolean||
|[redirectUris[]](#specdefinitionadminpermissionsclientredirecturis)|string||
|[registeredNodes](#specdefinitionadminpermissionsclientregisterednodes)|object||
|[registrationAccessToken](#specdefinitionadminpermissionsclientregistrationaccesstoken)|string||
|[rootUrl](#specdefinitionadminpermissionsclientrooturl)|string||
|[secret](#specdefinitionadminpermissionsclientsecret)|string||
|[serviceAccountsEnabled](#specdefinitionadminpermissionsclientserviceaccountsenabled)|boolean||
|[standardFlowEnabled](#specdefinitionadminpermissionsclientstandardflowenabled)|boolean||
|[surrogateAuthRequired](#specdefinitionadminpermissionsclientsurrogateauthrequired)|boolean||
|[type](#specdefinitionadminpermissionsclienttype)|string||
|[useTemplateConfig](#specdefinitionadminpermissionsclientusetemplateconfig)|boolean||
|[useTemplateMappers](#specdefinitionadminpermissionsclientusetemplatemappers)|boolean||
|[useTemplateScope](#specdefinitionadminpermissionsclientusetemplatescope)|boolean||
|[webOrigins[]](#specdefinitionadminpermissionsclientweborigins)|string||

ClientRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "access": { "type": "object", "additionalProperties": { "type": "boolean" } }, "adminUrl": { "title": "Admin URL", "description": "URL to the admin interface of the client. Set this if the client supports the adapter REST API. This REST API allows the auth server to push revocation policies and other administrative tasks. Usually this is set to the base URL of the client.", "type": "string" }, "alwaysDisplayInConsole": { "title": "Always display in UI", "description": "Always list this client in the Account UI, even if the user does not have an active session.", "type": "boolean" }, "attributes": { "type": "object", "additionalProperties": { "type": "string" } }, "authenticationFlowBindingOverrides": { "title": "Authentication flow overrides", "type": "object", "additionalProperties": { "type": "string" } }, "authorizationServicesEnabled": { "title": "Authorization", "description": "Enable/Disable fine-grained authorization support for a client.", "type": "boolean" }, "authorizationSettings": { "$ref": "#/$defs/ResourceServerRepresentation" }, "baseUrl": { "title": "Home URL", "description": "Default URL to use when the auth server needs to redirect or link back to the client.", "type": "string" }, "bearerOnly": { "description": "This is a special OIDC type. This client only allows bearer token requests and cannot participate in browser logins.", "type": "boolean" }, "clientAuthenticatorType": { "title": "Client Authenticator", "description": "Client Authenticator used for authentication of this client against Keycloak server", "type": "string", "enum": [ "client-jwt", "client-secret", "client-secret-jwt", "client-x509" ] }, "clientId": { "title": "Client ID", "description": "The client identifier registered with the identity provider.", "type": "string" }, "clientTemplate": { "type": "string" }, "consentRequired": { "title": "Consent required", "description": "If enabled, users have to consent to client access.", "type": "boolean" }, "defaultClientScopes": { "type": "array", "items": { "type": "string" } }, "defaultRoles": { "type": "array", "items": { "type": "string" } }, "description": { "title": "Description", "description": "Help text for the description of the new flow", "type": "string" }, "directAccessGrantsEnabled": { "title": "Direct access grants", "description": "This enables support for Direct Access Grants, which means that client has access to username/password of user and exchange it directly with Keycloak server for access token. In terms of OAuth2 specification, this enables support of 'Resource Owner Password Credentials Grant' for this client.", "type": "boolean" }, "directGrantsOnly": { "type": "boolean" }, "enabled": { "title": "Enabled", "description": "Disabled clients cannot initiate a login or have obtained access tokens.", "type": "boolean" }, "frontchannelLogout": { "title": "Front channel logout", "description": "When true, logout requires a browser redirect to client. When false, server performs a background invocation for logout.", "type": "boolean" }, "fullScopeAllowed": { "title": "Full scope allowed", "description": "Allows you to disable all restrictions.", "type": "boolean" }, "id": { "type": "string" }, "implicitFlowEnabled": { "title": "Implicit flow", "description": "This enables support for OpenID Connect redirect based authentication without authorization code. In terms of OpenID Connect or OAuth2 specifications, this enables support of 'Implicit Flow' for this client.", "type": "boolean" }, "name": { "title": "Name", "description": "Specifies display name of the client. For example 'My Client'. Supports keys for localized values as well. For example: ${my_client}.", "type": "string" }, "nodeReRegistrationTimeout": { "title": "Node Re-registration timeout", "description": "Interval to specify max time for registered clients cluster nodes to re-register. If cluster node will not send re-registration request to Keycloak within this time, it will be unregistered from Keycloak.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "notBefore": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "optionalClientScopes": { "type": "array", "items": { "type": "string" } }, "origin": { "type": "string" }, "protocol": { "title": "Protocol", "type": "string" }, "protocolMappers": { "type": "array", "items": { "$ref": "#/$defs/ProtocolMapperRepresentation" } }, "publicClient": { "title": "Client authentication", "description": "This defines the type of the OIDC client. When it's ON, the OIDC type is set to confidential access type. When it's OFF, it is set to public access type.", "type": "boolean" }, "redirectUris": { "title": "Valid redirect URIs", "description": "Valid URI pattern a browser can redirect to after a successful login. Simple wildcards are allowed such as 'http://example.com/*'. Relative path can be specified too such as /my/relative/path/*. Relative paths are relative to the client root URL, or if none is specified the auth server root URL is used. For SAML, you must set valid URI patterns if you are relying on the consumer service URL embedded with the login request.", "type": "array", "items": { "type": "string" } }, "registeredNodes": { "type": "object", "additionalProperties": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 } }, "registrationAccessToken": { "title": "Registration access token", "description": "The registration access token provides access for clients to the client registration service.", "type": "string" }, "rootUrl": { "title": "Root URL", "description": "Root URL appended to relative URLs", "type": "string" }, "secret": { "title": "Client Secret", "type": "string" }, "serviceAccountsEnabled": { "title": "Service accounts roles", "description": "Allows you to authenticate this client to Keycloak and retrieve access token dedicated to this client. In terms of OAuth2 specification, this enables support of 'Client Credentials Grant' for this client.", "type": "boolean" }, "standardFlowEnabled": { "title": "Standard flow", "description": "This enables standard OpenID Connect redirect based authentication with authorization code. In terms of OpenID Connect or OAuth2 specifications, this enables support of 'Authorization Code Flow' for this client.", "type": "boolean" }, "surrogateAuthRequired": { "type": "boolean" }, "type": { "type": "string" }, "useTemplateConfig": { "type": "boolean" }, "useTemplateMappers": { "type": "boolean" }, "useTemplateScope": { "type": "boolean" }, "webOrigins": { "title": "Web origins", "description": "Allowed CORS origins. To permit all origins of Valid Redirect URIs, add '+'. This does not include the '*' wildcard though. To permit all origins, explicitly add '*'.", "type": "array", "items": { "type": "string" } } } } ``` </details>

---

### spec.definition.adminPermissionsClient.access

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.adminUrl

Type: string

URL to the admin interface of the client. Set this if the client supports the adapter REST API. This REST API allows the auth server to push revocation policies and other administrative tasks. Usually this is set to the base URL of the client.

---

### spec.definition.adminPermissionsClient.alwaysDisplayInConsole

Type: boolean

Always list this client in the Account UI, even if the user does not have an active session.

---

### spec.definition.adminPermissionsClient.attributes

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.authenticationFlowBindingOverrides

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.authorizationServicesEnabled

Type: boolean

Enable/Disable fine-grained authorization support for a client.

---

### spec.definition.adminPermissionsClient.authorizationSettings

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[allowRemoteResourceManagement](#specdefinitionadminpermissionsclientauthorizationsettingsallowremoteresourcemanagement)|boolean||
|[authorizationSchema](#specdefinitionadminpermissionsclientauthorizationsettingsauthorizationschema)|object||
|[clientId](#specdefinitionadminpermissionsclientauthorizationsettingsclientid)|string||
|[decisionStrategy](#specdefinitionadminpermissionsclientauthorizationsettingsdecisionstrategy)|string||
|[id](#specdefinitionadminpermissionsclientauthorizationsettingsid)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingsname)|string||
|[policies[]](#specdefinitionadminpermissionsclientauthorizationsettingspolicies)|object||
|[policyEnforcementMode](#specdefinitionadminpermissionsclientauthorizationsettingspolicyenforcementmode)|string||
|[resources[]](#specdefinitionadminpermissionsclientauthorizationsettingsresources)|object||
|[scopes[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopes)|object||

ResourceServerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "allowRemoteResourceManagement": { "type": "boolean" }, "authorizationSchema": { "$ref": "#/$defs/AuthorizationSchema" }, "clientId": { "type": "string" }, "decisionStrategy": { "$ref": "#/$defs/DecisionStrategy" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "policyEnforcementMode": { "$ref": "#/$defs/PolicyEnforcementMode" }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" } } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.allowRemoteResourceManagement

Type: boolean

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.authorizationSchema

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[resourceTypes](#specdefinitionadminpermissionsclientauthorizationsettingsauthorizationschemaresourcetypes)|object||

AuthorizationSchema

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "resourceTypes": { "type": "object", "additionalProperties": { "$ref": "#/$defs/ResourceType" } } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.authorizationSchema.resourceTypes

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.clientId

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.decisionStrategy

Type: string

DecisionStrategy

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "AFFIRMATIVE", "UNANIMOUS", "CONSENSUS" ] } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesconfig)|object||
|[decisionStrategy](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesdecisionstrategy)|string||
|[description](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesdescription)|string||
|[id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesid)|string||
|[logic](#specdefinitionadminpermissionsclientauthorizationsettingspolicieslogic)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesname)|string||
|[owner](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesowner)|string||
|[policies[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciespolicies)|string||
|[resourceType](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcetype)|string||
|[resources[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresources)|string||
|[resourcesData[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdata)|object||
|[scopes[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopes)|string||
|[scopesData[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdata)|object||
|[type](#specdefinitionadminpermissionsclientauthorizationsettingspoliciestype)|string||

PolicyRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "decisionStrategy": { "$ref": "#/$defs/DecisionStrategy" }, "description": { "type": "string" }, "id": { "type": "string" }, "logic": { "$ref": "#/$defs/Logic" }, "name": { "type": "string" }, "owner": { "type": "string" }, "policies": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourceType": { "type": "string" }, "resources": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourcesData": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" }, "uniqueItems": true }, "scopes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "scopesData": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].config

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].decisionStrategy

Type: string

DecisionStrategy

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "AFFIRMATIVE", "UNANIMOUS", "CONSENSUS" ] } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].description

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].logic

Type: string

Logic

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "POSITIVE", "NEGATIVE" ] } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].owner

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].policies[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourceType

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resources[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[\_id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataid)|string||
|[attributes](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataattributes)|object||
|[displayName](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatadisplayname)|string||
|[icon_uri](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataiconuri)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataname)|string||
|[owner](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataowner)|object||
|[ownerManagedAccess](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataownermanagedaccess)|boolean||
|[scopes[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopes)|object||
|[scopesUma[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[type](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatatype)|string||
|[uri](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatauri)|string||
|[uris[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatauris)|string||

ResourceRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "_id": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "displayName": { "type": "string" }, "icon_uri": { "type": "string" }, "name": { "type": "string" }, "owner": { "type": "object", "allOf": [ { "$ref": "#/$defs/ResourceOwnerRepresentation" } ] }, "ownerManagedAccess": { "type": "boolean" }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "scopesUma": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" }, "uri": { "type": "string" }, "uris": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[]._id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].attributes

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].displayName

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].icon_uri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].owner

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataownerid)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdataownername)|string||

ResourceOwnerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "id": { "type": "string" }, "name": { "type": "string" } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].owner.id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].owner.name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopes[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[displayName](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[iconUri](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesname)|string||

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopesUma[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[displayName](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[iconUri](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesresourcesdatascopesumaname)|string||

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].type

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].uri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].resourcesData[].uris[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopes[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[displayName](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdatadisplayname)|string||
|[iconUri](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataiconuri)|string||
|[id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataid)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataname)|string||
|[resources[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresources)|object||

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].displayName

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].iconUri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[\_id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[attributes](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[displayName](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[icon_uri](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[owner](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[ownerManagedAccess](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[type](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[uri](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[uris[]](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesuris)|string||

ResourceRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "_id": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "displayName": { "type": "string" }, "icon_uri": { "type": "string" }, "name": { "type": "string" }, "owner": { "type": "object", "allOf": [ { "$ref": "#/$defs/ResourceOwnerRepresentation" } ] }, "ownerManagedAccess": { "type": "boolean" }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "scopesUma": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" }, "uri": { "type": "string" }, "uris": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingspoliciesscopesdataresourcesownername)|string||

ResourceOwnerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "id": { "type": "string" }, "name": { "type": "string" } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policies[].type

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.policyEnforcementMode

Type: string

PolicyEnforcementMode

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "ENFORCING", "PERMISSIVE", "DISABLED" ] } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[\_id](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesid)|string||
|[attributes](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesattributes)|object||
|[displayName](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesdisplayname)|string||
|[icon_uri](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesiconuri)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesname)|string||
|[owner](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesowner)|object||
|[ownerManagedAccess](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesownermanagedaccess)|boolean||
|[scopes[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopes)|object||
|[scopesUma[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesuma)|object||
|[type](#specdefinitionadminpermissionsclientauthorizationsettingsresourcestype)|string||
|[uri](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesuri)|string||
|[uris[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesuris)|string||

ResourceRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "_id": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "displayName": { "type": "string" }, "icon_uri": { "type": "string" }, "name": { "type": "string" }, "owner": { "type": "object", "allOf": [ { "$ref": "#/$defs/ResourceOwnerRepresentation" } ] }, "ownerManagedAccess": { "type": "boolean" }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "scopesUma": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" }, "uri": { "type": "string" }, "uris": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[]._id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].attributes

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].displayName

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].icon_uri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].owner

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesownerid)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesownername)|string||

ResourceOwnerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "id": { "type": "string" }, "name": { "type": "string" } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].owner.id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].owner.name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].ownerManagedAccess

Type: boolean

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[displayName](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesdisplayname)|string||
|[iconUri](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesiconuri)|string||
|[id](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesid)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesname)|string||
|[policies[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespolicies)|object||

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].displayName

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].iconUri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[decisionStrategy](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[description](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[id](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesid)|string||
|[logic](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespolicieslogic)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesname)|string||
|[owner](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesowner)|string||
|[policies[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[resourceType](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesresourcetype)|string||
|[resources[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesresources)|string||
|[scopes[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[type](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopespoliciestype)|string||

PolicyRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "decisionStrategy": { "$ref": "#/$defs/DecisionStrategy" }, "description": { "type": "string" }, "id": { "type": "string" }, "logic": { "$ref": "#/$defs/Logic" }, "name": { "type": "string" }, "owner": { "type": "string" }, "policies": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourceType": { "type": "string" }, "resources": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourcesData": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" }, "uniqueItems": true }, "scopes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "scopesData": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

DecisionStrategy

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "AFFIRMATIVE", "UNANIMOUS", "CONSENSUS" ] } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

Logic

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "POSITIVE", "NEGATIVE" ] } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].resourceType

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[displayName](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumadisplayname)|string||
|[iconUri](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumaiconuri)|string||
|[id](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumaid)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumaname)|string||
|[policies[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapolicies)|object||

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].displayName

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].iconUri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[decisionStrategy](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[description](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[id](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[logic](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[owner](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[policies[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[resourceType](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesresourcetype)|string||
|[resources[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[scopes[]](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[type](#specdefinitionadminpermissionsclientauthorizationsettingsresourcesscopesumapoliciestype)|string||

PolicyRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "decisionStrategy": { "$ref": "#/$defs/DecisionStrategy" }, "description": { "type": "string" }, "id": { "type": "string" }, "logic": { "$ref": "#/$defs/Logic" }, "name": { "type": "string" }, "owner": { "type": "string" }, "policies": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourceType": { "type": "string" }, "resources": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourcesData": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" }, "uniqueItems": true }, "scopes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "scopesData": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

DecisionStrategy

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "AFFIRMATIVE", "UNANIMOUS", "CONSENSUS" ] } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

Logic

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "POSITIVE", "NEGATIVE" ] } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].resourceType

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].type

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].uri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.resources[].uris[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[displayName](#specdefinitionadminpermissionsclientauthorizationsettingsscopesdisplayname)|string||
|[iconUri](#specdefinitionadminpermissionsclientauthorizationsettingsscopesiconuri)|string||
|[id](#specdefinitionadminpermissionsclientauthorizationsettingsscopesid)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingsscopesname)|string||
|[policies[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopespolicies)|object||
|[resources[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresources)|object||

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].displayName

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].iconUri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesconfig)|object||
|[decisionStrategy](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesdecisionstrategy)|string||
|[description](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesdescription)|string||
|[id](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesid)|string||
|[logic](#specdefinitionadminpermissionsclientauthorizationsettingsscopespolicieslogic)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesname)|string||
|[owner](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesowner)|string||
|[policies[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciespolicies)|string||
|[resourceType](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcetype)|string||
|[resources[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresources)|string||
|[resourcesData[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdata)|object||
|[scopes[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesscopes)|string||
|[type](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciestype)|string||

PolicyRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "decisionStrategy": { "$ref": "#/$defs/DecisionStrategy" }, "description": { "type": "string" }, "id": { "type": "string" }, "logic": { "$ref": "#/$defs/Logic" }, "name": { "type": "string" }, "owner": { "type": "string" }, "policies": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourceType": { "type": "string" }, "resources": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourcesData": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" }, "uniqueItems": true }, "scopes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "scopesData": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].config

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].decisionStrategy

Type: string

DecisionStrategy

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "AFFIRMATIVE", "UNANIMOUS", "CONSENSUS" ] } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].description

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].logic

Type: string

Logic

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "POSITIVE", "NEGATIVE" ] } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].owner

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].policies[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourceType

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resources[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[\_id](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[attributes](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[displayName](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[icon_uri](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[owner](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[ownerManagedAccess](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[type](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[uri](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[uris[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdatauris)|string||

ResourceRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "_id": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "displayName": { "type": "string" }, "icon_uri": { "type": "string" }, "name": { "type": "string" }, "owner": { "type": "object", "allOf": [ { "$ref": "#/$defs/ResourceOwnerRepresentation" } ] }, "ownerManagedAccess": { "type": "boolean" }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "scopesUma": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" }, "uri": { "type": "string" }, "uris": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingsscopespoliciesresourcesdataownername)|string||

ResourceOwnerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "id": { "type": "string" }, "name": { "type": "string" } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].scopes[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].policies[].type

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[\_id](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesid)|string||
|[attributes](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesattributes)|object||
|[displayName](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesdisplayname)|string||
|[icon_uri](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesiconuri)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesname)|string||
|[owner](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesowner)|object||
|[ownerManagedAccess](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesownermanagedaccess)|boolean||
|[type](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcestype)|string||
|[uri](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesuri)|string||
|[uris[]](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesuris)|string||

ResourceRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "_id": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "displayName": { "type": "string" }, "icon_uri": { "type": "string" }, "name": { "type": "string" }, "owner": { "type": "object", "allOf": [ { "$ref": "#/$defs/ResourceOwnerRepresentation" } ] }, "ownerManagedAccess": { "type": "boolean" }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "scopesUma": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" }, "uri": { "type": "string" }, "uris": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[]._id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].attributes

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].displayName

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].icon_uri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].owner

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[id](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesownerid)|string||
|[name](#specdefinitionadminpermissionsclientauthorizationsettingsscopesresourcesownername)|string||

ResourceOwnerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "id": { "type": "string" }, "name": { "type": "string" } } } ``` </details>

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].owner.id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].owner.name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].ownerManagedAccess

Type: boolean

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].type

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].uri

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.authorizationSettings.scopes[].resources[].uris[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.baseUrl

Type: string

Default URL to use when the auth server needs to redirect or link back to the client.

---

### spec.definition.adminPermissionsClient.bearerOnly

Type: boolean

This is a special OIDC type. This client only allows bearer token requests and cannot participate in browser logins.

---

### spec.definition.adminPermissionsClient.clientAuthenticatorType

Type: string

Client Authenticator used for authentication of this client against Keycloak server

---

### spec.definition.adminPermissionsClient.clientId

Type: string

The client identifier registered with the identity provider.

---

### spec.definition.adminPermissionsClient.clientTemplate

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.consentRequired

Type: boolean

If enabled, users have to consent to client access.

---

### spec.definition.adminPermissionsClient.defaultClientScopes[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.defaultRoles[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.description

Type: string

Help text for the description of the new flow

---

### spec.definition.adminPermissionsClient.directAccessGrantsEnabled

Type: boolean

This enables support for Direct Access Grants, which means that client has access to username/password of user and exchange it directly with Keycloak server for access token. In terms of OAuth2 specification, this enables support of 'Resource Owner Password Credentials Grant' for this client.

---

### spec.definition.adminPermissionsClient.directGrantsOnly

Type: boolean

*missing*

---

### spec.definition.adminPermissionsClient.enabled

Type: boolean

Disabled clients cannot initiate a login or have obtained access tokens.

---

### spec.definition.adminPermissionsClient.frontchannelLogout

Type: boolean

When true, logout requires a browser redirect to client. When false, server performs a background invocation for logout.

---

### spec.definition.adminPermissionsClient.fullScopeAllowed

Type: boolean

Allows you to disable all restrictions.

---

### spec.definition.adminPermissionsClient.id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.implicitFlowEnabled

Type: boolean

This enables support for OpenID Connect redirect based authentication without authorization code. In terms of OpenID Connect or OAuth2 specifications, this enables support of 'Implicit Flow' for this client.

---

### spec.definition.adminPermissionsClient.name

Type: string

Specifies display name of the client. For example 'My Client'. Supports keys for localized values as well. For example: ${my_client}.

---

### spec.definition.adminPermissionsClient.nodeReRegistrationTimeout

Type: integer

Interval to specify max time for registered clients cluster nodes to re-register. If cluster node will not send re-registration request to Keycloak within this time, it will be unregistered from Keycloak.

---

### spec.definition.adminPermissionsClient.notBefore

Type: integer

*missing*

---

### spec.definition.adminPermissionsClient.optionalClientScopes[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.origin

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.protocol

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.protocolMappers[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionadminpermissionsclientprotocolmappersconfig)|object||
|[consentRequired](#specdefinitionadminpermissionsclientprotocolmappersconsentrequired)|boolean||
|[consentText](#specdefinitionadminpermissionsclientprotocolmappersconsenttext)|string||
|[id](#specdefinitionadminpermissionsclientprotocolmappersid)|string||
|[name](#specdefinitionadminpermissionsclientprotocolmappersname)|string||
|[protocol](#specdefinitionadminpermissionsclientprotocolmappersprotocol)|string||
|[protocolMapper](#specdefinitionadminpermissionsclientprotocolmappersprotocolmapper)|string||

ProtocolMapperRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "consentRequired": { "type": "boolean" }, "consentText": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "protocol": { "type": "string", "enum": [ "openid-connect", "saml" ] }, "protocolMapper": { "type": "string" } } } ``` </details>

---

### spec.definition.adminPermissionsClient.protocolMappers[].config

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.protocolMappers[].consentRequired

Type: boolean

*missing*

---

### spec.definition.adminPermissionsClient.protocolMappers[].consentText

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.protocolMappers[].id

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.protocolMappers[].name

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.protocolMappers[].protocol

Type: string

ProtocolMapperRepresentationProtocol

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "openid-connect", "saml" ] } ``` </details>

---

### spec.definition.adminPermissionsClient.protocolMappers[].protocolMapper

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.publicClient

Type: boolean

This defines the type of the OIDC client. When it's ON, the OIDC type is set to confidential access type. When it's OFF, it is set to public access type.

---

### spec.definition.adminPermissionsClient.redirectUris[]

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.registeredNodes

Type: object

*missing*

---

### spec.definition.adminPermissionsClient.registrationAccessToken

Type: string

The registration access token provides access for clients to the client registration service.

---

### spec.definition.adminPermissionsClient.rootUrl

Type: string

Root URL appended to relative URLs

---

### spec.definition.adminPermissionsClient.secret

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.serviceAccountsEnabled

Type: boolean

Allows you to authenticate this client to Keycloak and retrieve access token dedicated to this client. In terms of OAuth2 specification, this enables support of 'Client Credentials Grant' for this client.

---

### spec.definition.adminPermissionsClient.standardFlowEnabled

Type: boolean

This enables standard OpenID Connect redirect based authentication with authorization code. In terms of OpenID Connect or OAuth2 specifications, this enables support of 'Authorization Code Flow' for this client.

---

### spec.definition.adminPermissionsClient.surrogateAuthRequired

Type: boolean

*missing*

---

### spec.definition.adminPermissionsClient.type

Type: string

*missing*

---

### spec.definition.adminPermissionsClient.useTemplateConfig

Type: boolean

*missing*

---

### spec.definition.adminPermissionsClient.useTemplateMappers

Type: boolean

*missing*

---

### spec.definition.adminPermissionsClient.useTemplateScope

Type: boolean

*missing*

---

### spec.definition.adminPermissionsClient.webOrigins[]

Type: string

*missing*

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

```json { "type": "object", "properties": { "contentSecurityPolicy": { "title": "Content-Security-Policy", "description": "Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>", "type": "string" }, "contentSecurityPolicyReportOnly": { "title": "Content-Security-Policy-Report-Only", "description": "For testing Content Security Policies <1>Learn more</1>", "type": "string" }, "strictTransportSecurity": { "title": "HTTP Strict Transport Security (HSTS)", "description": "The Strict-Transport-Security HTTP header tells browsers to always use HTTPS. Once a browser sees this header, it will only visit the site over HTTPS for the time specified (1 year) at max-age, including the subdomains. <1>Learn more</1>", "type": "string" }, "xContentTypeOptions": { "title": "X-Content-Type-Options", "description": "The default value prevents Internet Explorer and Google Chrome from MIME-sniffing a response away from the declared content-type. <1>Learn more</1>", "type": "string" }, "xFrameOptions": { "title": "X-Frame-Options", "description": "Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>", "type": "string" }, "xRobotsTag": { "title": "X-Robots-Tag", "description": "Prevent pages from appearing in search engines. <1>Learn more</1>", "type": "string" }, "xXSSProtection": { "title": "X-XSS-Protection", "description": "This header configures the Cross-site scripting (XSS) filter in your browser. Using the default behaviour, the browser will prevent rendering of the page when a XSS attack is detected. <1>Learn more</1>", "type": "string" } } } ``` </details>

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

```json { "type": "object", "properties": { "globalPolicies": { "type": "array", "items": { "$ref": "#/$defs/ClientPolicyRepresentation" } }, "policies": { "type": "array", "items": { "$ref": "#/$defs/ClientPolicyRepresentation" } } } } ``` </details>

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

```json { "type": "object", "properties": { "conditions": { "type": "array", "items": { "$ref": "#/$defs/ClientPolicyConditionRepresentation" } }, "description": { "type": "string" }, "enabled": { "type": "boolean" }, "name": { "type": "string" }, "profiles": { "type": "array", "items": { "type": "string" } } } } ``` </details>

---

### spec.definition.clientPolicies.globalPolicies[].conditions[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[condition](#specdefinitionclientpoliciesglobalpoliciesconditionscondition)|string||
|[configuration](#specdefinitionclientpoliciesglobalpoliciesconditionsconfiguration)|object||

ClientPolicyConditionRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "condition": { "type": "string" }, "configuration": { "type": "object" } } } ``` </details>

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

```json { "type": "object", "properties": { "conditions": { "type": "array", "items": { "$ref": "#/$defs/ClientPolicyConditionRepresentation" } }, "description": { "type": "string" }, "enabled": { "type": "boolean" }, "name": { "type": "string" }, "profiles": { "type": "array", "items": { "type": "string" } } } } ``` </details>

---

### spec.definition.clientPolicies.policies[].conditions[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[condition](#specdefinitionclientpoliciespoliciesconditionscondition)|string||
|[configuration](#specdefinitionclientpoliciespoliciesconditionsconfiguration)|object||

ClientPolicyConditionRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "condition": { "type": "string" }, "configuration": { "type": "object" } } } ``` </details>

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

```json { "type": "object", "properties": { "globalProfiles": { "type": "array", "items": { "$ref": "#/$defs/ClientProfileRepresentation" } }, "profiles": { "type": "array", "items": { "$ref": "#/$defs/ClientProfileRepresentation" } } } } ``` </details>

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

```json { "type": "object", "properties": { "description": { "type": "string" }, "executors": { "type": "array", "items": { "$ref": "#/$defs/ClientPolicyExecutorRepresentation" } }, "name": { "type": "string" } } } ``` </details>

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

```json { "type": "object", "properties": { "configuration": { "type": "object" }, "executor": { "type": "string" } } } ``` </details>

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

```json { "type": "object", "properties": { "description": { "type": "string" }, "executors": { "type": "array", "items": { "$ref": "#/$defs/ClientPolicyExecutorRepresentation" } }, "name": { "type": "string" } } } ``` </details>

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

```json { "type": "object", "properties": { "configuration": { "type": "object" }, "executor": { "type": "string" } } } ``` </details>

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

```json { "type": "object", "properties": { "attributes": { "type": "object", "additionalProperties": { "type": "string" } }, "bearerOnly": { "type": "boolean" }, "consentRequired": { "type": "boolean" }, "description": { "type": "string" }, "directAccessGrantsEnabled": { "type": "boolean" }, "frontchannelLogout": { "type": "boolean" }, "fullScopeAllowed": { "type": "boolean" }, "id": { "type": "string" }, "implicitFlowEnabled": { "type": "boolean" }, "name": { "type": "string" }, "protocol": { "type": "string" }, "protocolMappers": { "type": "array", "items": { "$ref": "#/$defs/ProtocolMapperRepresentation" } }, "publicClient": { "type": "boolean" }, "serviceAccountsEnabled": { "type": "boolean" }, "standardFlowEnabled": { "type": "boolean" } } } ``` </details>

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

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "consentRequired": { "type": "boolean" }, "consentText": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "protocol": { "type": "string", "enum": [ "openid-connect", "saml" ] }, "protocolMapper": { "type": "string" } } } ``` </details>

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

```json { "type": "object", "properties": { "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "clientRole": { "type": "boolean" }, "composite": { "type": "boolean" }, "composites": { "$ref": "#/$defs/Composites" }, "containerId": { "type": "string" }, "description": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "scopeParamRequired": { "type": "boolean" } } } ``` </details>

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

```json { "type": "object", "properties": { "application": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "client": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "realm": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } } } ``` </details>

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

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "id": { "type": "string" }, "identityProviderAlias": { "type": "string" }, "identityProviderMapper": { "type": "string" }, "name": { "title": "Name", "description": "Name of the mapper.", "type": "string" } } } ``` </details>

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

```json { "type": "object", "properties": { "application": { "type": "object", "additionalProperties": { "type": "array", "items": { "$ref": "#/$defs/RoleRepresentation" } } }, "client": { "type": "object", "additionalProperties": { "type": "array", "items": { "$ref": "#/$defs/RoleRepresentation" } } }, "realm": { "type": "array", "items": { "$ref": "#/$defs/RoleRepresentation" } } } } ``` </details>

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

```json { "type": "object", "properties": { "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "clientRole": { "type": "boolean" }, "composite": { "type": "boolean" }, "composites": { "$ref": "#/$defs/Composites" }, "containerId": { "type": "string" }, "description": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "scopeParamRequired": { "type": "boolean" } } } ``` </details>

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

```json { "type": "object", "properties": { "application": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "client": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "realm": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } } } ``` </details>

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

```json { "type": "object", "properties": { "client": { "type": "string" }, "clientScope": { "type": "string" }, "clientTemplate": { "type": "string" }, "roles": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "self": { "type": "string" } } } ``` </details>

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

```json { "type": "object", "properties": { "auth": { "title": "Authentication", "type": "string", "enum": [ "true", "false", "" ] }, "envelopeFrom": { "title": "Envelope from", "description": "An email address used for bounces (optional).", "type": "string" }, "from": { "title": "From", "type": "string" }, "fromDisplayName": { "title": "From display name", "description": "A user-friendly name for the 'From' address (optional).", "type": "string" }, "host": { "title": "Host", "type": "string" }, "password": { "title": "Password", "description": "SMTP password. This field is able to obtain its value from vault, use ${vault.ID} format.", "type": "string" }, "port": { "title": "Port", "type": "string" }, "replyTo": { "title": "Reply to", "type": "string" }, "replyToDisplayName": { "title": "Reply to display name", "description": "A user-friendly name for the 'Reply-To' address (optional).", "type": "string" }, "ssl": { "title": "Enable SSL", "type": "string", "enum": [ "true", "false", "" ] }, "starttls": { "title": "Enable StartTLS", "type": "string", "enum": [ "true", "false", "" ] }, "user": { "title": "Username", "type": "string" } } } ``` </details>

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

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "federationMapperType": { "type": "string" }, "federationProviderDisplayName": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" } } } ``` </details>

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

```json { "type": "object", "properties": { "changedSyncPeriod": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "config": { "type": "object", "additionalProperties": { "type": "string" } }, "displayName": { "type": "string" }, "fullSyncPeriod": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "id": { "type": "string" }, "lastSync": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "priority": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "providerName": { "type": "string" } } } ``` </details>

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

The name of the namespaced instance to which this object belongs to.

---

### spec.patchFrom[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[configMapKeyRef](#specpatchfromconfigmapkeyref)|object||
|[path](#specpatchfrompath)|string|✅|
|[secretKeyRef](#specpatchfromsecretkeyref)|object||
|[value](#specpatchfromvalue)|string||
|[value_as](#specpatchfromvalueas)|string||

*missing*

---

### spec.patchFrom[].configMapKeyRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[key](#specpatchfromconfigmapkeyrefkey)|string|✅|
|[name](#specpatchfromconfigmapkeyrefname)|string|✅|
|[optional](#specpatchfromconfigmapkeyrefoptional)|boolean||

Selects a key from a ConfigMap.

---

### spec.patchFrom[].configMapKeyRef.key

Type: string

The key to select.

---

### spec.patchFrom[].configMapKeyRef.name

Type: string

Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names

---

### spec.patchFrom[].configMapKeyRef.optional

Type: boolean

Specify whether the ConfigMap or its key must be defined

---

### spec.patchFrom[].path

Type: string

*missing*

---

### spec.patchFrom[].secretKeyRef

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[key](#specpatchfromsecretkeyrefkey)|string|✅|
|[name](#specpatchfromsecretkeyrefname)|string|✅|
|[optional](#specpatchfromsecretkeyrefoptional)|boolean||

SecretKeySelector selects a key of a Secret.

---

### spec.patchFrom[].secretKeyRef.key

Type: string

The key of the secret to select from.  Must be a valid secret key.

---

### spec.patchFrom[].secretKeyRef.name

Type: string

Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names

---

### spec.patchFrom[].secretKeyRef.optional

Type: boolean

Specify whether the Secret or its key must be defined

---

### spec.patchFrom[].value

Type: string

*missing*

---

### spec.patchFrom[].value_as

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