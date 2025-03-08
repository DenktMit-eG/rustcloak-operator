# KeycloakClient

## v1beta1

resource to define a Client within a [KeycloakRealm](./keycloakrealm.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clientSecret](#specclientsecret)|object||
|[spec.clientSecret.clientIdKey](#specclientsecretclientidkey)|string||
|[spec.clientSecret.clientSecretKey](#specclientsecretclientsecretkey)|string||
|[spec.clientSecret.secretName](#specclientsecretsecretname)|string|✅|
|[spec.clusterRealmRef](#specclusterrealmref)|string||
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.access](#specdefinitionaccess)|object||
|[spec.definition.adminUrl](#specdefinitionadminurl)|string||
|[spec.definition.alwaysDisplayInConsole](#specdefinitionalwaysdisplayinconsole)|boolean||
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.attributes.access.token.lifespan](#specdefinitionattributesaccesstokenlifespan)|string||
|[spec.definition.attributes.access.token.signed.response.alg](#specdefinitionattributesaccesstokensignedresponsealg)|string||
|[spec.definition.attributes.authorization.encrypted.response.alg](#specdefinitionattributesauthorizationencryptedresponsealg)|string||
|[spec.definition.attributes.authorization.encrypted.response.enc](#specdefinitionattributesauthorizationencryptedresponseenc)|string||
|[spec.definition.attributes.authorization.signed.response.alg](#specdefinitionattributesauthorizationsignedresponsealg)|string||
|[spec.definition.attributes.client.offline.session.idle.timeout](#specdefinitionattributesclientofflinesessionidletimeout)|string||
|[spec.definition.attributes.client.offline.session.max.lifespan](#specdefinitionattributesclientofflinesessionmaxlifespan)|string||
|[spec.definition.attributes.client.session.idle.timeout](#specdefinitionattributesclientsessionidletimeout)|string||
|[spec.definition.attributes.client.session.max.lifespan](#specdefinitionattributesclientsessionmaxlifespan)|string||
|[spec.definition.attributes.client_credentials.use_refresh_token](#specdefinitionattributesclientcredentialsuserefreshtoken)|string||
|[spec.definition.attributes.exclude.session.state.from.auth.response](#specdefinitionattributesexcludesessionstatefromauthresponse)|string||
|[spec.definition.attributes.id.token.encrypted.response.alg](#specdefinitionattributesidtokenencryptedresponsealg)|string||
|[spec.definition.attributes.id.token.encrypted.response.enc](#specdefinitionattributesidtokenencryptedresponseenc)|string||
|[spec.definition.attributes.id.token.signed.response.alg](#specdefinitionattributesidtokensignedresponsealg)|string||
|[spec.definition.attributes.logoUri](#specdefinitionattributeslogouri)|string||
|[spec.definition.attributes.pkce.code.challenge.method](#specdefinitionattributespkcecodechallengemethod)|string||
|[spec.definition.attributes.policyUri](#specdefinitionattributespolicyuri)|string||
|[spec.definition.attributes.post.logout.redirect.uris](#specdefinitionattributespostlogoutredirecturis)|string||
|[spec.definition.attributes.request.object.encryption.alg](#specdefinitionattributesrequestobjectencryptionalg)|string||
|[spec.definition.attributes.request.object.encryption.enc](#specdefinitionattributesrequestobjectencryptionenc)|string||
|[spec.definition.attributes.request.object.required](#specdefinitionattributesrequestobjectrequired)|string||
|[spec.definition.attributes.request.object.signature.alg](#specdefinitionattributesrequestobjectsignaturealg)|string||
|[spec.definition.attributes.require.pushed.authorization.requests](#specdefinitionattributesrequirepushedauthorizationrequests)|string||
|[spec.definition.attributes.tls.client.certificate.bound.access.tokens](#specdefinitionattributestlsclientcertificateboundaccesstokens)|string||
|[spec.definition.attributes.token.endpoint.auth.signing.alg](#specdefinitionattributestokenendpointauthsigningalg)|string||
|[spec.definition.attributes.token.response.type.bearer.lower-case](#specdefinitionattributestokenresponsetypebearerlowercase)|string||
|[spec.definition.attributes.tosUri](#specdefinitionattributestosuri)|string||
|[spec.definition.attributes.use.refresh.tokens](#specdefinitionattributesuserefreshtokens)|string||
|[spec.definition.attributes.user.info.encrypted.response.alg](#specdefinitionattributesuserinfoencryptedresponsealg)|string||
|[spec.definition.attributes.user.info.encrypted.response.enc](#specdefinitionattributesuserinfoencryptedresponseenc)|string||
|[spec.definition.attributes.user.info.response.signature.alg](#specdefinitionattributesuserinforesponsesignaturealg)|string||
|[spec.definition.attributes.x509.allow.regex.pattern.comparison](#specdefinitionattributesx509allowregexpatterncomparison)|string||
|[spec.definition.attributes.x509.subjectdn](#specdefinitionattributesx509subjectdn)|string||
|[spec.definition.authenticationFlowBindingOverrides](#specdefinitionauthenticationflowbindingoverrides)|object||
|[spec.definition.authenticationFlowBindingOverrides.browser](#specdefinitionauthenticationflowbindingoverridesbrowser)|string||
|[spec.definition.authenticationFlowBindingOverrides.direct_grant](#specdefinitionauthenticationflowbindingoverridesdirectgrant)|string||
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
|[status.resourcePath](#statusresourcepath)|string||
|[status.status](#statusstatus)|string||

---

### spec

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clientSecret](#specclientsecret)|object||
|[clusterRealmRef](#specclusterrealmref)|string||
|[definition](#specdefinition)|object|✅|
|[options](#specoptions)|object||
|[patchFrom](#specpatchfrom)|object||
|[patchFrom2[]](#specpatchfrom2)|object||
|[realmRef](#specrealmref)|string||

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

ClientRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "access": { "type": "object", "additionalProperties": { "type": "boolean" } }, "adminUrl": { "title": "Admin URL", "description": "URL to the admin interface of the client. Set this if the client supports the adapter REST API. This REST API allows the auth server to push revocation policies and other administrative tasks. Usually this is set to the base URL of the client.", "type": "string" }, "alwaysDisplayInConsole": { "title": "Always display in UI", "description": "Always list this client in the Account UI, even if the user does not have an active session.", "type": "boolean" }, "attributes": { "type": "object", "properties": { "access.token.lifespan": { "title": "Access Token Lifespan", "description": "Max time before an access token is expired. This value is recommended to be short relative to the SSO timeout.", "type": "string", "pattern": "^[0-9]*$" }, "access.token.signed.response.alg": { "title": "Access token signature algorithm", "description": "JWA algorithm used for signing access tokens.", "type": "string" }, "authorization.encrypted.response.alg": { "title": "Authorization response encryption key management algorithm", "description": "JWA Algorithm used for key management in encrypting the authorization response when the response mode is jwt. This option is needed if you want encrypted authorization response. If left empty, the authorization response is just signed, but not encrypted.", "type": "string" }, "authorization.encrypted.response.enc": { "title": "Authorization response encryption content encryption algorithm", "description": "JWA Algorithm used for content encryption in encrypting the authorization response when the response mode is jwt. This option is needed if you want encrypted authorization response. If left empty, the authorization response is just signed, but not encrypted.", "type": "string" }, "authorization.signed.response.alg": { "title": "Authorization response signature algorithm", "description": "JWA algorithm used for signing authorization response tokens when the response mode is jwt.", "type": "string" }, "client.offline.session.idle.timeout": { "title": "Client Offline Session Idle", "description": "Time a client offline session is allowed to be idle before it expires. Offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Idle value.", "type": "string", "pattern": "^[0-9]*$" }, "client.offline.session.max.lifespan": { "title": "Client Offline Session Max", "description": "Max time before a client offline session is expired. If Offline Session Max Limited is enabled at realm level, offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Max value.", "type": "string", "pattern": "^[0-9]*$" }, "client.session.idle.timeout": { "title": "Client Session Idle", "description": "Time a client session is allowed to be idle before it expires. Tokens are invalidated when a client session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Idle value.", "type": "string", "pattern": "^[0-9]*$" }, "client.session.max.lifespan": { "title": "Client Session Max", "description": "Max time before a client session is expired. Tokens are invalidated when a session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Max value.", "type": "string", "pattern": "^[0-9]*$" }, "client_credentials.use_refresh_token": { "title": "Use refresh tokens for client credentials grant", "description": "If this is on, a refresh_token will be created and added to the token response if the client_credentials grant is used. The OAuth 2.0 RFC6749 Section 4.4.3 states that a refresh_token should not be generated when client_credentials grant is used. If this is off then no refresh_token will be generated and the associated user session will be removed.", "type": "string", "enum": [ "true", "false", "" ] }, "exclude.session.state.from.auth.response": { "title": "Exclude Session State From Authentication Response", "description": "If this is on, the parameter 'session_state' will not be included in OpenID Connect Authentication Response. It is useful if the client uses an older OIDC / OAuth2 adapter, which does not support the 'session_state' parameter.", "type": "string", "enum": [ "true", "false", "" ] }, "id.token.encrypted.response.alg": { "title": "ID token encryption key management algorithm", "description": "JWA Algorithm used for key management in encrypting ID tokens. This option is needed if you want encrypted ID tokens. If left empty, ID Tokens are just signed, but not encrypted.", "type": "string" }, "id.token.encrypted.response.enc": { "title": "ID token encryption content encryption algorithm", "description": "JWA Algorithm used for content encryption in encrypting ID tokens. This option is needed just if you want encrypted ID tokens. If left empty, ID Tokens are just signed, but not encrypted.", "type": "string" }, "id.token.signed.response.alg": { "title": "ID token signature algorithm", "description": "JWA algorithm used for signing ID tokens.", "type": "string" }, "logoUri": { "title": "Logo URL", "description": "URL that references a logo for the Client application", "type": "string" }, "pkce.code.challenge.method": { "title": "Proof Key for Code Exchange Code Challenge Method", "description": "Choose which code challenge method for PKCE is used. If not specified, keycloak does not applies PKCE to a client unless the client sends an authorization request with appropriate code challenge and code exchange method.", "type": "string" }, "policyUri": { "title": "Policy URL", "description": "URL that the Relying Party Client provides to the End-User to read about the how the profile data will be used", "type": "string" }, "post.logout.redirect.uris": { "title": "Valid post logout redirect URIs", "description": "Valid URI pattern a browser can redirect to after a successful login. Simple wildcards are allowed such as 'http://example.com/*'. Relative path can be specified too such as /my/relative/path/*. Relative paths are relative to the client root URL, or if none is specified the auth server root URL is used. For SAML, you must set valid URI patterns if you are relying on the consumer service URL embedded with the login request.", "type": "string" }, "request.object.encryption.alg": { "title": "Request object encryption algorithm", "description": "JWE algorithm, which client needs to use when sending OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', encryption is optional and any algorithm is allowed.", "type": "string" }, "request.object.encryption.enc": { "title": "Request object content encryption algorithm", "description": "JWE algorithm, which client needs to use when encrypting the content of the OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', any algorithm is allowed.", "type": "string" }, "request.object.required": { "title": "Request object required", "description": "Specifies if the client needs to provide a request object with their authorization requests, and what method they can use for this. If set to \"not required\", providing a request object is optional. In all other cases, providing a request object is mandatory. If set to \"request\", the request object must be provided by value. If set to \"request_uri\", the request object must be provided by reference. If set to \"request or request_uri\", either method can be used.", "type": "string" }, "request.object.signature.alg": { "title": "Request object signature algorithm", "description": "JWA algorithm, which client needs to use when sending OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', Request object can be signed by any algorithm (including 'none' ).", "type": "string" }, "require.pushed.authorization.requests": { "title": "Pushed authorization request required", "description": "Boolean parameter indicating whether the authorization server accepts authorization request data only via the pushed authorization request method.", "type": "string", "enum": [ "true", "false", "" ] }, "tls.client.certificate.bound.access.tokens": { "title": "OAuth 2.0 Mutual TLS Certificate Bound Access Tokens Enabled", "description": "This enables support for OAuth 2.0 Mutual TLS Certificate Bound Access Tokens, which means that keycloak bind an access token and a refresh token with a X.509 certificate of a token requesting client exchanged in mutual TLS between keycloak's Token Endpoint and this client. These tokens can be treated as Holder-of-Key tokens instead of bearer tokens.", "type": "string", "enum": [ "true", "false", "" ] }, "token.endpoint.auth.signing.alg": { "title": "Signature algorithm", "description": "The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.", "type": "string" }, "token.response.type.bearer.lower-case": { "title": "Use lower-case bearer type in token responses", "description": "If this is on, token responses will be set the with the type \"bearer\" in lower-case. By default, the server sets the type as \"Bearer\" as defined by RFC6750.", "type": "string", "enum": [ "true", "false", "" ] }, "tosUri": { "title": "Terms of service URL", "description": "URL that the Relying Party Client provides to the End-User to read about the Relying Party's terms of service", "type": "string" }, "use.refresh.tokens": { "title": "Use refresh tokens", "description": "If this is on, a refresh_token will be created and added to the token response. If this is off then no refresh_token will be generated.", "type": "string", "enum": [ "true", "false", "" ] }, "user.info.encrypted.response.alg": { "title": "User info response encryption key management algorithm", "description": "JWA Algorithm used for key management in encrypting User Info Endpoint responses. This option is needed if you want encrypted User Info Endpoint responses. If left empty, User Info Endpoint responses are not encrypted.", "type": "string" }, "user.info.encrypted.response.enc": { "title": "User info response encryption content encryption algorithm", "description": "JWA Algorithm used for content encryption in encrypting User Info Endpoint responses. If User Info response encryption key management algorithm is specified, the default for this value is A128CBC-HS256.", "type": "string" }, "user.info.response.signature.alg": { "title": "User info signed response algorithm", "description": "JWA algorithm used for signed User Info Endpoint response. If set to 'unsigned', User Info Response won't be signed and will be returned in application/json format.", "type": "string" }, "x509.allow.regex.pattern.comparison": { "title": "Allow regex pattern comparison", "description": "If OFF, then the Subject DN from given client certificate must exactly match the given DN from the 'Subject DN' property as described in the RFC8705 specification. The Subject DN can be in the RFC4514 or RFC1779 format. If ON, then the Subject DN from given client certificate should match regex specified by 'Subject DN' property.", "type": "string", "enum": [ "true", "false", "" ] }, "x509.subjectdn": { "title": "Subject DN", "description": "A regular expression for validating Subject DN in the Client Certificate. Use \"(.*?)(?:$)\" to match all kind of expressions.", "type": "string" } }, "additionalProperties": { "type": "string" } }, "authenticationFlowBindingOverrides": { "title": "Authentication flow overrides", "type": "object", "properties": { "browser": { "title": "Browser Flow", "description": "Select the flow you want to use for browser authentication.", "type": "string" }, "direct_grant": { "title": "Direct Grant Flow", "description": "Select the flow you want to use for direct grant authentication.", "type": "string" } }, "additionalProperties": { "type": "string" } }, "authorizationServicesEnabled": { "title": "Authorization", "description": "Enable/Disable fine-grained authorization support for a client.", "type": "boolean" }, "authorizationSettings": { "$ref": "#/$defs/ResourceServerRepresentation" }, "baseUrl": { "title": "Home URL", "description": "Default URL to use when the auth server needs to redirect or link back to the client.", "type": "string" }, "bearerOnly": { "description": "This is a special OIDC type. This client only allows bearer token requests and cannot participate in browser logins.", "type": "boolean" }, "clientAuthenticatorType": { "title": "Client Authenticator", "description": "Client Authenticator used for authentication of this client against Keycloak server", "type": "string", "enum": [ "client-jwt", "client-secret", "client-secret-jwt", "client-x509" ] }, "clientId": { "title": "Client ID", "description": "The client identifier registered with the identity provider.", "type": "string" }, "clientTemplate": { "type": "string" }, "consentRequired": { "title": "Consent required", "description": "If enabled, users have to consent to client access.", "type": "boolean" }, "defaultClientScopes": { "type": "array", "items": { "type": "string" } }, "defaultRoles": { "type": "array", "items": { "type": "string" } }, "description": { "title": "Description", "description": "Help text for the description of the new flow", "type": "string" }, "directAccessGrantsEnabled": { "title": "Direct access grants", "description": "This enables support for Direct Access Grants, which means that client has access to username/password of user and exchange it directly with Keycloak server for access token. In terms of OAuth2 specification, this enables support of 'Resource Owner Password Credentials Grant' for this client.", "type": "boolean" }, "directGrantsOnly": { "type": "boolean" }, "enabled": { "title": "Enabled", "description": "Disabled clients cannot initiate a login or have obtained access tokens.", "type": "boolean" }, "frontchannelLogout": { "title": "Front channel logout", "description": "When true, logout requires a browser redirect to client. When false, server performs a background invocation for logout.", "type": "boolean" }, "fullScopeAllowed": { "title": "Full scope allowed", "description": "Allows you to disable all restrictions.", "type": "boolean" }, "id": { "type": "string" }, "implicitFlowEnabled": { "title": "Implicit flow", "description": "This enables support for OpenID Connect redirect based authentication without authorization code. In terms of OpenID Connect or OAuth2 specifications, this enables support of 'Implicit Flow' for this client.", "type": "boolean" }, "name": { "title": "Name", "description": "Specifies display name of the client. For example 'My Client'. Supports keys for localized values as well. For example: ${my_client}.", "type": "string" }, "nodeReRegistrationTimeout": { "title": "Node Re-registration timeout", "description": "Interval to specify max time for registered clients cluster nodes to re-register. If cluster node will not send re-registration request to Keycloak within this time, it will be unregistered from Keycloak.", "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "notBefore": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 }, "optionalClientScopes": { "type": "array", "items": { "type": "string" } }, "origin": { "type": "string" }, "protocol": { "title": "Protocol", "type": "string" }, "protocolMappers": { "type": "array", "items": { "$ref": "#/$defs/ProtocolMapperRepresentation" } }, "publicClient": { "title": "Client authentication", "description": "This defines the type of the OIDC client. When it's ON, the OIDC type is set to confidential access type. When it's OFF, it is set to public access type.", "type": "boolean" }, "redirectUris": { "title": "Valid redirect URIs", "description": "Valid URI pattern a browser can redirect to after a successful login. Simple wildcards are allowed such as 'http://example.com/*'. Relative path can be specified too such as /my/relative/path/*. Relative paths are relative to the client root URL, or if none is specified the auth server root URL is used. For SAML, you must set valid URI patterns if you are relying on the consumer service URL embedded with the login request.", "type": "array", "items": { "type": "string" } }, "registeredNodes": { "type": "object", "additionalProperties": { "type": "integer", "format": "int32", "maximum": 2147483647.0, "minimum": -2147483648.0 } }, "registrationAccessToken": { "title": "Registration access token", "description": "The registration access token provides access for clients to the client registration service.", "type": "string" }, "rootUrl": { "title": "Root URL", "description": "Root URL appended to relative URLs", "type": "string" }, "secret": { "title": "Client Secret", "type": "string" }, "serviceAccountsEnabled": { "title": "Service accounts roles", "description": "Allows you to authenticate this client to Keycloak and retrieve access token dedicated to this client. In terms of OAuth2 specification, this enables support of 'Client Credentials Grant' for this client.", "type": "boolean" }, "standardFlowEnabled": { "title": "Standard flow", "description": "This enables standard OpenID Connect redirect based authentication with authorization code. In terms of OpenID Connect or OAuth2 specifications, this enables support of 'Authorization Code Flow' for this client.", "type": "boolean" }, "surrogateAuthRequired": { "type": "boolean" }, "type": { "type": "string" }, "useTemplateConfig": { "type": "boolean" }, "useTemplateMappers": { "type": "boolean" }, "useTemplateScope": { "type": "boolean" }, "webOrigins": { "title": "Web origins", "description": "Allowed CORS origins. To permit all origins of Valid Redirect URIs, add '+'. This does not include the '*' wildcard though. To permit all origins, explicitly add '*'.", "type": "array", "items": { "type": "string" } } }, "additionalProperties": false } ``` </details>

---

### spec.definition.access

Type: object

*missing*

---

### spec.definition.adminUrl

Type: string

URL to the admin interface of the client. Set this if the client supports the adapter REST API. This REST API allows the auth server to push revocation policies and other administrative tasks. Usually this is set to the base URL of the client.

---

### spec.definition.alwaysDisplayInConsole

Type: boolean

Always list this client in the Account UI, even if the user does not have an active session.

---

### spec.definition.attributes

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[access.token.lifespan](#specdefinitionattributesaccesstokenlifespan)|string||
|[access.token.signed.response.alg](#specdefinitionattributesaccesstokensignedresponsealg)|string||
|[authorization.encrypted.response.alg](#specdefinitionattributesauthorizationencryptedresponsealg)|string||
|[authorization.encrypted.response.enc](#specdefinitionattributesauthorizationencryptedresponseenc)|string||
|[authorization.signed.response.alg](#specdefinitionattributesauthorizationsignedresponsealg)|string||
|[client.offline.session.idle.timeout](#specdefinitionattributesclientofflinesessionidletimeout)|string||
|[client.offline.session.max.lifespan](#specdefinitionattributesclientofflinesessionmaxlifespan)|string||
|[client.session.idle.timeout](#specdefinitionattributesclientsessionidletimeout)|string||
|[client.session.max.lifespan](#specdefinitionattributesclientsessionmaxlifespan)|string||
|[client_credentials.use_refresh_token](#specdefinitionattributesclientcredentialsuserefreshtoken)|string||
|[exclude.session.state.from.auth.response](#specdefinitionattributesexcludesessionstatefromauthresponse)|string||
|[id.token.encrypted.response.alg](#specdefinitionattributesidtokenencryptedresponsealg)|string||
|[id.token.encrypted.response.enc](#specdefinitionattributesidtokenencryptedresponseenc)|string||
|[id.token.signed.response.alg](#specdefinitionattributesidtokensignedresponsealg)|string||
|[logoUri](#specdefinitionattributeslogouri)|string||
|[pkce.code.challenge.method](#specdefinitionattributespkcecodechallengemethod)|string||
|[policyUri](#specdefinitionattributespolicyuri)|string||
|[post.logout.redirect.uris](#specdefinitionattributespostlogoutredirecturis)|string||
|[request.object.encryption.alg](#specdefinitionattributesrequestobjectencryptionalg)|string||
|[request.object.encryption.enc](#specdefinitionattributesrequestobjectencryptionenc)|string||
|[request.object.required](#specdefinitionattributesrequestobjectrequired)|string||
|[request.object.signature.alg](#specdefinitionattributesrequestobjectsignaturealg)|string||
|[require.pushed.authorization.requests](#specdefinitionattributesrequirepushedauthorizationrequests)|string||
|[tls.client.certificate.bound.access.tokens](#specdefinitionattributestlsclientcertificateboundaccesstokens)|string||
|[token.endpoint.auth.signing.alg](#specdefinitionattributestokenendpointauthsigningalg)|string||
|[token.response.type.bearer.lower-case](#specdefinitionattributestokenresponsetypebearerlowercase)|string||
|[tosUri](#specdefinitionattributestosuri)|string||
|[use.refresh.tokens](#specdefinitionattributesuserefreshtokens)|string||
|[user.info.encrypted.response.alg](#specdefinitionattributesuserinfoencryptedresponsealg)|string||
|[user.info.encrypted.response.enc](#specdefinitionattributesuserinfoencryptedresponseenc)|string||
|[user.info.response.signature.alg](#specdefinitionattributesuserinforesponsesignaturealg)|string||
|[x509.allow.regex.pattern.comparison](#specdefinitionattributesx509allowregexpatterncomparison)|string||
|[x509.subjectdn](#specdefinitionattributesx509subjectdn)|string||

ClientRepresentationAttributes

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "access.token.lifespan": { "title": "Access Token Lifespan", "description": "Max time before an access token is expired. This value is recommended to be short relative to the SSO timeout.", "type": "string", "pattern": "^[0-9]*$" }, "access.token.signed.response.alg": { "title": "Access token signature algorithm", "description": "JWA algorithm used for signing access tokens.", "type": "string" }, "authorization.encrypted.response.alg": { "title": "Authorization response encryption key management algorithm", "description": "JWA Algorithm used for key management in encrypting the authorization response when the response mode is jwt. This option is needed if you want encrypted authorization response. If left empty, the authorization response is just signed, but not encrypted.", "type": "string" }, "authorization.encrypted.response.enc": { "title": "Authorization response encryption content encryption algorithm", "description": "JWA Algorithm used for content encryption in encrypting the authorization response when the response mode is jwt. This option is needed if you want encrypted authorization response. If left empty, the authorization response is just signed, but not encrypted.", "type": "string" }, "authorization.signed.response.alg": { "title": "Authorization response signature algorithm", "description": "JWA algorithm used for signing authorization response tokens when the response mode is jwt.", "type": "string" }, "client.offline.session.idle.timeout": { "title": "Client Offline Session Idle", "description": "Time a client offline session is allowed to be idle before it expires. Offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Idle value.", "type": "string", "pattern": "^[0-9]*$" }, "client.offline.session.max.lifespan": { "title": "Client Offline Session Max", "description": "Max time before a client offline session is expired. If Offline Session Max Limited is enabled at realm level, offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Max value.", "type": "string", "pattern": "^[0-9]*$" }, "client.session.idle.timeout": { "title": "Client Session Idle", "description": "Time a client session is allowed to be idle before it expires. Tokens are invalidated when a client session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Idle value.", "type": "string", "pattern": "^[0-9]*$" }, "client.session.max.lifespan": { "title": "Client Session Max", "description": "Max time before a client session is expired. Tokens are invalidated when a session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Max value.", "type": "string", "pattern": "^[0-9]*$" }, "client_credentials.use_refresh_token": { "title": "Use refresh tokens for client credentials grant", "description": "If this is on, a refresh_token will be created and added to the token response if the client_credentials grant is used. The OAuth 2.0 RFC6749 Section 4.4.3 states that a refresh_token should not be generated when client_credentials grant is used. If this is off then no refresh_token will be generated and the associated user session will be removed.", "type": "string", "enum": [ "true", "false", "" ] }, "exclude.session.state.from.auth.response": { "title": "Exclude Session State From Authentication Response", "description": "If this is on, the parameter 'session_state' will not be included in OpenID Connect Authentication Response. It is useful if the client uses an older OIDC / OAuth2 adapter, which does not support the 'session_state' parameter.", "type": "string", "enum": [ "true", "false", "" ] }, "id.token.encrypted.response.alg": { "title": "ID token encryption key management algorithm", "description": "JWA Algorithm used for key management in encrypting ID tokens. This option is needed if you want encrypted ID tokens. If left empty, ID Tokens are just signed, but not encrypted.", "type": "string" }, "id.token.encrypted.response.enc": { "title": "ID token encryption content encryption algorithm", "description": "JWA Algorithm used for content encryption in encrypting ID tokens. This option is needed just if you want encrypted ID tokens. If left empty, ID Tokens are just signed, but not encrypted.", "type": "string" }, "id.token.signed.response.alg": { "title": "ID token signature algorithm", "description": "JWA algorithm used for signing ID tokens.", "type": "string" }, "logoUri": { "title": "Logo URL", "description": "URL that references a logo for the Client application", "type": "string" }, "pkce.code.challenge.method": { "title": "Proof Key for Code Exchange Code Challenge Method", "description": "Choose which code challenge method for PKCE is used. If not specified, keycloak does not applies PKCE to a client unless the client sends an authorization request with appropriate code challenge and code exchange method.", "type": "string" }, "policyUri": { "title": "Policy URL", "description": "URL that the Relying Party Client provides to the End-User to read about the how the profile data will be used", "type": "string" }, "post.logout.redirect.uris": { "title": "Valid post logout redirect URIs", "description": "Valid URI pattern a browser can redirect to after a successful login. Simple wildcards are allowed such as 'http://example.com/*'. Relative path can be specified too such as /my/relative/path/*. Relative paths are relative to the client root URL, or if none is specified the auth server root URL is used. For SAML, you must set valid URI patterns if you are relying on the consumer service URL embedded with the login request.", "type": "string" }, "request.object.encryption.alg": { "title": "Request object encryption algorithm", "description": "JWE algorithm, which client needs to use when sending OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', encryption is optional and any algorithm is allowed.", "type": "string" }, "request.object.encryption.enc": { "title": "Request object content encryption algorithm", "description": "JWE algorithm, which client needs to use when encrypting the content of the OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', any algorithm is allowed.", "type": "string" }, "request.object.required": { "title": "Request object required", "description": "Specifies if the client needs to provide a request object with their authorization requests, and what method they can use for this. If set to \"not required\", providing a request object is optional. In all other cases, providing a request object is mandatory. If set to \"request\", the request object must be provided by value. If set to \"request_uri\", the request object must be provided by reference. If set to \"request or request_uri\", either method can be used.", "type": "string" }, "request.object.signature.alg": { "title": "Request object signature algorithm", "description": "JWA algorithm, which client needs to use when sending OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', Request object can be signed by any algorithm (including 'none' ).", "type": "string" }, "require.pushed.authorization.requests": { "title": "Pushed authorization request required", "description": "Boolean parameter indicating whether the authorization server accepts authorization request data only via the pushed authorization request method.", "type": "string", "enum": [ "true", "false", "" ] }, "tls.client.certificate.bound.access.tokens": { "title": "OAuth 2.0 Mutual TLS Certificate Bound Access Tokens Enabled", "description": "This enables support for OAuth 2.0 Mutual TLS Certificate Bound Access Tokens, which means that keycloak bind an access token and a refresh token with a X.509 certificate of a token requesting client exchanged in mutual TLS between keycloak's Token Endpoint and this client. These tokens can be treated as Holder-of-Key tokens instead of bearer tokens.", "type": "string", "enum": [ "true", "false", "" ] }, "token.endpoint.auth.signing.alg": { "title": "Signature algorithm", "description": "The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.", "type": "string" }, "token.response.type.bearer.lower-case": { "title": "Use lower-case bearer type in token responses", "description": "If this is on, token responses will be set the with the type \"bearer\" in lower-case. By default, the server sets the type as \"Bearer\" as defined by RFC6750.", "type": "string", "enum": [ "true", "false", "" ] }, "tosUri": { "title": "Terms of service URL", "description": "URL that the Relying Party Client provides to the End-User to read about the Relying Party's terms of service", "type": "string" }, "use.refresh.tokens": { "title": "Use refresh tokens", "description": "If this is on, a refresh_token will be created and added to the token response. If this is off then no refresh_token will be generated.", "type": "string", "enum": [ "true", "false", "" ] }, "user.info.encrypted.response.alg": { "title": "User info response encryption key management algorithm", "description": "JWA Algorithm used for key management in encrypting User Info Endpoint responses. This option is needed if you want encrypted User Info Endpoint responses. If left empty, User Info Endpoint responses are not encrypted.", "type": "string" }, "user.info.encrypted.response.enc": { "title": "User info response encryption content encryption algorithm", "description": "JWA Algorithm used for content encryption in encrypting User Info Endpoint responses. If User Info response encryption key management algorithm is specified, the default for this value is A128CBC-HS256.", "type": "string" }, "user.info.response.signature.alg": { "title": "User info signed response algorithm", "description": "JWA algorithm used for signed User Info Endpoint response. If set to 'unsigned', User Info Response won't be signed and will be returned in application/json format.", "type": "string" }, "x509.allow.regex.pattern.comparison": { "title": "Allow regex pattern comparison", "description": "If OFF, then the Subject DN from given client certificate must exactly match the given DN from the 'Subject DN' property as described in the RFC8705 specification. The Subject DN can be in the RFC4514 or RFC1779 format. If ON, then the Subject DN from given client certificate should match regex specified by 'Subject DN' property.", "type": "string", "enum": [ "true", "false", "" ] }, "x509.subjectdn": { "title": "Subject DN", "description": "A regular expression for validating Subject DN in the Client Certificate. Use \"(.*?)(?:$)\" to match all kind of expressions.", "type": "string" } }, "additionalProperties": { "type": "string" } } ``` </details>

---

### spec.definition.attributes.access.token.lifespan

Type: string

Max time before an access token is expired. This value is recommended to be short relative to the SSO timeout.

---

### spec.definition.attributes.access.token.signed.response.alg

Type: string

JWA algorithm used for signing access tokens.

---

### spec.definition.attributes.authorization.encrypted.response.alg

Type: string

JWA Algorithm used for key management in encrypting the authorization response when the response mode is jwt. This option is needed if you want encrypted authorization response. If left empty, the authorization response is just signed, but not encrypted.

---

### spec.definition.attributes.authorization.encrypted.response.enc

Type: string

JWA Algorithm used for content encryption in encrypting the authorization response when the response mode is jwt. This option is needed if you want encrypted authorization response. If left empty, the authorization response is just signed, but not encrypted.

---

### spec.definition.attributes.authorization.signed.response.alg

Type: string

JWA algorithm used for signing authorization response tokens when the response mode is jwt.

---

### spec.definition.attributes.client.offline.session.idle.timeout

Type: string

Time a client offline session is allowed to be idle before it expires. Offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Idle value.

---

### spec.definition.attributes.client.offline.session.max.lifespan

Type: string

Max time before a client offline session is expired. If Offline Session Max Limited is enabled at realm level, offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Max value.

---

### spec.definition.attributes.client.session.idle.timeout

Type: string

Time a client session is allowed to be idle before it expires. Tokens are invalidated when a client session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Idle value.

---

### spec.definition.attributes.client.session.max.lifespan

Type: string

Max time before a client session is expired. Tokens are invalidated when a session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Max value.

---

### spec.definition.attributes.client_credentials.use_refresh_token

Type: string

If this is on, a refresh_token will be created and added to the token response if the client_credentials grant is used. The OAuth 2.0 RFC6749 Section 4.4.3 states that a refresh_token should not be generated when client_credentials grant is used. If this is off then no refresh_token will be generated and the associated user session will be removed.

---

### spec.definition.attributes.exclude.session.state.from.auth.response

Type: string

If this is on, the parameter 'session_state' will not be included in OpenID Connect Authentication Response. It is useful if the client uses an older OIDC / OAuth2 adapter, which does not support the 'session_state' parameter.

---

### spec.definition.attributes.id.token.encrypted.response.alg

Type: string

JWA Algorithm used for key management in encrypting ID tokens. This option is needed if you want encrypted ID tokens. If left empty, ID Tokens are just signed, but not encrypted.

---

### spec.definition.attributes.id.token.encrypted.response.enc

Type: string

JWA Algorithm used for content encryption in encrypting ID tokens. This option is needed just if you want encrypted ID tokens. If left empty, ID Tokens are just signed, but not encrypted.

---

### spec.definition.attributes.id.token.signed.response.alg

Type: string

JWA algorithm used for signing ID tokens.

---

### spec.definition.attributes.logoUri

Type: string

URL that references a logo for the Client application

---

### spec.definition.attributes.pkce.code.challenge.method

Type: string

Choose which code challenge method for PKCE is used. If not specified, keycloak does not applies PKCE to a client unless the client sends an authorization request with appropriate code challenge and code exchange method.

---

### spec.definition.attributes.policyUri

Type: string

URL that the Relying Party Client provides to the End-User to read about the how the profile data will be used

---

### spec.definition.attributes.post.logout.redirect.uris

Type: string

Valid URI pattern a browser can redirect to after a successful login. Simple wildcards are allowed such as 'http://example.com/*'. Relative path can be specified too such as /my/relative/path/*. Relative paths are relative to the client root URL, or if none is specified the auth server root URL is used. For SAML, you must set valid URI patterns if you are relying on the consumer service URL embedded with the login request.

---

### spec.definition.attributes.request.object.encryption.alg

Type: string

JWE algorithm, which client needs to use when sending OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', encryption is optional and any algorithm is allowed.

---

### spec.definition.attributes.request.object.encryption.enc

Type: string

JWE algorithm, which client needs to use when encrypting the content of the OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', any algorithm is allowed.

---

### spec.definition.attributes.request.object.required

Type: string

Specifies if the client needs to provide a request object with their authorization requests, and what method they can use for this. If set to "not required", providing a request object is optional. In all other cases, providing a request object is mandatory. If set to "request", the request object must be provided by value. If set to "request_uri", the request object must be provided by reference. If set to "request or request_uri", either method can be used.

---

### spec.definition.attributes.request.object.signature.alg

Type: string

JWA algorithm, which client needs to use when sending OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', Request object can be signed by any algorithm (including 'none' ).

---

### spec.definition.attributes.require.pushed.authorization.requests

Type: string

Boolean parameter indicating whether the authorization server accepts authorization request data only via the pushed authorization request method.

---

### spec.definition.attributes.tls.client.certificate.bound.access.tokens

Type: string

This enables support for OAuth 2.0 Mutual TLS Certificate Bound Access Tokens, which means that keycloak bind an access token and a refresh token with a X.509 certificate of a token requesting client exchanged in mutual TLS between keycloak's Token Endpoint and this client. These tokens can be treated as Holder-of-Key tokens instead of bearer tokens.

---

### spec.definition.attributes.token.endpoint.auth.signing.alg

Type: string

The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.

---

### spec.definition.attributes.token.response.type.bearer.lower-case

Type: string

If this is on, token responses will be set the with the type "bearer" in lower-case. By default, the server sets the type as "Bearer" as defined by RFC6750.

---

### spec.definition.attributes.tosUri

Type: string

URL that the Relying Party Client provides to the End-User to read about the Relying Party's terms of service

---

### spec.definition.attributes.use.refresh.tokens

Type: string

If this is on, a refresh_token will be created and added to the token response. If this is off then no refresh_token will be generated.

---

### spec.definition.attributes.user.info.encrypted.response.alg

Type: string

JWA Algorithm used for key management in encrypting User Info Endpoint responses. This option is needed if you want encrypted User Info Endpoint responses. If left empty, User Info Endpoint responses are not encrypted.

---

### spec.definition.attributes.user.info.encrypted.response.enc

Type: string

JWA Algorithm used for content encryption in encrypting User Info Endpoint responses. If User Info response encryption key management algorithm is specified, the default for this value is A128CBC-HS256.

---

### spec.definition.attributes.user.info.response.signature.alg

Type: string

JWA algorithm used for signed User Info Endpoint response. If set to 'unsigned', User Info Response won't be signed and will be returned in application/json format.

---

### spec.definition.attributes.x509.allow.regex.pattern.comparison

Type: string

If OFF, then the Subject DN from given client certificate must exactly match the given DN from the 'Subject DN' property as described in the RFC8705 specification. The Subject DN can be in the RFC4514 or RFC1779 format. If ON, then the Subject DN from given client certificate should match regex specified by 'Subject DN' property.

---

### spec.definition.attributes.x509.subjectdn

Type: string

A regular expression for validating Subject DN in the Client Certificate. Use "(.*?)(?:$)" to match all kind of expressions.

---

### spec.definition.authenticationFlowBindingOverrides

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[browser](#specdefinitionauthenticationflowbindingoverridesbrowser)|string||
|[direct_grant](#specdefinitionauthenticationflowbindingoverridesdirectgrant)|string||

AuthenticationFlowOverrides

<details><summary>JSON schema</summary>

```json { "title": "Authentication flow overrides", "type": "object", "properties": { "browser": { "title": "Browser Flow", "description": "Select the flow you want to use for browser authentication.", "type": "string" }, "direct_grant": { "title": "Direct Grant Flow", "description": "Select the flow you want to use for direct grant authentication.", "type": "string" } }, "additionalProperties": { "type": "string" } } ``` </details>

---

### spec.definition.authenticationFlowBindingOverrides.browser

Type: string

Select the flow you want to use for browser authentication.

---

### spec.definition.authenticationFlowBindingOverrides.direct_grant

Type: string

Select the flow you want to use for direct grant authentication.

---

### spec.definition.authorizationServicesEnabled

Type: boolean

Enable/Disable fine-grained authorization support for a client.

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

ResourceServerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "allowRemoteResourceManagement": { "type": "boolean" }, "authorizationSchema": { "$ref": "#/$defs/AuthorizationSchema" }, "clientId": { "type": "string" }, "decisionStrategy": { "$ref": "#/$defs/DecisionStrategy" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "policyEnforcementMode": { "$ref": "#/$defs/PolicyEnforcementMode" }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" } } }, "additionalProperties": false } ``` </details>

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

AuthorizationSchema

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "resourceTypes": { "type": "object", "additionalProperties": { "$ref": "#/$defs/ResourceType" } } }, "additionalProperties": false } ``` </details>

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

DecisionStrategy

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "AFFIRMATIVE", "UNANIMOUS", "CONSENSUS" ] } ``` </details>

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

PolicyRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "decisionStrategy": { "$ref": "#/$defs/DecisionStrategy" }, "description": { "type": "string" }, "id": { "type": "string" }, "logic": { "$ref": "#/$defs/Logic" }, "name": { "type": "string" }, "owner": { "type": "string" }, "policies": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourceType": { "type": "string" }, "resources": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourcesData": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" }, "uniqueItems": true }, "scopes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "scopesData": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" } }, "additionalProperties": false } ``` </details>

---

### spec.definition.authorizationSettings.policies[].config

Type: object

*missing*

---

### spec.definition.authorizationSettings.policies[].decisionStrategy

Type: string

DecisionStrategy

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "AFFIRMATIVE", "UNANIMOUS", "CONSENSUS" ] } ``` </details>

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

Logic

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "POSITIVE", "NEGATIVE" ] } ``` </details>

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

ResourceRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "_id": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "displayName": { "type": "string" }, "icon_uri": { "type": "string" }, "name": { "type": "string" }, "owner": { "type": "object", "allOf": [ { "$ref": "#/$defs/ResourceOwnerRepresentation" } ] }, "ownerManagedAccess": { "type": "boolean" }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "scopesUma": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" }, "uri": { "type": "string" }, "uris": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } }, "additionalProperties": false } ``` </details>

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

ResourceOwnerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "id": { "type": "string" }, "name": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } }, "additionalProperties": false } ``` </details>

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

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } }, "additionalProperties": false } ``` </details>

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

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } }, "additionalProperties": false } ``` </details>

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

ResourceRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "_id": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "displayName": { "type": "string" }, "icon_uri": { "type": "string" }, "name": { "type": "string" }, "owner": { "type": "object", "allOf": [ { "$ref": "#/$defs/ResourceOwnerRepresentation" } ] }, "ownerManagedAccess": { "type": "boolean" }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "scopesUma": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" }, "uri": { "type": "string" }, "uris": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } }, "additionalProperties": false } ``` </details>

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

ResourceOwnerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "id": { "type": "string" }, "name": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

PolicyEnforcementMode

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "ENFORCING", "PERMISSIVE", "DISABLED" ] } ``` </details>

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

ResourceRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "_id": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "displayName": { "type": "string" }, "icon_uri": { "type": "string" }, "name": { "type": "string" }, "owner": { "type": "object", "allOf": [ { "$ref": "#/$defs/ResourceOwnerRepresentation" } ] }, "ownerManagedAccess": { "type": "boolean" }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "scopesUma": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" }, "uri": { "type": "string" }, "uris": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } }, "additionalProperties": false } ``` </details>

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

ResourceOwnerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "id": { "type": "string" }, "name": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } }, "additionalProperties": false } ``` </details>

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

PolicyRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "decisionStrategy": { "$ref": "#/$defs/DecisionStrategy" }, "description": { "type": "string" }, "id": { "type": "string" }, "logic": { "$ref": "#/$defs/Logic" }, "name": { "type": "string" }, "owner": { "type": "string" }, "policies": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourceType": { "type": "string" }, "resources": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourcesData": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" }, "uniqueItems": true }, "scopes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "scopesData": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" } }, "additionalProperties": false } ``` </details>

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[].config

Type: object

*missing*

---

### spec.definition.authorizationSettings.resources[].scopes[].policies[].decisionStrategy

Type: string

DecisionStrategy

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "AFFIRMATIVE", "UNANIMOUS", "CONSENSUS" ] } ``` </details>

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

Logic

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "POSITIVE", "NEGATIVE" ] } ``` </details>

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

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } }, "additionalProperties": false } ``` </details>

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

PolicyRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "decisionStrategy": { "$ref": "#/$defs/DecisionStrategy" }, "description": { "type": "string" }, "id": { "type": "string" }, "logic": { "$ref": "#/$defs/Logic" }, "name": { "type": "string" }, "owner": { "type": "string" }, "policies": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourceType": { "type": "string" }, "resources": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourcesData": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" }, "uniqueItems": true }, "scopes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "scopesData": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" } }, "additionalProperties": false } ``` </details>

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].config

Type: object

*missing*

---

### spec.definition.authorizationSettings.resources[].scopesUma[].policies[].decisionStrategy

Type: string

DecisionStrategy

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "AFFIRMATIVE", "UNANIMOUS", "CONSENSUS" ] } ``` </details>

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

Logic

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "POSITIVE", "NEGATIVE" ] } ``` </details>

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

ScopeRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "displayName": { "type": "string" }, "iconUri": { "type": "string" }, "id": { "type": "string" }, "name": { "type": "string" }, "policies": { "type": "array", "items": { "$ref": "#/$defs/PolicyRepresentation" } }, "resources": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" } } }, "additionalProperties": false } ``` </details>

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

PolicyRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "config": { "type": "object", "additionalProperties": { "type": "string" } }, "decisionStrategy": { "$ref": "#/$defs/DecisionStrategy" }, "description": { "type": "string" }, "id": { "type": "string" }, "logic": { "$ref": "#/$defs/Logic" }, "name": { "type": "string" }, "owner": { "type": "string" }, "policies": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourceType": { "type": "string" }, "resources": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "resourcesData": { "type": "array", "items": { "$ref": "#/$defs/ResourceRepresentation" }, "uniqueItems": true }, "scopes": { "type": "array", "items": { "type": "string" }, "uniqueItems": true }, "scopesData": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" } }, "additionalProperties": false } ``` </details>

---

### spec.definition.authorizationSettings.scopes[].policies[].config

Type: object

*missing*

---

### spec.definition.authorizationSettings.scopes[].policies[].decisionStrategy

Type: string

DecisionStrategy

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "AFFIRMATIVE", "UNANIMOUS", "CONSENSUS" ] } ``` </details>

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

Logic

<details><summary>JSON schema</summary>

```json { "type": "string", "enum": [ "POSITIVE", "NEGATIVE" ] } ``` </details>

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

ResourceRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "_id": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "displayName": { "type": "string" }, "icon_uri": { "type": "string" }, "name": { "type": "string" }, "owner": { "type": "object", "allOf": [ { "$ref": "#/$defs/ResourceOwnerRepresentation" } ] }, "ownerManagedAccess": { "type": "boolean" }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "scopesUma": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" }, "uri": { "type": "string" }, "uris": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } }, "additionalProperties": false } ``` </details>

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

ResourceOwnerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "id": { "type": "string" }, "name": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

ResourceRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "_id": { "type": "string" }, "attributes": { "type": "object", "additionalProperties": { "type": "array", "items": { "type": "string" } } }, "displayName": { "type": "string" }, "icon_uri": { "type": "string" }, "name": { "type": "string" }, "owner": { "type": "object", "allOf": [ { "$ref": "#/$defs/ResourceOwnerRepresentation" } ] }, "ownerManagedAccess": { "type": "boolean" }, "scopes": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "scopesUma": { "type": "array", "items": { "$ref": "#/$defs/ScopeRepresentation" }, "uniqueItems": true }, "type": { "type": "string" }, "uri": { "type": "string" }, "uris": { "type": "array", "items": { "type": "string" }, "uniqueItems": true } }, "additionalProperties": false } ``` </details>

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

ResourceOwnerRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "id": { "type": "string" }, "name": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

Default URL to use when the auth server needs to redirect or link back to the client.

---

### spec.definition.bearerOnly

Type: boolean

This is a special OIDC type. This client only allows bearer token requests and cannot participate in browser logins.

---

### spec.definition.clientAuthenticatorType

Type: string

Client Authenticator used for authentication of this client against Keycloak server

---

### spec.definition.clientId

Type: string

The client identifier registered with the identity provider.

---

### spec.definition.clientTemplate

Type: string

*missing*

---

### spec.definition.consentRequired

Type: boolean

If enabled, users have to consent to client access.

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

Help text for the description of the new flow

---

### spec.definition.directAccessGrantsEnabled

Type: boolean

This enables support for Direct Access Grants, which means that client has access to username/password of user and exchange it directly with Keycloak server for access token. In terms of OAuth2 specification, this enables support of 'Resource Owner Password Credentials Grant' for this client.

---

### spec.definition.directGrantsOnly

Type: boolean

*missing*

---

### spec.definition.enabled

Type: boolean

Disabled clients cannot initiate a login or have obtained access tokens.

---

### spec.definition.frontchannelLogout

Type: boolean

When true, logout requires a browser redirect to client. When false, server performs a background invocation for logout.

---

### spec.definition.fullScopeAllowed

Type: boolean

Allows you to disable all restrictions.

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

This enables support for OpenID Connect redirect based authentication without authorization code. In terms of OpenID Connect or OAuth2 specifications, this enables support of 'Implicit Flow' for this client.

---

### spec.definition.name

Type: string

Specifies display name of the client. For example 'My Client'. Supports keys for localized values as well. For example: ${my_client}.

---

### spec.definition.nodeReRegistrationTimeout

Type: integer

Interval to specify max time for registered clients cluster nodes to re-register. If cluster node will not send re-registration request to Keycloak within this time, it will be unregistered from Keycloak.

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

### spec.definition.publicClient

Type: boolean

This defines the type of the OIDC client. When it's ON, the OIDC type is set to confidential access type. When it's OFF, it is set to public access type.

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

The registration access token provides access for clients to the client registration service.

---

### spec.definition.rootUrl

Type: string

Root URL appended to relative URLs

---

### spec.definition.secret

Type: string

*missing*

---

### spec.definition.serviceAccountsEnabled

Type: boolean

Allows you to authenticate this client to Keycloak and retrieve access token dedicated to this client. In terms of OAuth2 specification, this enables support of 'Client Credentials Grant' for this client.

---

### spec.definition.standardFlowEnabled

Type: boolean

This enables standard OpenID Connect redirect based authentication with authorization code. In terms of OpenID Connect or OAuth2 specifications, this enables support of 'Authorization Code Flow' for this client.

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