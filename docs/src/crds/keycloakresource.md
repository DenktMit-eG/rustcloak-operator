# KeycloakResource

## v1

Auto-generated derived type for KeycloakResourceSpec via `CustomResource`

|Name|Type|Required|
|:---|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clientRef](#specclientref)|string|✅|
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition._id](#specdefinitionid)|string||
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.displayName](#specdefinitiondisplayname)|string||
|[spec.definition.icon_uri](#specdefinitioniconuri)|string||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.definition.owner](#specdefinitionowner)|object||
|[spec.definition.owner.id](#specdefinitionownerid)|string||
|[spec.definition.owner.name](#specdefinitionownername)|string||
|[spec.definition.ownerManagedAccess](#specdefinitionownermanagedaccess)|boolean||
|[spec.definition.scopes](#specdefinitionscopes)|array||
|[spec.definition.scopes[]](#specdefinitionscopes)|object||
|[spec.definition.scopes[].displayName](#specdefinitionscopesdisplayname)|string||
|[spec.definition.scopes[].iconUri](#specdefinitionscopesiconuri)|string||
|[spec.definition.scopes[].id](#specdefinitionscopesid)|string||
|[spec.definition.scopes[].name](#specdefinitionscopesname)|string||
|[spec.definition.scopes[].policies](#specdefinitionscopespolicies)|array||
|[spec.definition.scopes[].policies[]](#specdefinitionscopespolicies)|object||
|[spec.definition.scopes[].policies[].config](#specdefinitionscopespoliciesconfig)|object||
|[spec.definition.scopes[].policies[].decisionStrategy](#specdefinitionscopespoliciesdecisionstrategy)|string||
|[spec.definition.scopes[].policies[].description](#specdefinitionscopespoliciesdescription)|string||
|[spec.definition.scopes[].policies[].id](#specdefinitionscopespoliciesid)|string||
|[spec.definition.scopes[].policies[].logic](#specdefinitionscopespolicieslogic)|string||
|[spec.definition.scopes[].policies[].name](#specdefinitionscopespoliciesname)|string||
|[spec.definition.scopes[].policies[].owner](#specdefinitionscopespoliciesowner)|string||
|[spec.definition.scopes[].policies[].policies](#specdefinitionscopespoliciespolicies)|array||
|[spec.definition.scopes[].policies[].policies[]](#specdefinitionscopespoliciespolicies)|string||
|[spec.definition.scopes[].policies[].resources](#specdefinitionscopespoliciesresources)|array||
|[spec.definition.scopes[].policies[].resources[]](#specdefinitionscopespoliciesresources)|string||
|[spec.definition.scopes[].policies[].scopes](#specdefinitionscopespoliciesscopes)|array||
|[spec.definition.scopes[].policies[].scopes[]](#specdefinitionscopespoliciesscopes)|string||
|[spec.definition.scopes[].policies[].type](#specdefinitionscopespoliciestype)|string||
|[spec.definition.scopes[].policies[].config](#specdefinitionscopespoliciesconfig)|object||
|[spec.definition.scopes[].policies[].decisionStrategy](#specdefinitionscopespoliciesdecisionstrategy)|string||
|[spec.definition.scopes[].policies[].description](#specdefinitionscopespoliciesdescription)|string||
|[spec.definition.scopes[].policies[].id](#specdefinitionscopespoliciesid)|string||
|[spec.definition.scopes[].policies[].logic](#specdefinitionscopespolicieslogic)|string||
|[spec.definition.scopes[].policies[].name](#specdefinitionscopespoliciesname)|string||
|[spec.definition.scopes[].policies[].owner](#specdefinitionscopespoliciesowner)|string||
|[spec.definition.scopes[].policies[].policies](#specdefinitionscopespoliciespolicies)|array||
|[spec.definition.scopes[].policies[].policies[]](#specdefinitionscopespoliciespolicies)|string||
|[spec.definition.scopes[].policies[].policies[]](#specdefinitionscopespoliciespolicies)|string||
|[spec.definition.scopes[].policies[].resources](#specdefinitionscopespoliciesresources)|array||
|[spec.definition.scopes[].policies[].resources[]](#specdefinitionscopespoliciesresources)|string||
|[spec.definition.scopes[].policies[].resources[]](#specdefinitionscopespoliciesresources)|string||
|[spec.definition.scopes[].policies[].scopes](#specdefinitionscopespoliciesscopes)|array||
|[spec.definition.scopes[].policies[].scopes[]](#specdefinitionscopespoliciesscopes)|string||
|[spec.definition.scopes[].policies[].scopes[]](#specdefinitionscopespoliciesscopes)|string||
|[spec.definition.scopes[].policies[].type](#specdefinitionscopespoliciestype)|string||
|[spec.definition.scopes[].displayName](#specdefinitionscopesdisplayname)|string||
|[spec.definition.scopes[].iconUri](#specdefinitionscopesiconuri)|string||
|[spec.definition.scopes[].id](#specdefinitionscopesid)|string||
|[spec.definition.scopes[].name](#specdefinitionscopesname)|string||
|[spec.definition.scopes[].policies](#specdefinitionscopespolicies)|array||
|[spec.definition.scopes[].policies[]](#specdefinitionscopespolicies)|object||
|[spec.definition.scopes[].policies[].config](#specdefinitionscopespoliciesconfig)|object||
|[spec.definition.scopes[].policies[].decisionStrategy](#specdefinitionscopespoliciesdecisionstrategy)|string||
|[spec.definition.scopes[].policies[].description](#specdefinitionscopespoliciesdescription)|string||
|[spec.definition.scopes[].policies[].id](#specdefinitionscopespoliciesid)|string||
|[spec.definition.scopes[].policies[].logic](#specdefinitionscopespolicieslogic)|string||
|[spec.definition.scopes[].policies[].name](#specdefinitionscopespoliciesname)|string||
|[spec.definition.scopes[].policies[].owner](#specdefinitionscopespoliciesowner)|string||
|[spec.definition.scopes[].policies[].policies](#specdefinitionscopespoliciespolicies)|array||
|[spec.definition.scopes[].policies[].policies[]](#specdefinitionscopespoliciespolicies)|string||
|[spec.definition.scopes[].policies[].resources](#specdefinitionscopespoliciesresources)|array||
|[spec.definition.scopes[].policies[].resources[]](#specdefinitionscopespoliciesresources)|string||
|[spec.definition.scopes[].policies[].scopes](#specdefinitionscopespoliciesscopes)|array||
|[spec.definition.scopes[].policies[].scopes[]](#specdefinitionscopespoliciesscopes)|string||
|[spec.definition.scopes[].policies[].type](#specdefinitionscopespoliciestype)|string||
|[spec.definition.scopes[].policies[].config](#specdefinitionscopespoliciesconfig)|object||
|[spec.definition.scopes[].policies[].decisionStrategy](#specdefinitionscopespoliciesdecisionstrategy)|string||
|[spec.definition.scopes[].policies[].description](#specdefinitionscopespoliciesdescription)|string||
|[spec.definition.scopes[].policies[].id](#specdefinitionscopespoliciesid)|string||
|[spec.definition.scopes[].policies[].logic](#specdefinitionscopespolicieslogic)|string||
|[spec.definition.scopes[].policies[].name](#specdefinitionscopespoliciesname)|string||
|[spec.definition.scopes[].policies[].owner](#specdefinitionscopespoliciesowner)|string||
|[spec.definition.scopes[].policies[].policies](#specdefinitionscopespoliciespolicies)|array||
|[spec.definition.scopes[].policies[].policies[]](#specdefinitionscopespoliciespolicies)|string||
|[spec.definition.scopes[].policies[].policies[]](#specdefinitionscopespoliciespolicies)|string||
|[spec.definition.scopes[].policies[].resources](#specdefinitionscopespoliciesresources)|array||
|[spec.definition.scopes[].policies[].resources[]](#specdefinitionscopespoliciesresources)|string||
|[spec.definition.scopes[].policies[].resources[]](#specdefinitionscopespoliciesresources)|string||
|[spec.definition.scopes[].policies[].scopes](#specdefinitionscopespoliciesscopes)|array||
|[spec.definition.scopes[].policies[].scopes[]](#specdefinitionscopespoliciesscopes)|string||
|[spec.definition.scopes[].policies[].scopes[]](#specdefinitionscopespoliciesscopes)|string||
|[spec.definition.scopes[].policies[].type](#specdefinitionscopespoliciestype)|string||
|[spec.definition.scopes[].policies[]](#specdefinitionscopespolicies)|object||
|[spec.definition.scopes[].policies[].config](#specdefinitionscopespoliciesconfig)|object||
|[spec.definition.scopes[].policies[].decisionStrategy](#specdefinitionscopespoliciesdecisionstrategy)|string||
|[spec.definition.scopes[].policies[].description](#specdefinitionscopespoliciesdescription)|string||
|[spec.definition.scopes[].policies[].id](#specdefinitionscopespoliciesid)|string||
|[spec.definition.scopes[].policies[].logic](#specdefinitionscopespolicieslogic)|string||
|[spec.definition.scopes[].policies[].name](#specdefinitionscopespoliciesname)|string||
|[spec.definition.scopes[].policies[].owner](#specdefinitionscopespoliciesowner)|string||
|[spec.definition.scopes[].policies[].policies](#specdefinitionscopespoliciespolicies)|array||
|[spec.definition.scopes[].policies[].policies[]](#specdefinitionscopespoliciespolicies)|string||
|[spec.definition.scopes[].policies[].resources](#specdefinitionscopespoliciesresources)|array||
|[spec.definition.scopes[].policies[].resources[]](#specdefinitionscopespoliciesresources)|string||
|[spec.definition.scopes[].policies[].scopes](#specdefinitionscopespoliciesscopes)|array||
|[spec.definition.scopes[].policies[].scopes[]](#specdefinitionscopespoliciesscopes)|string||
|[spec.definition.scopes[].policies[].type](#specdefinitionscopespoliciestype)|string||
|[spec.definition.scopes[].policies[].config](#specdefinitionscopespoliciesconfig)|object||
|[spec.definition.scopes[].policies[].decisionStrategy](#specdefinitionscopespoliciesdecisionstrategy)|string||
|[spec.definition.scopes[].policies[].description](#specdefinitionscopespoliciesdescription)|string||
|[spec.definition.scopes[].policies[].id](#specdefinitionscopespoliciesid)|string||
|[spec.definition.scopes[].policies[].logic](#specdefinitionscopespolicieslogic)|string||
|[spec.definition.scopes[].policies[].name](#specdefinitionscopespoliciesname)|string||
|[spec.definition.scopes[].policies[].owner](#specdefinitionscopespoliciesowner)|string||
|[spec.definition.scopes[].policies[].policies](#specdefinitionscopespoliciespolicies)|array||
|[spec.definition.scopes[].policies[].policies[]](#specdefinitionscopespoliciespolicies)|string||
|[spec.definition.scopes[].policies[].policies[]](#specdefinitionscopespoliciespolicies)|string||
|[spec.definition.scopes[].policies[].resources](#specdefinitionscopespoliciesresources)|array||
|[spec.definition.scopes[].policies[].resources[]](#specdefinitionscopespoliciesresources)|string||
|[spec.definition.scopes[].policies[].resources[]](#specdefinitionscopespoliciesresources)|string||
|[spec.definition.scopes[].policies[].scopes](#specdefinitionscopespoliciesscopes)|array||
|[spec.definition.scopes[].policies[].scopes[]](#specdefinitionscopespoliciesscopes)|string||
|[spec.definition.scopes[].policies[].scopes[]](#specdefinitionscopespoliciesscopes)|string||
|[spec.definition.scopes[].policies[].type](#specdefinitionscopespoliciestype)|string||
|[spec.definition.scopes[].policies[].config](#specdefinitionscopespoliciesconfig)|object||
|[spec.definition.scopes[].policies[].decisionStrategy](#specdefinitionscopespoliciesdecisionstrategy)|string||
|[spec.definition.scopes[].policies[].description](#specdefinitionscopespoliciesdescription)|string||
|[spec.definition.scopes[].policies[].id](#specdefinitionscopespoliciesid)|string||
|[spec.definition.scopes[].policies[].logic](#specdefinitionscopespolicieslogic)|string||
|[spec.definition.scopes[].policies[].name](#specdefinitionscopespoliciesname)|string||
|[spec.definition.scopes[].policies[].owner](#specdefinitionscopespoliciesowner)|string||
|[spec.definition.scopes[].policies[].policies](#specdefinitionscopespoliciespolicies)|array||
|[spec.definition.scopes[].policies[].policies[]](#specdefinitionscopespoliciespolicies)|string||
|[spec.definition.scopes[].policies[].policies[]](#specdefinitionscopespoliciespolicies)|string||
|[spec.definition.scopes[].policies[].policies[]](#specdefinitionscopespoliciespolicies)|string||
|[spec.definition.scopes[].policies[].resources](#specdefinitionscopespoliciesresources)|array||
|[spec.definition.scopes[].policies[].resources[]](#specdefinitionscopespoliciesresources)|string||
|[spec.definition.scopes[].policies[].resources[]](#specdefinitionscopespoliciesresources)|string||
|[spec.definition.scopes[].policies[].resources[]](#specdefinitionscopespoliciesresources)|string||
|[spec.definition.scopes[].policies[].scopes](#specdefinitionscopespoliciesscopes)|array||
|[spec.definition.scopes[].policies[].scopes[]](#specdefinitionscopespoliciesscopes)|string||
|[spec.definition.scopes[].policies[].scopes[]](#specdefinitionscopespoliciesscopes)|string||
|[spec.definition.scopes[].policies[].scopes[]](#specdefinitionscopespoliciesscopes)|string||
|[spec.definition.scopes[].policies[].type](#specdefinitionscopespoliciestype)|string||
|[spec.definition.scopesUma](#specdefinitionscopesuma)|array||
|[spec.definition.scopesUma[]](#specdefinitionscopesuma)|object||
|[spec.definition.scopesUma[].displayName](#specdefinitionscopesumadisplayname)|string||
|[spec.definition.scopesUma[].iconUri](#specdefinitionscopesumaiconuri)|string||
|[spec.definition.scopesUma[].id](#specdefinitionscopesumaid)|string||
|[spec.definition.scopesUma[].name](#specdefinitionscopesumaname)|string||
|[spec.definition.scopesUma[].policies](#specdefinitionscopesumapolicies)|array||
|[spec.definition.scopesUma[].policies[]](#specdefinitionscopesumapolicies)|object||
|[spec.definition.scopesUma[].policies[].config](#specdefinitionscopesumapoliciesconfig)|object||
|[spec.definition.scopesUma[].policies[].decisionStrategy](#specdefinitionscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.scopesUma[].policies[].description](#specdefinitionscopesumapoliciesdescription)|string||
|[spec.definition.scopesUma[].policies[].id](#specdefinitionscopesumapoliciesid)|string||
|[spec.definition.scopesUma[].policies[].logic](#specdefinitionscopesumapolicieslogic)|string||
|[spec.definition.scopesUma[].policies[].name](#specdefinitionscopesumapoliciesname)|string||
|[spec.definition.scopesUma[].policies[].owner](#specdefinitionscopesumapoliciesowner)|string||
|[spec.definition.scopesUma[].policies[].policies](#specdefinitionscopesumapoliciespolicies)|array||
|[spec.definition.scopesUma[].policies[].policies[]](#specdefinitionscopesumapoliciespolicies)|string||
|[spec.definition.scopesUma[].policies[].resources](#specdefinitionscopesumapoliciesresources)|array||
|[spec.definition.scopesUma[].policies[].resources[]](#specdefinitionscopesumapoliciesresources)|string||
|[spec.definition.scopesUma[].policies[].scopes](#specdefinitionscopesumapoliciesscopes)|array||
|[spec.definition.scopesUma[].policies[].scopes[]](#specdefinitionscopesumapoliciesscopes)|string||
|[spec.definition.scopesUma[].policies[].type](#specdefinitionscopesumapoliciestype)|string||
|[spec.definition.scopesUma[].policies[].config](#specdefinitionscopesumapoliciesconfig)|object||
|[spec.definition.scopesUma[].policies[].decisionStrategy](#specdefinitionscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.scopesUma[].policies[].description](#specdefinitionscopesumapoliciesdescription)|string||
|[spec.definition.scopesUma[].policies[].id](#specdefinitionscopesumapoliciesid)|string||
|[spec.definition.scopesUma[].policies[].logic](#specdefinitionscopesumapolicieslogic)|string||
|[spec.definition.scopesUma[].policies[].name](#specdefinitionscopesumapoliciesname)|string||
|[spec.definition.scopesUma[].policies[].owner](#specdefinitionscopesumapoliciesowner)|string||
|[spec.definition.scopesUma[].policies[].policies](#specdefinitionscopesumapoliciespolicies)|array||
|[spec.definition.scopesUma[].policies[].policies[]](#specdefinitionscopesumapoliciespolicies)|string||
|[spec.definition.scopesUma[].policies[].policies[]](#specdefinitionscopesumapoliciespolicies)|string||
|[spec.definition.scopesUma[].policies[].resources](#specdefinitionscopesumapoliciesresources)|array||
|[spec.definition.scopesUma[].policies[].resources[]](#specdefinitionscopesumapoliciesresources)|string||
|[spec.definition.scopesUma[].policies[].resources[]](#specdefinitionscopesumapoliciesresources)|string||
|[spec.definition.scopesUma[].policies[].scopes](#specdefinitionscopesumapoliciesscopes)|array||
|[spec.definition.scopesUma[].policies[].scopes[]](#specdefinitionscopesumapoliciesscopes)|string||
|[spec.definition.scopesUma[].policies[].scopes[]](#specdefinitionscopesumapoliciesscopes)|string||
|[spec.definition.scopesUma[].policies[].type](#specdefinitionscopesumapoliciestype)|string||
|[spec.definition.scopesUma[].displayName](#specdefinitionscopesumadisplayname)|string||
|[spec.definition.scopesUma[].iconUri](#specdefinitionscopesumaiconuri)|string||
|[spec.definition.scopesUma[].id](#specdefinitionscopesumaid)|string||
|[spec.definition.scopesUma[].name](#specdefinitionscopesumaname)|string||
|[spec.definition.scopesUma[].policies](#specdefinitionscopesumapolicies)|array||
|[spec.definition.scopesUma[].policies[]](#specdefinitionscopesumapolicies)|object||
|[spec.definition.scopesUma[].policies[].config](#specdefinitionscopesumapoliciesconfig)|object||
|[spec.definition.scopesUma[].policies[].decisionStrategy](#specdefinitionscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.scopesUma[].policies[].description](#specdefinitionscopesumapoliciesdescription)|string||
|[spec.definition.scopesUma[].policies[].id](#specdefinitionscopesumapoliciesid)|string||
|[spec.definition.scopesUma[].policies[].logic](#specdefinitionscopesumapolicieslogic)|string||
|[spec.definition.scopesUma[].policies[].name](#specdefinitionscopesumapoliciesname)|string||
|[spec.definition.scopesUma[].policies[].owner](#specdefinitionscopesumapoliciesowner)|string||
|[spec.definition.scopesUma[].policies[].policies](#specdefinitionscopesumapoliciespolicies)|array||
|[spec.definition.scopesUma[].policies[].policies[]](#specdefinitionscopesumapoliciespolicies)|string||
|[spec.definition.scopesUma[].policies[].resources](#specdefinitionscopesumapoliciesresources)|array||
|[spec.definition.scopesUma[].policies[].resources[]](#specdefinitionscopesumapoliciesresources)|string||
|[spec.definition.scopesUma[].policies[].scopes](#specdefinitionscopesumapoliciesscopes)|array||
|[spec.definition.scopesUma[].policies[].scopes[]](#specdefinitionscopesumapoliciesscopes)|string||
|[spec.definition.scopesUma[].policies[].type](#specdefinitionscopesumapoliciestype)|string||
|[spec.definition.scopesUma[].policies[].config](#specdefinitionscopesumapoliciesconfig)|object||
|[spec.definition.scopesUma[].policies[].decisionStrategy](#specdefinitionscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.scopesUma[].policies[].description](#specdefinitionscopesumapoliciesdescription)|string||
|[spec.definition.scopesUma[].policies[].id](#specdefinitionscopesumapoliciesid)|string||
|[spec.definition.scopesUma[].policies[].logic](#specdefinitionscopesumapolicieslogic)|string||
|[spec.definition.scopesUma[].policies[].name](#specdefinitionscopesumapoliciesname)|string||
|[spec.definition.scopesUma[].policies[].owner](#specdefinitionscopesumapoliciesowner)|string||
|[spec.definition.scopesUma[].policies[].policies](#specdefinitionscopesumapoliciespolicies)|array||
|[spec.definition.scopesUma[].policies[].policies[]](#specdefinitionscopesumapoliciespolicies)|string||
|[spec.definition.scopesUma[].policies[].policies[]](#specdefinitionscopesumapoliciespolicies)|string||
|[spec.definition.scopesUma[].policies[].resources](#specdefinitionscopesumapoliciesresources)|array||
|[spec.definition.scopesUma[].policies[].resources[]](#specdefinitionscopesumapoliciesresources)|string||
|[spec.definition.scopesUma[].policies[].resources[]](#specdefinitionscopesumapoliciesresources)|string||
|[spec.definition.scopesUma[].policies[].scopes](#specdefinitionscopesumapoliciesscopes)|array||
|[spec.definition.scopesUma[].policies[].scopes[]](#specdefinitionscopesumapoliciesscopes)|string||
|[spec.definition.scopesUma[].policies[].scopes[]](#specdefinitionscopesumapoliciesscopes)|string||
|[spec.definition.scopesUma[].policies[].type](#specdefinitionscopesumapoliciestype)|string||
|[spec.definition.scopesUma[].policies[]](#specdefinitionscopesumapolicies)|object||
|[spec.definition.scopesUma[].policies[].config](#specdefinitionscopesumapoliciesconfig)|object||
|[spec.definition.scopesUma[].policies[].decisionStrategy](#specdefinitionscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.scopesUma[].policies[].description](#specdefinitionscopesumapoliciesdescription)|string||
|[spec.definition.scopesUma[].policies[].id](#specdefinitionscopesumapoliciesid)|string||
|[spec.definition.scopesUma[].policies[].logic](#specdefinitionscopesumapolicieslogic)|string||
|[spec.definition.scopesUma[].policies[].name](#specdefinitionscopesumapoliciesname)|string||
|[spec.definition.scopesUma[].policies[].owner](#specdefinitionscopesumapoliciesowner)|string||
|[spec.definition.scopesUma[].policies[].policies](#specdefinitionscopesumapoliciespolicies)|array||
|[spec.definition.scopesUma[].policies[].policies[]](#specdefinitionscopesumapoliciespolicies)|string||
|[spec.definition.scopesUma[].policies[].resources](#specdefinitionscopesumapoliciesresources)|array||
|[spec.definition.scopesUma[].policies[].resources[]](#specdefinitionscopesumapoliciesresources)|string||
|[spec.definition.scopesUma[].policies[].scopes](#specdefinitionscopesumapoliciesscopes)|array||
|[spec.definition.scopesUma[].policies[].scopes[]](#specdefinitionscopesumapoliciesscopes)|string||
|[spec.definition.scopesUma[].policies[].type](#specdefinitionscopesumapoliciestype)|string||
|[spec.definition.scopesUma[].policies[].config](#specdefinitionscopesumapoliciesconfig)|object||
|[spec.definition.scopesUma[].policies[].decisionStrategy](#specdefinitionscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.scopesUma[].policies[].description](#specdefinitionscopesumapoliciesdescription)|string||
|[spec.definition.scopesUma[].policies[].id](#specdefinitionscopesumapoliciesid)|string||
|[spec.definition.scopesUma[].policies[].logic](#specdefinitionscopesumapolicieslogic)|string||
|[spec.definition.scopesUma[].policies[].name](#specdefinitionscopesumapoliciesname)|string||
|[spec.definition.scopesUma[].policies[].owner](#specdefinitionscopesumapoliciesowner)|string||
|[spec.definition.scopesUma[].policies[].policies](#specdefinitionscopesumapoliciespolicies)|array||
|[spec.definition.scopesUma[].policies[].policies[]](#specdefinitionscopesumapoliciespolicies)|string||
|[spec.definition.scopesUma[].policies[].policies[]](#specdefinitionscopesumapoliciespolicies)|string||
|[spec.definition.scopesUma[].policies[].resources](#specdefinitionscopesumapoliciesresources)|array||
|[spec.definition.scopesUma[].policies[].resources[]](#specdefinitionscopesumapoliciesresources)|string||
|[spec.definition.scopesUma[].policies[].resources[]](#specdefinitionscopesumapoliciesresources)|string||
|[spec.definition.scopesUma[].policies[].scopes](#specdefinitionscopesumapoliciesscopes)|array||
|[spec.definition.scopesUma[].policies[].scopes[]](#specdefinitionscopesumapoliciesscopes)|string||
|[spec.definition.scopesUma[].policies[].scopes[]](#specdefinitionscopesumapoliciesscopes)|string||
|[spec.definition.scopesUma[].policies[].type](#specdefinitionscopesumapoliciestype)|string||
|[spec.definition.scopesUma[].policies[].config](#specdefinitionscopesumapoliciesconfig)|object||
|[spec.definition.scopesUma[].policies[].decisionStrategy](#specdefinitionscopesumapoliciesdecisionstrategy)|string||
|[spec.definition.scopesUma[].policies[].description](#specdefinitionscopesumapoliciesdescription)|string||
|[spec.definition.scopesUma[].policies[].id](#specdefinitionscopesumapoliciesid)|string||
|[spec.definition.scopesUma[].policies[].logic](#specdefinitionscopesumapolicieslogic)|string||
|[spec.definition.scopesUma[].policies[].name](#specdefinitionscopesumapoliciesname)|string||
|[spec.definition.scopesUma[].policies[].owner](#specdefinitionscopesumapoliciesowner)|string||
|[spec.definition.scopesUma[].policies[].policies](#specdefinitionscopesumapoliciespolicies)|array||
|[spec.definition.scopesUma[].policies[].policies[]](#specdefinitionscopesumapoliciespolicies)|string||
|[spec.definition.scopesUma[].policies[].policies[]](#specdefinitionscopesumapoliciespolicies)|string||
|[spec.definition.scopesUma[].policies[].policies[]](#specdefinitionscopesumapoliciespolicies)|string||
|[spec.definition.scopesUma[].policies[].resources](#specdefinitionscopesumapoliciesresources)|array||
|[spec.definition.scopesUma[].policies[].resources[]](#specdefinitionscopesumapoliciesresources)|string||
|[spec.definition.scopesUma[].policies[].resources[]](#specdefinitionscopesumapoliciesresources)|string||
|[spec.definition.scopesUma[].policies[].resources[]](#specdefinitionscopesumapoliciesresources)|string||
|[spec.definition.scopesUma[].policies[].scopes](#specdefinitionscopesumapoliciesscopes)|array||
|[spec.definition.scopesUma[].policies[].scopes[]](#specdefinitionscopesumapoliciesscopes)|string||
|[spec.definition.scopesUma[].policies[].scopes[]](#specdefinitionscopesumapoliciesscopes)|string||
|[spec.definition.scopesUma[].policies[].scopes[]](#specdefinitionscopesumapoliciesscopes)|string||
|[spec.definition.scopesUma[].policies[].type](#specdefinitionscopesumapoliciestype)|string||
|[spec.definition.type](#specdefinitiontype)|string||
|[spec.definition.uri](#specdefinitionuri)|string||
|[spec.definition.uris](#specdefinitionuris)|array||
|[spec.definition.uris[]](#specdefinitionuris)|string||
|[spec.options](#specoptions)|object||
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

the KeycloakResource resource

### spec.clientRef

Type: string

the name of the kubernetes object that created the client.

### spec.definition

Type: object

#### Validations

|Rule|Error Message|
|:---|:------------|
|has(self._id) == has(oldSelf._id)|Value is immutable|

*missing*

### spec.definition._id

Type: string

#### Validations

|Rule|Error Message|
|:---|:------------|
|self == oldSelf|Value is immutable|

*missing*

### spec.definition.attributes

Type: object

*missing*

### spec.definition.displayName

Type: string

*missing*

### spec.definition.icon_uri

Type: string

*missing*

### spec.definition.name

Type: string

*missing*

### spec.definition.owner

Type: object

*missing*

### spec.definition.owner.id

Type: string

*missing*

### spec.definition.owner.name

Type: string

*missing*

### spec.definition.ownerManagedAccess

Type: boolean

*missing*

### spec.definition.scopes

Type: array

*missing*

### spec.definition.scopes[]

Type: object

*missing*

### spec.definition.scopes[].displayName

Type: string

*missing*

### spec.definition.scopes[].iconUri

Type: string

*missing*

### spec.definition.scopes[].id

Type: string

*missing*

### spec.definition.scopes[].name

Type: string

*missing*

### spec.definition.scopes[].policies

Type: array

*missing*

### spec.definition.scopes[].policies[]

Type: object

*missing*

### spec.definition.scopes[].policies[].config

Type: object

*missing*

### spec.definition.scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.scopes[].policies[].description

Type: string

*missing*

### spec.definition.scopes[].policies[].id

Type: string

*missing*

### spec.definition.scopes[].policies[].logic

Type: string

*missing*

### spec.definition.scopes[].policies[].name

Type: string

*missing*

### spec.definition.scopes[].policies[].owner

Type: string

*missing*

### spec.definition.scopes[].policies[].policies

Type: array

*missing*

### spec.definition.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopes[].policies[].resources

Type: array

*missing*

### spec.definition.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopes[].policies[].type

Type: string

*missing*

### spec.definition.scopes[].policies[].config

Type: object

*missing*

### spec.definition.scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.scopes[].policies[].description

Type: string

*missing*

### spec.definition.scopes[].policies[].id

Type: string

*missing*

### spec.definition.scopes[].policies[].logic

Type: string

*missing*

### spec.definition.scopes[].policies[].name

Type: string

*missing*

### spec.definition.scopes[].policies[].owner

Type: string

*missing*

### spec.definition.scopes[].policies[].policies

Type: array

*missing*

### spec.definition.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopes[].policies[].resources

Type: array

*missing*

### spec.definition.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopes[].policies[].type

Type: string

*missing*

### spec.definition.scopes[].displayName

Type: string

*missing*

### spec.definition.scopes[].iconUri

Type: string

*missing*

### spec.definition.scopes[].id

Type: string

*missing*

### spec.definition.scopes[].name

Type: string

*missing*

### spec.definition.scopes[].policies

Type: array

*missing*

### spec.definition.scopes[].policies[]

Type: object

*missing*

### spec.definition.scopes[].policies[].config

Type: object

*missing*

### spec.definition.scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.scopes[].policies[].description

Type: string

*missing*

### spec.definition.scopes[].policies[].id

Type: string

*missing*

### spec.definition.scopes[].policies[].logic

Type: string

*missing*

### spec.definition.scopes[].policies[].name

Type: string

*missing*

### spec.definition.scopes[].policies[].owner

Type: string

*missing*

### spec.definition.scopes[].policies[].policies

Type: array

*missing*

### spec.definition.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopes[].policies[].resources

Type: array

*missing*

### spec.definition.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopes[].policies[].type

Type: string

*missing*

### spec.definition.scopes[].policies[].config

Type: object

*missing*

### spec.definition.scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.scopes[].policies[].description

Type: string

*missing*

### spec.definition.scopes[].policies[].id

Type: string

*missing*

### spec.definition.scopes[].policies[].logic

Type: string

*missing*

### spec.definition.scopes[].policies[].name

Type: string

*missing*

### spec.definition.scopes[].policies[].owner

Type: string

*missing*

### spec.definition.scopes[].policies[].policies

Type: array

*missing*

### spec.definition.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopes[].policies[].resources

Type: array

*missing*

### spec.definition.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopes[].policies[].type

Type: string

*missing*

### spec.definition.scopes[].policies[]

Type: object

*missing*

### spec.definition.scopes[].policies[].config

Type: object

*missing*

### spec.definition.scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.scopes[].policies[].description

Type: string

*missing*

### spec.definition.scopes[].policies[].id

Type: string

*missing*

### spec.definition.scopes[].policies[].logic

Type: string

*missing*

### spec.definition.scopes[].policies[].name

Type: string

*missing*

### spec.definition.scopes[].policies[].owner

Type: string

*missing*

### spec.definition.scopes[].policies[].policies

Type: array

*missing*

### spec.definition.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopes[].policies[].resources

Type: array

*missing*

### spec.definition.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopes[].policies[].type

Type: string

*missing*

### spec.definition.scopes[].policies[].config

Type: object

*missing*

### spec.definition.scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.scopes[].policies[].description

Type: string

*missing*

### spec.definition.scopes[].policies[].id

Type: string

*missing*

### spec.definition.scopes[].policies[].logic

Type: string

*missing*

### spec.definition.scopes[].policies[].name

Type: string

*missing*

### spec.definition.scopes[].policies[].owner

Type: string

*missing*

### spec.definition.scopes[].policies[].policies

Type: array

*missing*

### spec.definition.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopes[].policies[].resources

Type: array

*missing*

### spec.definition.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopes[].policies[].type

Type: string

*missing*

### spec.definition.scopes[].policies[].config

Type: object

*missing*

### spec.definition.scopes[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.scopes[].policies[].description

Type: string

*missing*

### spec.definition.scopes[].policies[].id

Type: string

*missing*

### spec.definition.scopes[].policies[].logic

Type: string

*missing*

### spec.definition.scopes[].policies[].name

Type: string

*missing*

### spec.definition.scopes[].policies[].owner

Type: string

*missing*

### spec.definition.scopes[].policies[].policies

Type: array

*missing*

### spec.definition.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopes[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopes[].policies[].resources

Type: array

*missing*

### spec.definition.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopes[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopes[].policies[].scopes

Type: array

*missing*

### spec.definition.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopes[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopes[].policies[].type

Type: string

*missing*

### spec.definition.scopesUma

Type: array

*missing*

### spec.definition.scopesUma[]

Type: object

*missing*

### spec.definition.scopesUma[].displayName

Type: string

*missing*

### spec.definition.scopesUma[].iconUri

Type: string

*missing*

### spec.definition.scopesUma[].id

Type: string

*missing*

### spec.definition.scopesUma[].name

Type: string

*missing*

### spec.definition.scopesUma[].policies

Type: array

*missing*

### spec.definition.scopesUma[].policies[]

Type: object

*missing*

### spec.definition.scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.scopesUma[].displayName

Type: string

*missing*

### spec.definition.scopesUma[].iconUri

Type: string

*missing*

### spec.definition.scopesUma[].id

Type: string

*missing*

### spec.definition.scopesUma[].name

Type: string

*missing*

### spec.definition.scopesUma[].policies

Type: array

*missing*

### spec.definition.scopesUma[].policies[]

Type: object

*missing*

### spec.definition.scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.scopesUma[].policies[]

Type: object

*missing*

### spec.definition.scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.scopesUma[].policies[].config

Type: object

*missing*

### spec.definition.scopesUma[].policies[].decisionStrategy

Type: string

*missing*

### spec.definition.scopesUma[].policies[].description

Type: string

*missing*

### spec.definition.scopesUma[].policies[].id

Type: string

*missing*

### spec.definition.scopesUma[].policies[].logic

Type: string

*missing*

### spec.definition.scopesUma[].policies[].name

Type: string

*missing*

### spec.definition.scopesUma[].policies[].owner

Type: string

*missing*

### spec.definition.scopesUma[].policies[].policies

Type: array

*missing*

### spec.definition.scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].policies[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].resources

Type: array

*missing*

### spec.definition.scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].resources[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].scopes

Type: array

*missing*

### spec.definition.scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].scopes[]

Type: string

*missing*

### spec.definition.scopesUma[].policies[].type

Type: string

*missing*

### spec.definition.type

Type: string

*missing*

### spec.definition.uri

Type: string

*missing*

### spec.definition.uris

Type: array

*missing*

### spec.definition.uris[]

Type: string

*missing*

### spec.options

Type: object

Options for the request to the Keycloak Admin API.

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