# KeycloakIdentityProvider

## v1beta1

resource to define an identity provider in a [KeycloakRealm](./keycloakrealm.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clusterRealmRef](#specclusterrealmref)|string||
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.addReadTokenRoleOnCreate](#specdefinitionaddreadtokenroleoncreate)|boolean||
|[spec.definition.alias](#specdefinitionalias)|string||
|[spec.definition.authenticateByDefault](#specdefinitionauthenticatebydefault)|boolean||
|[spec.definition.config](#specdefinitionconfig)|object||
|[spec.definition.displayName](#specdefinitiondisplayname)|string||
|[spec.definition.enabled](#specdefinitionenabled)|boolean||
|[spec.definition.firstBrokerLoginFlowAlias](#specdefinitionfirstbrokerloginflowalias)|string||
|[spec.definition.hideOnLogin](#specdefinitionhideonlogin)|boolean||
|[spec.definition.internalId](#specdefinitioninternalid)|string||
|[spec.definition.linkOnly](#specdefinitionlinkonly)|boolean||
|[spec.definition.organizationId](#specdefinitionorganizationid)|string||
|[spec.definition.postBrokerLoginFlowAlias](#specdefinitionpostbrokerloginflowalias)|string||
|[spec.definition.providerId](#specdefinitionproviderid)|string||
|[spec.definition.storeToken](#specdefinitionstoretoken)|boolean||
|[spec.definition.trustEmail](#specdefinitiontrustemail)|boolean||
|[spec.definition.updateProfileFirstLogin](#specdefinitionupdateprofilefirstlogin)|boolean||
|[spec.definition.updateProfileFirstLoginMode](#specdefinitionupdateprofilefirstloginmode)|string||
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
|[spec.realmRef](#specrealmref)|string||
|[status](#status)|object||
|[status.apiObjectRef](#statusapiobjectref)|string||
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
|[status.realm](#statusrealm)|object||
|[status.realm.clusterRealmRef](#statusrealmclusterrealmref)|string||
|[status.realm.realmRef](#statusrealmrealmref)|string||
|[status.resourcePath](#statusresourcepath)|string||
|[status.status](#statusstatus)|string||

---

### spec

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clusterRealmRef](#specclusterrealmref)|string||
|[definition](#specdefinition)|object|✅|
|[patchFrom[]](#specpatchfrom)|object||
|[realmRef](#specrealmref)|string||

the KeycloakIdentityProvider resource

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
|[addReadTokenRoleOnCreate](#specdefinitionaddreadtokenroleoncreate)|boolean||
|[alias](#specdefinitionalias)|string||
|[authenticateByDefault](#specdefinitionauthenticatebydefault)|boolean||
|[config](#specdefinitionconfig)|object||
|[displayName](#specdefinitiondisplayname)|string||
|[enabled](#specdefinitionenabled)|boolean||
|[firstBrokerLoginFlowAlias](#specdefinitionfirstbrokerloginflowalias)|string||
|[hideOnLogin](#specdefinitionhideonlogin)|boolean||
|[internalId](#specdefinitioninternalid)|string||
|[linkOnly](#specdefinitionlinkonly)|boolean||
|[organizationId](#specdefinitionorganizationid)|string||
|[postBrokerLoginFlowAlias](#specdefinitionpostbrokerloginflowalias)|string||
|[providerId](#specdefinitionproviderid)|string||
|[storeToken](#specdefinitionstoretoken)|boolean||
|[trustEmail](#specdefinitiontrustemail)|boolean||
|[updateProfileFirstLogin](#specdefinitionupdateprofilefirstlogin)|boolean||
|[updateProfileFirstLoginMode](#specdefinitionupdateprofilefirstloginmode)|string||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.alias) == has(oldSelf.alias)|Value is immutable|

IdentityProviderRepresentation

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "addReadTokenRoleOnCreate": { "title": "Stored tokens readable", "description": "Enable/disable if new users can read any stored tokens. This assigns the broker.read-token role.", "type": "boolean" }, "alias": { "title": "Alias", "description": "The alias uniquely identifies an identity provider and it is also used to build the redirect uri.", "type": "string" }, "authenticateByDefault": { "type": "boolean" }, "config": { "type": "object", "additionalProperties": { "type": "string" } }, "displayName": { "title": "Display name", "description": "Friendly name for Identity Providers.", "type": "string" }, "enabled": { "title": "Enabled", "type": "boolean" }, "firstBrokerLoginFlowAlias": { "title": "First login flow override", "description": "Alias of authentication flow, which is triggered after first login with this identity provider. Term 'First Login' means that no Keycloak account is currently linked to the authenticated identity provider account.", "type": "string" }, "hideOnLogin": { "title": "Hide on login page", "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.", "type": "boolean" }, "internalId": { "type": "string" }, "linkOnly": { "title": "Account linking only", "description": "If true, users cannot log in through this provider.  They can only link to this provider.  This is useful if you don't want to allow login from the provider, but want to integrate with a provider.", "type": "boolean" }, "organizationId": { "type": "string" }, "postBrokerLoginFlowAlias": { "title": "Post login flow", "description": "Alias of authentication flow, which is triggered after each login with this identity provider. Useful if you want additional verification of each user authenticated with this identity provider (for example OTP). Leave this to \"None\" if you need no any additional authenticators to be triggered after login with this identity provider. Also note that authenticator implementations must assume that user is already set in ClientSession as identity provider already set it.", "type": "string" }, "providerId": { "type": "string" }, "storeToken": { "title": "Store tokens", "description": "Enable/disable if tokens must be stored after authenticating users.", "type": "boolean" }, "trustEmail": { "title": "Trust Email", "description": "If enabled, email provided by this provider is not verified even if verification is enabled for the realm.", "type": "boolean" }, "updateProfileFirstLogin": { "type": "boolean" }, "updateProfileFirstLoginMode": { "type": "string" } } } ``` </details>

---

### spec.definition.addReadTokenRoleOnCreate

Type: boolean

Enable/disable if new users can read any stored tokens. This assigns the broker.read-token role.

---

### spec.definition.alias

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The alias uniquely identifies an identity provider and it is also used to build the redirect uri.

---

### spec.definition.authenticateByDefault

Type: boolean

*missing*

---

### spec.definition.config

Type: object

*missing*

---

### spec.definition.displayName

Type: string

Friendly name for Identity Providers.

---

### spec.definition.enabled

Type: boolean

*missing*

---

### spec.definition.firstBrokerLoginFlowAlias

Type: string

Alias of authentication flow, which is triggered after first login with this identity provider. Term 'First Login' means that no Keycloak account is currently linked to the authenticated identity provider account.

---

### spec.definition.hideOnLogin

Type: boolean

If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.

---

### spec.definition.internalId

Type: string

*missing*

---

### spec.definition.linkOnly

Type: boolean

If true, users cannot log in through this provider.  They can only link to this provider.  This is useful if you don't want to allow login from the provider, but want to integrate with a provider.

---

### spec.definition.organizationId

Type: string

*missing*

---

### spec.definition.postBrokerLoginFlowAlias

Type: string

Alias of authentication flow, which is triggered after each login with this identity provider. Useful if you want additional verification of each user authenticated with this identity provider (for example OTP). Leave this to "None" if you need no any additional authenticators to be triggered after login with this identity provider. Also note that authenticator implementations must assume that user is already set in ClientSession as identity provider already set it.

---

### spec.definition.providerId

Type: string

*missing*

---

### spec.definition.storeToken

Type: boolean

Enable/disable if tokens must be stored after authenticating users.

---

### spec.definition.trustEmail

Type: boolean

If enabled, email provided by this provider is not verified even if verification is enabled for the realm.

---

### spec.definition.updateProfileFirstLogin

Type: boolean

*missing*

---

### spec.definition.updateProfileFirstLoginMode

Type: string

*missing*

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
|[apiObjectRef](#statusapiobjectref)|string||
|[conditions[]](#statusconditions)|object||
|[instance](#statusinstance)|object||
|[message](#statusmessage)|string||
|[ready](#statusready)|boolean|✅|
|[realm](#statusrealm)|object||
|[resourcePath](#statusresourcepath)|string||
|[status](#statusstatus)|string||

*missing*

---

### status.apiObjectRef

Type: string

Reference to the API object name (KeycloakApiObject or ClusterKeycloakApiObject)

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

### status.realm

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clusterRealmRef](#statusrealmclusterrealmref)|string||
|[realmRef](#statusrealmrealmref)|string||

Optional for backwards compatibility

---

### status.realm.clusterRealmRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the cluster realm to which this object belongs to

---

### status.realm.realmRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the realm to which this object belongs to

---

### status.resourcePath

Type: string

*missing*

---

### status.status

Type: string

*missing*