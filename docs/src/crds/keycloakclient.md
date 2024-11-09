# KeycloakClient

## v1

Auto-generated derived type for KeycloakClientSpec via `CustomResource`

|Name|Type|Required|
|:---|:---|:------:|
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
|[spec.definition.authorizationSettings.clientId](#specdefinitionauthorizationsettingsclientid)|string||
|[spec.definition.authorizationSettings.decisionStrategy](#specdefinitionauthorizationsettingsdecisionstrategy)|string||
|[spec.definition.authorizationSettings.id](#specdefinitionauthorizationsettingsid)|string||
|[spec.definition.authorizationSettings.name](#specdefinitionauthorizationsettingsname)|string||
|[spec.definition.authorizationSettings.policies](#specdefinitionauthorizationsettingspolicies)|array||
|[spec.definition.authorizationSettings.policies[]](#specdefinitionauthorizationsettingspolicies)|object||
|[spec.definition.authorizationSettings.policies[].config](#specdefinitionauthorizationsettingspoliciesconfig)|object||
|[spec.definition.authorizationSettings.policies[].decisionStrategy](#specdefinitionauthorizationsettingspoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.policies[].description](#specdefinitionauthorizationsettingspoliciesdescription)|string||
|[spec.definition.authorizationSettings.policies[].id](#specdefinitionauthorizationsettingspoliciesid)|string||
|[spec.definition.authorizationSettings.policies[].logic](#specdefinitionauthorizationsettingspolicieslogic)|string||
|[spec.definition.authorizationSettings.policies[].name](#specdefinitionauthorizationsettingspoliciesname)|string||
|[spec.definition.authorizationSettings.policies[].owner](#specdefinitionauthorizationsettingspoliciesowner)|string||
|[spec.definition.authorizationSettings.policies[].policies](#specdefinitionauthorizationsettingspoliciespolicies)|array||
|[spec.definition.authorizationSettings.policies[].policies[]](#specdefinitionauthorizationsettingspoliciespolicies)|string||
|[spec.definition.authorizationSettings.policies[].resources](#specdefinitionauthorizationsettingspoliciesresources)|array||
|[spec.definition.authorizationSettings.policies[].resources[]](#specdefinitionauthorizationsettingspoliciesresources)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData](#specdefinitionauthorizationsettingspoliciesresourcesdata)|array||
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
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].type](#specdefinitionauthorizationsettingspoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uri](#specdefinitionauthorizationsettingspoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[]._id](#specdefinitionauthorizationsettingspoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingspoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingspoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].name](#specdefinitionauthorizationsettingspoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner](#specdefinitionauthorizationsettingspoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingspoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingspoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingspoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingspoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].type](#specdefinitionauthorizationsettingspoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uri](#specdefinitionauthorizationsettingspoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.policies[].scopes](#specdefinitionauthorizationsettingspoliciesscopes)|array||
|[spec.definition.authorizationSettings.policies[].scopes[]](#specdefinitionauthorizationsettingspoliciesscopes)|string||
|[spec.definition.authorizationSettings.policies[].scopesData](#specdefinitionauthorizationsettingspoliciesscopesdata)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[]](#specdefinitionauthorizationsettingspoliciesscopesdata)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdatadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].iconUri](#specdefinitionauthorizationsettingspoliciesscopesdataiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].id](#specdefinitionauthorizationsettingspoliciesscopesdataid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].name](#specdefinitionauthorizationsettingspoliciesscopesdataname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources](#specdefinitionauthorizationsettingspoliciesscopesdataresources)|array||
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
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdatadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].iconUri](#specdefinitionauthorizationsettingspoliciesscopesdataiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].id](#specdefinitionauthorizationsettingspoliciesscopesdataid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].name](#specdefinitionauthorizationsettingspoliciesscopesdataname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources](#specdefinitionauthorizationsettingspoliciesscopesdataresources)|array||
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
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
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
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].type](#specdefinitionauthorizationsettingspoliciestype)|string||
|[spec.definition.authorizationSettings.policies[].config](#specdefinitionauthorizationsettingspoliciesconfig)|object||
|[spec.definition.authorizationSettings.policies[].decisionStrategy](#specdefinitionauthorizationsettingspoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.policies[].description](#specdefinitionauthorizationsettingspoliciesdescription)|string||
|[spec.definition.authorizationSettings.policies[].id](#specdefinitionauthorizationsettingspoliciesid)|string||
|[spec.definition.authorizationSettings.policies[].logic](#specdefinitionauthorizationsettingspolicieslogic)|string||
|[spec.definition.authorizationSettings.policies[].name](#specdefinitionauthorizationsettingspoliciesname)|string||
|[spec.definition.authorizationSettings.policies[].owner](#specdefinitionauthorizationsettingspoliciesowner)|string||
|[spec.definition.authorizationSettings.policies[].policies](#specdefinitionauthorizationsettingspoliciespolicies)|array||
|[spec.definition.authorizationSettings.policies[].policies[]](#specdefinitionauthorizationsettingspoliciespolicies)|string||
|[spec.definition.authorizationSettings.policies[].policies[]](#specdefinitionauthorizationsettingspoliciespolicies)|string||
|[spec.definition.authorizationSettings.policies[].resources](#specdefinitionauthorizationsettingspoliciesresources)|array||
|[spec.definition.authorizationSettings.policies[].resources[]](#specdefinitionauthorizationsettingspoliciesresources)|string||
|[spec.definition.authorizationSettings.policies[].resources[]](#specdefinitionauthorizationsettingspoliciesresources)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData](#specdefinitionauthorizationsettingspoliciesresourcesdata)|array||
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
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].type](#specdefinitionauthorizationsettingspoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uri](#specdefinitionauthorizationsettingspoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[]._id](#specdefinitionauthorizationsettingspoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingspoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingspoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].name](#specdefinitionauthorizationsettingspoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner](#specdefinitionauthorizationsettingspoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingspoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingspoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingspoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingspoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].type](#specdefinitionauthorizationsettingspoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uri](#specdefinitionauthorizationsettingspoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|string||
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
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].type](#specdefinitionauthorizationsettingspoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uri](#specdefinitionauthorizationsettingspoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[]._id](#specdefinitionauthorizationsettingspoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingspoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingspoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].name](#specdefinitionauthorizationsettingspoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner](#specdefinitionauthorizationsettingspoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingspoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingspoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingspoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingspoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].type](#specdefinitionauthorizationsettingspoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uri](#specdefinitionauthorizationsettingspoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[]._id](#specdefinitionauthorizationsettingspoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingspoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingspoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].name](#specdefinitionauthorizationsettingspoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner](#specdefinitionauthorizationsettingspoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingspoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingspoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingspoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingspoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingspoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingspoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopes)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesuma)|object||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaiconuri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaid)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name](#specdefinitionauthorizationsettingspoliciesresourcesdatascopesumaname)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].type](#specdefinitionauthorizationsettingspoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uri](#specdefinitionauthorizationsettingspoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingspoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.policies[].scopes](#specdefinitionauthorizationsettingspoliciesscopes)|array||
|[spec.definition.authorizationSettings.policies[].scopes[]](#specdefinitionauthorizationsettingspoliciesscopes)|string||
|[spec.definition.authorizationSettings.policies[].scopes[]](#specdefinitionauthorizationsettingspoliciesscopes)|string||
|[spec.definition.authorizationSettings.policies[].scopesData](#specdefinitionauthorizationsettingspoliciesscopesdata)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[]](#specdefinitionauthorizationsettingspoliciesscopesdata)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdatadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].iconUri](#specdefinitionauthorizationsettingspoliciesscopesdataiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].id](#specdefinitionauthorizationsettingspoliciesscopesdataid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].name](#specdefinitionauthorizationsettingspoliciesscopesdataname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources](#specdefinitionauthorizationsettingspoliciesscopesdataresources)|array||
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
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdatadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].iconUri](#specdefinitionauthorizationsettingspoliciesscopesdataiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].id](#specdefinitionauthorizationsettingspoliciesscopesdataid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].name](#specdefinitionauthorizationsettingspoliciesscopesdataname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources](#specdefinitionauthorizationsettingspoliciesscopesdataresources)|array||
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
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
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
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[]](#specdefinitionauthorizationsettingspoliciesscopesdata)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdatadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].iconUri](#specdefinitionauthorizationsettingspoliciesscopesdataiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].id](#specdefinitionauthorizationsettingspoliciesscopesdataid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].name](#specdefinitionauthorizationsettingspoliciesscopesdataname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources](#specdefinitionauthorizationsettingspoliciesscopesdataresources)|array||
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
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdatadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].iconUri](#specdefinitionauthorizationsettingspoliciesscopesdataiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].id](#specdefinitionauthorizationsettingspoliciesscopesdataid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].name](#specdefinitionauthorizationsettingspoliciesscopesdataname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources](#specdefinitionauthorizationsettingspoliciesscopesdataresources)|array||
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
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
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
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdatadisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].iconUri](#specdefinitionauthorizationsettingspoliciesscopesdataiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].id](#specdefinitionauthorizationsettingspoliciesscopesdataid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].name](#specdefinitionauthorizationsettingspoliciesscopesdataname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources](#specdefinitionauthorizationsettingspoliciesscopesdataresources)|array||
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
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
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
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
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
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesattributes)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesiconuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesname)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesowner)|object||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownerid)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownername)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].type](#specdefinitionauthorizationsettingspoliciesscopesdataresourcestype)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuri)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|array||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]](#specdefinitionauthorizationsettingspoliciesscopesdataresourcesuris)|string||
|[spec.definition.authorizationSettings.policies[].type](#specdefinitionauthorizationsettingspoliciestype)|string||
|[spec.definition.authorizationSettings.policyEnforcementMode](#specdefinitionauthorizationsettingspolicyenforcementmode)|string||
|[spec.definition.authorizationSettings.resources](#specdefinitionauthorizationsettingsresources)|array||
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
|[spec.definition.authorizationSettings.resources[].scopes](#specdefinitionauthorizationsettingsresourcesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopes)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].displayName](#specdefinitionauthorizationsettingsresourcesscopesdisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].id](#specdefinitionauthorizationsettingsresourcesscopesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].name](#specdefinitionauthorizationsettingsresourcesscopesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies](#specdefinitionauthorizationsettingsresourcesscopespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].displayName](#specdefinitionauthorizationsettingsresourcesscopesdisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].id](#specdefinitionauthorizationsettingsresourcesscopesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].name](#specdefinitionauthorizationsettingsresourcesscopesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies](#specdefinitionauthorizationsettingsresourcesscopespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma](#specdefinitionauthorizationsettingsresourcesscopesuma)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[]](#specdefinitionauthorizationsettingsresourcesscopesuma)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].displayName](#specdefinitionauthorizationsettingsresourcesscopesumadisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesumaiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].id](#specdefinitionauthorizationsettingsresourcesscopesumaid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].name](#specdefinitionauthorizationsettingsresourcesscopesumaname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].displayName](#specdefinitionauthorizationsettingsresourcesscopesumadisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesumaiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].id](#specdefinitionauthorizationsettingsresourcesscopesumaid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].name](#specdefinitionauthorizationsettingsresourcesscopesumaname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].type](#specdefinitionauthorizationsettingsresourcestype)|string||
|[spec.definition.authorizationSettings.resources[].uri](#specdefinitionauthorizationsettingsresourcesuri)|string||
|[spec.definition.authorizationSettings.resources[].uris](#specdefinitionauthorizationsettingsresourcesuris)|array||
|[spec.definition.authorizationSettings.resources[].uris[]](#specdefinitionauthorizationsettingsresourcesuris)|string||
|[spec.definition.authorizationSettings.resources[]._id](#specdefinitionauthorizationsettingsresourcesid)|string||
|[spec.definition.authorizationSettings.resources[].attributes](#specdefinitionauthorizationsettingsresourcesattributes)|object||
|[spec.definition.authorizationSettings.resources[].displayName](#specdefinitionauthorizationsettingsresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.resources[].icon_uri](#specdefinitionauthorizationsettingsresourcesiconuri)|string||
|[spec.definition.authorizationSettings.resources[].name](#specdefinitionauthorizationsettingsresourcesname)|string||
|[spec.definition.authorizationSettings.resources[].owner](#specdefinitionauthorizationsettingsresourcesowner)|object||
|[spec.definition.authorizationSettings.resources[].owner.id](#specdefinitionauthorizationsettingsresourcesownerid)|string||
|[spec.definition.authorizationSettings.resources[].owner.name](#specdefinitionauthorizationsettingsresourcesownername)|string||
|[spec.definition.authorizationSettings.resources[].owner.id](#specdefinitionauthorizationsettingsresourcesownerid)|string||
|[spec.definition.authorizationSettings.resources[].owner.name](#specdefinitionauthorizationsettingsresourcesownername)|string||
|[spec.definition.authorizationSettings.resources[].ownerManagedAccess](#specdefinitionauthorizationsettingsresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.resources[].scopes](#specdefinitionauthorizationsettingsresourcesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopes)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].displayName](#specdefinitionauthorizationsettingsresourcesscopesdisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].id](#specdefinitionauthorizationsettingsresourcesscopesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].name](#specdefinitionauthorizationsettingsresourcesscopesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies](#specdefinitionauthorizationsettingsresourcesscopespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].displayName](#specdefinitionauthorizationsettingsresourcesscopesdisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].id](#specdefinitionauthorizationsettingsresourcesscopesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].name](#specdefinitionauthorizationsettingsresourcesscopesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies](#specdefinitionauthorizationsettingsresourcesscopespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopes)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].displayName](#specdefinitionauthorizationsettingsresourcesscopesdisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].id](#specdefinitionauthorizationsettingsresourcesscopesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].name](#specdefinitionauthorizationsettingsresourcesscopesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies](#specdefinitionauthorizationsettingsresourcesscopespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].displayName](#specdefinitionauthorizationsettingsresourcesscopesdisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].id](#specdefinitionauthorizationsettingsresourcesscopesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].name](#specdefinitionauthorizationsettingsresourcesscopesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies](#specdefinitionauthorizationsettingsresourcesscopespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].displayName](#specdefinitionauthorizationsettingsresourcesscopesdisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].id](#specdefinitionauthorizationsettingsresourcesscopesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].name](#specdefinitionauthorizationsettingsresourcesscopesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies](#specdefinitionauthorizationsettingsresourcesscopespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopespoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopespoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopes[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopespoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma](#specdefinitionauthorizationsettingsresourcesscopesuma)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[]](#specdefinitionauthorizationsettingsresourcesscopesuma)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].displayName](#specdefinitionauthorizationsettingsresourcesscopesumadisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesumaiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].id](#specdefinitionauthorizationsettingsresourcesscopesumaid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].name](#specdefinitionauthorizationsettingsresourcesscopesumaname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].displayName](#specdefinitionauthorizationsettingsresourcesscopesumadisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesumaiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].id](#specdefinitionauthorizationsettingsresourcesscopesumaid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].name](#specdefinitionauthorizationsettingsresourcesscopesumaname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[]](#specdefinitionauthorizationsettingsresourcesscopesuma)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].displayName](#specdefinitionauthorizationsettingsresourcesscopesumadisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesumaiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].id](#specdefinitionauthorizationsettingsresourcesscopesumaid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].name](#specdefinitionauthorizationsettingsresourcesscopesumaname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].displayName](#specdefinitionauthorizationsettingsresourcesscopesumadisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesumaiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].id](#specdefinitionauthorizationsettingsresourcesscopesumaid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].name](#specdefinitionauthorizationsettingsresourcesscopesumaname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].displayName](#specdefinitionauthorizationsettingsresourcesscopesumadisplayname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].iconUri](#specdefinitionauthorizationsettingsresourcesscopesumaiconuri)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].id](#specdefinitionauthorizationsettingsresourcesscopesumaid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].name](#specdefinitionauthorizationsettingsresourcesscopesumaname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapolicies)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesconfig)|object||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesdescription)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesid)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic](#specdefinitionauthorizationsettingsresourcesscopesumapolicieslogic)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesname)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesowner)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciespolicies)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesresources)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|array||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]](#specdefinitionauthorizationsettingsresourcesscopesumapoliciesscopes)|string||
|[spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type](#specdefinitionauthorizationsettingsresourcesscopesumapoliciestype)|string||
|[spec.definition.authorizationSettings.resources[].type](#specdefinitionauthorizationsettingsresourcestype)|string||
|[spec.definition.authorizationSettings.resources[].uri](#specdefinitionauthorizationsettingsresourcesuri)|string||
|[spec.definition.authorizationSettings.resources[].uris](#specdefinitionauthorizationsettingsresourcesuris)|array||
|[spec.definition.authorizationSettings.resources[].uris[]](#specdefinitionauthorizationsettingsresourcesuris)|string||
|[spec.definition.authorizationSettings.resources[].uris[]](#specdefinitionauthorizationsettingsresourcesuris)|string||
|[spec.definition.authorizationSettings.scopes](#specdefinitionauthorizationsettingsscopes)|array||
|[spec.definition.authorizationSettings.scopes[]](#specdefinitionauthorizationsettingsscopes)|object||
|[spec.definition.authorizationSettings.scopes[].displayName](#specdefinitionauthorizationsettingsscopesdisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].iconUri](#specdefinitionauthorizationsettingsscopesiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].id](#specdefinitionauthorizationsettingsscopesid)|string||
|[spec.definition.authorizationSettings.scopes[].name](#specdefinitionauthorizationsettingsscopesname)|string||
|[spec.definition.authorizationSettings.scopes[].policies](#specdefinitionauthorizationsettingsscopespolicies)|array||
|[spec.definition.authorizationSettings.scopes[].policies[]](#specdefinitionauthorizationsettingsscopespolicies)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].config](#specdefinitionauthorizationsettingsscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].description](#specdefinitionauthorizationsettingsscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].id](#specdefinitionauthorizationsettingsscopespoliciesid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].logic](#specdefinitionauthorizationsettingsscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].name](#specdefinitionauthorizationsettingsscopespoliciesname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].owner](#specdefinitionauthorizationsettingsscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].policies](#specdefinitionauthorizationsettingsscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resources](#specdefinitionauthorizationsettingsscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData](#specdefinitionauthorizationsettingsscopespoliciesresourcesdata)|array||
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
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes](#specdefinitionauthorizationsettingsscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].type](#specdefinitionauthorizationsettingsscopespoliciestype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].config](#specdefinitionauthorizationsettingsscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].description](#specdefinitionauthorizationsettingsscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].id](#specdefinitionauthorizationsettingsscopespoliciesid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].logic](#specdefinitionauthorizationsettingsscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].name](#specdefinitionauthorizationsettingsscopespoliciesname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].owner](#specdefinitionauthorizationsettingsscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].policies](#specdefinitionauthorizationsettingsscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resources](#specdefinitionauthorizationsettingsscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData](#specdefinitionauthorizationsettingsscopespoliciesresourcesdata)|array||
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
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
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
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes](#specdefinitionauthorizationsettingsscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].type](#specdefinitionauthorizationsettingsscopespoliciestype)|string||
|[spec.definition.authorizationSettings.scopes[].resources](#specdefinitionauthorizationsettingsscopesresources)|array||
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
|[spec.definition.authorizationSettings.scopes[].resources[].uris](#specdefinitionauthorizationsettingsscopesresourcesuris)|array||
|[spec.definition.authorizationSettings.scopes[].resources[].uris[]](#specdefinitionauthorizationsettingsscopesresourcesuris)|string||
|[spec.definition.authorizationSettings.scopes[].resources[]._id](#specdefinitionauthorizationsettingsscopesresourcesid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].attributes](#specdefinitionauthorizationsettingsscopesresourcesattributes)|object||
|[spec.definition.authorizationSettings.scopes[].resources[].displayName](#specdefinitionauthorizationsettingsscopesresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].icon_uri](#specdefinitionauthorizationsettingsscopesresourcesiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].name](#specdefinitionauthorizationsettingsscopesresourcesname)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner](#specdefinitionauthorizationsettingsscopesresourcesowner)|object||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.id](#specdefinitionauthorizationsettingsscopesresourcesownerid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.name](#specdefinitionauthorizationsettingsscopesresourcesownername)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.id](#specdefinitionauthorizationsettingsscopesresourcesownerid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.name](#specdefinitionauthorizationsettingsscopesresourcesownername)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopesresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].resources[].type](#specdefinitionauthorizationsettingsscopesresourcestype)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uri](#specdefinitionauthorizationsettingsscopesresourcesuri)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uris](#specdefinitionauthorizationsettingsscopesresourcesuris)|array||
|[spec.definition.authorizationSettings.scopes[].resources[].uris[]](#specdefinitionauthorizationsettingsscopesresourcesuris)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uris[]](#specdefinitionauthorizationsettingsscopesresourcesuris)|string||
|[spec.definition.authorizationSettings.scopes[].displayName](#specdefinitionauthorizationsettingsscopesdisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].iconUri](#specdefinitionauthorizationsettingsscopesiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].id](#specdefinitionauthorizationsettingsscopesid)|string||
|[spec.definition.authorizationSettings.scopes[].name](#specdefinitionauthorizationsettingsscopesname)|string||
|[spec.definition.authorizationSettings.scopes[].policies](#specdefinitionauthorizationsettingsscopespolicies)|array||
|[spec.definition.authorizationSettings.scopes[].policies[]](#specdefinitionauthorizationsettingsscopespolicies)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].config](#specdefinitionauthorizationsettingsscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].description](#specdefinitionauthorizationsettingsscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].id](#specdefinitionauthorizationsettingsscopespoliciesid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].logic](#specdefinitionauthorizationsettingsscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].name](#specdefinitionauthorizationsettingsscopespoliciesname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].owner](#specdefinitionauthorizationsettingsscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].policies](#specdefinitionauthorizationsettingsscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resources](#specdefinitionauthorizationsettingsscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData](#specdefinitionauthorizationsettingsscopespoliciesresourcesdata)|array||
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
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes](#specdefinitionauthorizationsettingsscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].type](#specdefinitionauthorizationsettingsscopespoliciestype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].config](#specdefinitionauthorizationsettingsscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].description](#specdefinitionauthorizationsettingsscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].id](#specdefinitionauthorizationsettingsscopespoliciesid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].logic](#specdefinitionauthorizationsettingsscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].name](#specdefinitionauthorizationsettingsscopespoliciesname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].owner](#specdefinitionauthorizationsettingsscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].policies](#specdefinitionauthorizationsettingsscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resources](#specdefinitionauthorizationsettingsscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData](#specdefinitionauthorizationsettingsscopespoliciesresourcesdata)|array||
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
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
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
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes](#specdefinitionauthorizationsettingsscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].type](#specdefinitionauthorizationsettingsscopespoliciestype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[]](#specdefinitionauthorizationsettingsscopespolicies)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].config](#specdefinitionauthorizationsettingsscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].description](#specdefinitionauthorizationsettingsscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].id](#specdefinitionauthorizationsettingsscopespoliciesid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].logic](#specdefinitionauthorizationsettingsscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].name](#specdefinitionauthorizationsettingsscopespoliciesname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].owner](#specdefinitionauthorizationsettingsscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].policies](#specdefinitionauthorizationsettingsscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resources](#specdefinitionauthorizationsettingsscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData](#specdefinitionauthorizationsettingsscopespoliciesresourcesdata)|array||
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
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes](#specdefinitionauthorizationsettingsscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].type](#specdefinitionauthorizationsettingsscopespoliciestype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].config](#specdefinitionauthorizationsettingsscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].description](#specdefinitionauthorizationsettingsscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].id](#specdefinitionauthorizationsettingsscopespoliciesid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].logic](#specdefinitionauthorizationsettingsscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].name](#specdefinitionauthorizationsettingsscopespoliciesname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].owner](#specdefinitionauthorizationsettingsscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].policies](#specdefinitionauthorizationsettingsscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resources](#specdefinitionauthorizationsettingsscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData](#specdefinitionauthorizationsettingsscopespoliciesresourcesdata)|array||
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
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
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
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes](#specdefinitionauthorizationsettingsscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].type](#specdefinitionauthorizationsettingsscopespoliciestype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].config](#specdefinitionauthorizationsettingsscopespoliciesconfig)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy](#specdefinitionauthorizationsettingsscopespoliciesdecisionstrategy)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].description](#specdefinitionauthorizationsettingsscopespoliciesdescription)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].id](#specdefinitionauthorizationsettingsscopespoliciesid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].logic](#specdefinitionauthorizationsettingsscopespolicieslogic)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].name](#specdefinitionauthorizationsettingsscopespoliciesname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].owner](#specdefinitionauthorizationsettingsscopespoliciesowner)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].policies](#specdefinitionauthorizationsettingsscopespoliciespolicies)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].policies[]](#specdefinitionauthorizationsettingsscopespoliciespolicies)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resources](#specdefinitionauthorizationsettingsscopespoliciesresources)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resources[]](#specdefinitionauthorizationsettingsscopespoliciesresources)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData](#specdefinitionauthorizationsettingsscopespoliciesresourcesdata)|array||
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
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
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
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
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
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataattributes)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatadisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataname)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataowner)|object||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownerid)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownername)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopespoliciesresourcesdataownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatatype)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauri)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]](#specdefinitionauthorizationsettingsscopespoliciesresourcesdatauris)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes](#specdefinitionauthorizationsettingsscopespoliciesscopes)|array||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].scopes[]](#specdefinitionauthorizationsettingsscopespoliciesscopes)|string||
|[spec.definition.authorizationSettings.scopes[].policies[].type](#specdefinitionauthorizationsettingsscopespoliciestype)|string||
|[spec.definition.authorizationSettings.scopes[].resources](#specdefinitionauthorizationsettingsscopesresources)|array||
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
|[spec.definition.authorizationSettings.scopes[].resources[].uris](#specdefinitionauthorizationsettingsscopesresourcesuris)|array||
|[spec.definition.authorizationSettings.scopes[].resources[].uris[]](#specdefinitionauthorizationsettingsscopesresourcesuris)|string||
|[spec.definition.authorizationSettings.scopes[].resources[]._id](#specdefinitionauthorizationsettingsscopesresourcesid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].attributes](#specdefinitionauthorizationsettingsscopesresourcesattributes)|object||
|[spec.definition.authorizationSettings.scopes[].resources[].displayName](#specdefinitionauthorizationsettingsscopesresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].icon_uri](#specdefinitionauthorizationsettingsscopesresourcesiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].name](#specdefinitionauthorizationsettingsscopesresourcesname)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner](#specdefinitionauthorizationsettingsscopesresourcesowner)|object||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.id](#specdefinitionauthorizationsettingsscopesresourcesownerid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.name](#specdefinitionauthorizationsettingsscopesresourcesownername)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.id](#specdefinitionauthorizationsettingsscopesresourcesownerid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.name](#specdefinitionauthorizationsettingsscopesresourcesownername)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopesresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].resources[].type](#specdefinitionauthorizationsettingsscopesresourcestype)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uri](#specdefinitionauthorizationsettingsscopesresourcesuri)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uris](#specdefinitionauthorizationsettingsscopesresourcesuris)|array||
|[spec.definition.authorizationSettings.scopes[].resources[].uris[]](#specdefinitionauthorizationsettingsscopesresourcesuris)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uris[]](#specdefinitionauthorizationsettingsscopesresourcesuris)|string||
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
|[spec.definition.authorizationSettings.scopes[].resources[].uris](#specdefinitionauthorizationsettingsscopesresourcesuris)|array||
|[spec.definition.authorizationSettings.scopes[].resources[].uris[]](#specdefinitionauthorizationsettingsscopesresourcesuris)|string||
|[spec.definition.authorizationSettings.scopes[].resources[]._id](#specdefinitionauthorizationsettingsscopesresourcesid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].attributes](#specdefinitionauthorizationsettingsscopesresourcesattributes)|object||
|[spec.definition.authorizationSettings.scopes[].resources[].displayName](#specdefinitionauthorizationsettingsscopesresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].icon_uri](#specdefinitionauthorizationsettingsscopesresourcesiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].name](#specdefinitionauthorizationsettingsscopesresourcesname)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner](#specdefinitionauthorizationsettingsscopesresourcesowner)|object||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.id](#specdefinitionauthorizationsettingsscopesresourcesownerid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.name](#specdefinitionauthorizationsettingsscopesresourcesownername)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.id](#specdefinitionauthorizationsettingsscopesresourcesownerid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.name](#specdefinitionauthorizationsettingsscopesresourcesownername)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopesresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].resources[].type](#specdefinitionauthorizationsettingsscopesresourcestype)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uri](#specdefinitionauthorizationsettingsscopesresourcesuri)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uris](#specdefinitionauthorizationsettingsscopesresourcesuris)|array||
|[spec.definition.authorizationSettings.scopes[].resources[].uris[]](#specdefinitionauthorizationsettingsscopesresourcesuris)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uris[]](#specdefinitionauthorizationsettingsscopesresourcesuris)|string||
|[spec.definition.authorizationSettings.scopes[].resources[]._id](#specdefinitionauthorizationsettingsscopesresourcesid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].attributes](#specdefinitionauthorizationsettingsscopesresourcesattributes)|object||
|[spec.definition.authorizationSettings.scopes[].resources[].displayName](#specdefinitionauthorizationsettingsscopesresourcesdisplayname)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].icon_uri](#specdefinitionauthorizationsettingsscopesresourcesiconuri)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].name](#specdefinitionauthorizationsettingsscopesresourcesname)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner](#specdefinitionauthorizationsettingsscopesresourcesowner)|object||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.id](#specdefinitionauthorizationsettingsscopesresourcesownerid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.name](#specdefinitionauthorizationsettingsscopesresourcesownername)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.id](#specdefinitionauthorizationsettingsscopesresourcesownerid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.name](#specdefinitionauthorizationsettingsscopesresourcesownername)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.id](#specdefinitionauthorizationsettingsscopesresourcesownerid)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].owner.name](#specdefinitionauthorizationsettingsscopesresourcesownername)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].ownerManagedAccess](#specdefinitionauthorizationsettingsscopesresourcesownermanagedaccess)|boolean||
|[spec.definition.authorizationSettings.scopes[].resources[].type](#specdefinitionauthorizationsettingsscopesresourcestype)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uri](#specdefinitionauthorizationsettingsscopesresourcesuri)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uris](#specdefinitionauthorizationsettingsscopesresourcesuris)|array||
|[spec.definition.authorizationSettings.scopes[].resources[].uris[]](#specdefinitionauthorizationsettingsscopesresourcesuris)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uris[]](#specdefinitionauthorizationsettingsscopesresourcesuris)|string||
|[spec.definition.authorizationSettings.scopes[].resources[].uris[]](#specdefinitionauthorizationsettingsscopesresourcesuris)|string||
|[spec.definition.baseUrl](#specdefinitionbaseurl)|string||
|[spec.definition.bearerOnly](#specdefinitionbeareronly)|boolean||
|[spec.definition.clientAuthenticatorType](#specdefinitionclientauthenticatortype)|string||
|[spec.definition.clientId](#specdefinitionclientid)|string||
|[spec.definition.clientTemplate](#specdefinitionclienttemplate)|string||
|[spec.definition.consentRequired](#specdefinitionconsentrequired)|boolean||
|[spec.definition.defaultClientScopes](#specdefinitiondefaultclientscopes)|array||
|[spec.definition.defaultClientScopes[]](#specdefinitiondefaultclientscopes)|string||
|[spec.definition.defaultRoles](#specdefinitiondefaultroles)|array||
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
|[spec.definition.optionalClientScopes](#specdefinitionoptionalclientscopes)|array||
|[spec.definition.optionalClientScopes[]](#specdefinitionoptionalclientscopes)|string||
|[spec.definition.origin](#specdefinitionorigin)|string||
|[spec.definition.protocol](#specdefinitionprotocol)|string||
|[spec.definition.protocolMappers](#specdefinitionprotocolmappers)|array||
|[spec.definition.protocolMappers[]](#specdefinitionprotocolmappers)|object||
|[spec.definition.protocolMappers[].config](#specdefinitionprotocolmappersconfig)|object||
|[spec.definition.protocolMappers[].consentRequired](#specdefinitionprotocolmappersconsentrequired)|boolean||
|[spec.definition.protocolMappers[].consentText](#specdefinitionprotocolmappersconsenttext)|string||
|[spec.definition.protocolMappers[].id](#specdefinitionprotocolmappersid)|string||
|[spec.definition.protocolMappers[].name](#specdefinitionprotocolmappersname)|string||
|[spec.definition.protocolMappers[].protocol](#specdefinitionprotocolmappersprotocol)|string||
|[spec.definition.protocolMappers[].protocolMapper](#specdefinitionprotocolmappersprotocolmapper)|string||
|[spec.definition.protocolMappers[].config](#specdefinitionprotocolmappersconfig)|object||
|[spec.definition.protocolMappers[].consentRequired](#specdefinitionprotocolmappersconsentrequired)|boolean||
|[spec.definition.protocolMappers[].consentText](#specdefinitionprotocolmappersconsenttext)|string||
|[spec.definition.protocolMappers[].id](#specdefinitionprotocolmappersid)|string||
|[spec.definition.protocolMappers[].name](#specdefinitionprotocolmappersname)|string||
|[spec.definition.protocolMappers[].protocol](#specdefinitionprotocolmappersprotocol)|string||
|[spec.definition.protocolMappers[].protocolMapper](#specdefinitionprotocolmappersprotocolmapper)|string||
|[spec.definition.publicClient](#specdefinitionpublicclient)|boolean||
|[spec.definition.redirectUris](#specdefinitionredirecturis)|array||
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
|[spec.definition.webOrigins](#specdefinitionweborigins)|array||
|[spec.definition.webOrigins[]](#specdefinitionweborigins)|string||
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

the KeycloakClient resource

### spec.clientSecret

Type: object

*missing*

### spec.clientSecret.clientIdKey

Type: string

*missing*

### spec.clientSecret.clientSecretKey

Type: string

*missing*

### spec.clientSecret.secretName

Type: string

*missing*

### spec.definition

Type: object

#### Validations

|Rule|Error Message|
|:---|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

*missing*

### spec.definition.access

Type: object

*missing*

### spec.definition.adminUrl

Type: string

*missing*

### spec.definition.alwaysDisplayInConsole

Type: boolean

*missing*

### spec.definition.attributes

Type: object

*missing*

### spec.definition.authenticationFlowBindingOverrides

Type: object

*missing*

### spec.definition.authorizationServicesEnabled

Type: boolean

*missing*

### spec.definition.authorizationSettings

Type: object

*missing*

### spec.definition.authorizationSettings.allowRemoteResourceManagement

Type: boolean

*missing*

### spec.definition.authorizationSettings.clientId

Type: string

*missing*

### spec.definition.authorizationSettings.decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.id

Type: string

*missing*

### spec.definition.authorizationSettings.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies

Type: array

*missing*

### spec.definition.authorizationSettings.policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].scopesData[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.policyEnforcementMode

Type: string

*missing*

### spec.definition.authorizationSettings.resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.resources[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.resources[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].iconUri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].config

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].description

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].logic

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].owner

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].resourcesData[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].policies[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[]

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[]._id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].attributes

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].displayName

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].icon_uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner

Type: object

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.id

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].owner.name

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].ownerManagedAccess

Type: boolean

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].type

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uri

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris

Type: array

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris[]

Type: string

*missing*

### spec.definition.authorizationSettings.scopes[].resources[].uris[]

Type: string

*missing*

### spec.definition.baseUrl

Type: string

*missing*

### spec.definition.bearerOnly

Type: boolean

*missing*

### spec.definition.clientAuthenticatorType

Type: string

*missing*

### spec.definition.clientId

Type: string

*missing*

### spec.definition.clientTemplate

Type: string

*missing*

### spec.definition.consentRequired

Type: boolean

*missing*

### spec.definition.defaultClientScopes

Type: array

*missing*

### spec.definition.defaultClientScopes[]

Type: string

*missing*

### spec.definition.defaultRoles

Type: array

*missing*

### spec.definition.defaultRoles[]

Type: string

*missing*

### spec.definition.description

Type: string

*missing*

### spec.definition.directAccessGrantsEnabled

Type: boolean

*missing*

### spec.definition.directGrantsOnly

Type: boolean

*missing*

### spec.definition.enabled

Type: boolean

*missing*

### spec.definition.frontchannelLogout

Type: boolean

*missing*

### spec.definition.fullScopeAllowed

Type: boolean

*missing*

### spec.definition.id

Type: string

#### Validations

|Rule|Error Message|
|:---|:------------|
|self == oldSelf|Value is immutable|

*missing*

### spec.definition.implicitFlowEnabled

Type: boolean

*missing*

### spec.definition.name

Type: string

*missing*

### spec.definition.nodeReRegistrationTimeout

Type: integer

*missing*

### spec.definition.notBefore

Type: integer

*missing*

### spec.definition.optionalClientScopes

Type: array

*missing*

### spec.definition.optionalClientScopes[]

Type: string

*missing*

### spec.definition.origin

Type: string

*missing*

### spec.definition.protocol

Type: string

*missing*

### spec.definition.protocolMappers

Type: array

*missing*

### spec.definition.protocolMappers[]

Type: object

*missing*

### spec.definition.protocolMappers[].config

Type: object

*missing*

### spec.definition.protocolMappers[].consentRequired

Type: boolean

*missing*

### spec.definition.protocolMappers[].consentText

Type: string

*missing*

### spec.definition.protocolMappers[].id

Type: string

*missing*

### spec.definition.protocolMappers[].name

Type: string

*missing*

### spec.definition.protocolMappers[].protocol

Type: string

*missing*

### spec.definition.protocolMappers[].protocolMapper

Type: string

*missing*

### spec.definition.protocolMappers[].config

Type: object

*missing*

### spec.definition.protocolMappers[].consentRequired

Type: boolean

*missing*

### spec.definition.protocolMappers[].consentText

Type: string

*missing*

### spec.definition.protocolMappers[].id

Type: string

*missing*

### spec.definition.protocolMappers[].name

Type: string

*missing*

### spec.definition.protocolMappers[].protocol

Type: string

*missing*

### spec.definition.protocolMappers[].protocolMapper

Type: string

*missing*

### spec.definition.publicClient

Type: boolean

*missing*

### spec.definition.redirectUris

Type: array

*missing*

### spec.definition.redirectUris[]

Type: string

*missing*

### spec.definition.registeredNodes

Type: object

*missing*

### spec.definition.registrationAccessToken

Type: string

*missing*

### spec.definition.rootUrl

Type: string

*missing*

### spec.definition.secret

Type: string

*missing*

### spec.definition.serviceAccountsEnabled

Type: boolean

*missing*

### spec.definition.standardFlowEnabled

Type: boolean

*missing*

### spec.definition.surrogateAuthRequired

Type: boolean

*missing*

### spec.definition.type

Type: string

*missing*

### spec.definition.useTemplateConfig

Type: boolean

*missing*

### spec.definition.useTemplateMappers

Type: boolean

*missing*

### spec.definition.useTemplateScope

Type: boolean

*missing*

### spec.definition.webOrigins

Type: array

*missing*

### spec.definition.webOrigins[]

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