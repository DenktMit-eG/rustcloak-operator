# KeycloakIdentityProvider

## v1beta1

resource to define a identity provider in a [KeyclaokRealm](./keycloakrealm.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.clusterRealmRef](#specclusterrealmref)|string||
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.addReadTokenRoleOnCreate](#specdefinitionaddreadtokenroleoncreate)|boolean||
|[spec.definition.alias](#specdefinitionalias)|string||
|[spec.definition.authenticateByDefault](#specdefinitionauthenticatebydefault)|boolean||
|[spec.definition.config](#specdefinitionconfig)|object||
|[spec.definition.config.allowCreate](#specdefinitionconfigallowcreate)|string||
|[spec.definition.config.allowedClockSkew](#specdefinitionconfigallowedclockskew)|string||
|[spec.definition.config.attributeConsumingServiceIndex](#specdefinitionconfigattributeconsumingserviceindex)|string||
|[spec.definition.config.authnContextClassRefs](#specdefinitionconfigauthncontextclassrefs)|string||
|[spec.definition.config.authnContextComparisonType](#specdefinitionconfigauthncontextcomparisontype)|string||
|[spec.definition.config.authnContextDeclRefs](#specdefinitionconfigauthncontextdeclrefs)|string||
|[spec.definition.config.backchannelSupported](#specdefinitionconfigbackchannelsupported)|string||
|[spec.definition.config.encryptionAlgorithm](#specdefinitionconfigencryptionalgorithm)|string||
|[spec.definition.config.entityId](#specdefinitionconfigentityid)|string||
|[spec.definition.config.forceAuthn](#specdefinitionconfigforceauthn)|string||
|[spec.definition.config.hideOnLoginPage](#specdefinitionconfighideonloginpage)|string||
|[spec.definition.config.idpEntityId](#specdefinitionconfigidpentityid)|string||
|[spec.definition.config.loginHint](#specdefinitionconfigloginhint)|string||
|[spec.definition.config.nameIDPolicyFormat](#specdefinitionconfignameidpolicyformat)|string||
|[spec.definition.config.postBindingAuthnRequest](#specdefinitionconfigpostbindingauthnrequest)|string||
|[spec.definition.config.postBindingLogout](#specdefinitionconfigpostbindinglogout)|string||
|[spec.definition.config.postBindingResponse](#specdefinitionconfigpostbindingresponse)|string||
|[spec.definition.config.principalAttribute](#specdefinitionconfigprincipalattribute)|string||
|[spec.definition.config.principalType](#specdefinitionconfigprincipaltype)|string||
|[spec.definition.config.signSpMetadata](#specdefinitionconfigsignspmetadata)|string||
|[spec.definition.config.signatureAlgorithm](#specdefinitionconfigsignaturealgorithm)|string||
|[spec.definition.config.signingCertificate](#specdefinitionconfigsigningcertificate)|string||
|[spec.definition.config.singleSignOnServiceUrl](#specdefinitionconfigsinglesignonserviceurl)|string||
|[spec.definition.config.syncMode](#specdefinitionconfigsyncmode)|string||
|[spec.definition.config.validateSignature](#specdefinitionconfigvalidatesignature)|string||
|[spec.definition.config.wantAssertionsEncrypted](#specdefinitionconfigwantassertionsencrypted)|string||
|[spec.definition.config.wantAssertionsSigned](#specdefinitionconfigwantassertionssigned)|string||
|[spec.definition.config.wantAuthnRequestsSigned](#specdefinitionconfigwantauthnrequestssigned)|string||
|[spec.definition.config.xmlSigKeyInfoKeyNameTransformer](#specdefinitionconfigxmlsigkeyinfokeynametransformer)|string||
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
|[spec.options](#specoptions)|object||
|[spec.patchFrom](#specpatchfrom)|object||
|[spec.realmRef](#specrealmref)|string||
|[status](#status)|object||
|[status.clusterInstanceRef](#statusclusterinstanceref)|string||
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
|[clusterRealmRef](#specclusterrealmref)|string||
|[definition](#specdefinition)|object|✅|
|[options](#specoptions)|object||
|[patchFrom](#specpatchfrom)|object||
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

```json { "type": "object", "properties": { "addReadTokenRoleOnCreate": { "title": "Stored tokens readable", "description": "Enable/disable if new users can read any stored tokens. This assigns the broker.read-token role.", "type": "boolean" }, "alias": { "title": "Alias", "description": "The alias uniquely identifies an identity provider and it is also used to build the redirect uri.", "type": "string" }, "authenticateByDefault": { "type": "boolean" }, "config": { "type": "object", "properties": { "allowCreate": { "title": "Allow create", "description": "Allow the external identity provider to create a new identifier to represent the principal.", "type": "string", "enum": [ "true", "false", "" ] }, "allowedClockSkew": { "title": "Allowed clock skew", "description": "Clock skew in seconds that is tolerated when validating identity provider tokens. Default value is zero.", "type": "string", "pattern": "^[0-9]*$" }, "attributeConsumingServiceIndex": { "title": "Attribute Consuming Service Index", "description": "Index of the Attribute Consuming Service profile to request during authentication.", "type": "string", "pattern": "^[0-9]*$" }, "authnContextClassRefs": { "title": "AuthnContext ClassRefs", "description": "Ordered list of requested AuthnContext ClassRefs.", "type": "string" }, "authnContextComparisonType": { "title": "Comparison", "description": "Specifies the comparison method used to evaluate the requested context classes or statements. The default is \"Exact\".", "type": "string", "enum": [ "exact", "minimum", "maximum", "better" ] }, "authnContextDeclRefs": { "title": "AuthnContext DeclRefs", "description": "Ordered list of requested AuthnContext DeclRefs.", "type": "string" }, "backchannelSupported": { "title": "Backchannel logout", "description": "Does the external IDP support backchannel logout?", "type": "string", "enum": [ "true", "false", "" ] }, "encryptionAlgorithm": { "title": "Encryption Algorithm", "description": "Encryption algorithm, which is used by SAML IDP for encryption of SAML documents, assertions or IDs. The corresponding decryption key for decrypt SAML document parts will be chosen based on this configured algorithm and should be available in realm keys for the encryption (ENC) usage. If algorithm is not configured, then any supported algorithm is allowed and decryption key will be chosen based on the algorithm configured in SAML document itself.", "type": "string" }, "entityId": { "title": "Service provider entity ID", "description": "The Entity ID that will be used to uniquely identify this SAML Service Provider.", "type": "string" }, "forceAuthn": { "title": "Force authentication", "description": "Indicates whether the identity provider must authenticate the presenter directly rather than rely on a previous security context.", "type": "string", "enum": [ "true", "false", "" ] }, "hideOnLoginPage": { "title": "Hide on login page", "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.", "type": "string", "enum": [ "true", "false", "" ] }, "idpEntityId": { "title": "Identity provider entity ID", "description": "The Entity ID used to validate the Issuer for received SAML assertions. If empty, no Issuer validation is performed.", "type": "string" }, "loginHint": { "title": "Pass subject", "description": "During login phase, forward an optional login_hint query parameter to SAML AuthnRequest's Subject.", "type": "string", "enum": [ "true", "false", "" ] }, "nameIDPolicyFormat": { "title": "NameID policy format", "description": "Specifies the URI reference corresponding to a name identifier format.", "type": "string" }, "postBindingAuthnRequest": { "title": "HTTP-POST binding for AuthnRequest", "description": "Indicates whether the AuthnRequest must be sent using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "postBindingLogout": { "title": "HTTP-POST binding logout", "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "postBindingResponse": { "title": "HTTP-POST binding response", "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "principalAttribute": { "title": "Principal attribute", "description": "Name or Friendly Name of the attribute used to identify external users.", "type": "string" }, "principalType": { "title": "Principal type", "description": "Way to identify and track external users from the assertion. Default is using Subject NameID, alternatively you can set up identifying attribute.", "type": "string", "enum": [ "SUBJECT", "ATTRIBUTE", "FRIENDLY_ATTRIBUTE" ] }, "signSpMetadata": { "title": "Sign service provider metadata", "description": "Enable/disable signature of the provider SAML metadata.", "type": "string", "enum": [ "true", "false", "" ] }, "signatureAlgorithm": { "title": "Signature algorithm", "description": "The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.", "type": "string" }, "signingCertificate": { "title": "Validating X509 certificates", "description": "The public certificates Keycloak uses to validate the signatures of SAML requests and responses from the external IDP when Use metadata descriptor URL is OFF. Multiple certificates can be entered separated by comma (,). The certificates can be re-imported from the Metadata descriptor URL clicking the Import Keys action in the identity provider page. The action downloads the current certificates in the metadata endpoint and assigns them to the config in this same option. You need to click Save to definitely store the re-imported certificates.", "type": "string" }, "singleSignOnServiceUrl": { "title": "Single Sign-On service URL", "description": "The Url that must be used to send authentication requests (SAML AuthnRequest).", "type": "string" }, "syncMode": { "title": "Sync mode", "description": "Default sync mode for all mappers. The sync mode determines when user data will be synced using the mappers. Possible values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider.", "type": "string", "enum": [ "IMPORT", "LEGACY", "FORCE" ] }, "validateSignature": { "title": "Validate Signatures", "description": "Enable/disable signature validation of external IDP signatures.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAssertionsEncrypted": { "title": "Want Assertions encrypted", "description": "Indicates whether this service provider expects an encrypted Assertion.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAssertionsSigned": { "title": "Want Assertions signed", "description": "Indicates whether this service provider expects a signed Assertion.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAuthnRequestsSigned": { "title": "Want AuthnRequests signed", "description": "Indicates whether the identity provider expects a signed AuthnRequest.", "type": "string", "enum": [ "true", "false", "" ] }, "xmlSigKeyInfoKeyNameTransformer": { "title": "SAML signature key name", "description": "Signed SAML documents contain identification of signing key in KeyName element. For Keycloak / RH-SSO counter-party, use KEY_ID, for MS AD FS use CERT_SUBJECT, for others check and use NONE if no other option works.", "type": "string", "enum": [ "NONE", "KEY_ID", "CERT_SUBJECT" ] } }, "additionalProperties": { "type": "string" } }, "displayName": { "title": "Display name", "description": "Friendly name for Identity Providers.", "type": "string" }, "enabled": { "title": "Enabled", "type": "boolean" }, "firstBrokerLoginFlowAlias": { "title": "First login flow override", "description": "Alias of authentication flow, which is triggered after first login with this identity provider. Term 'First Login' means that no Keycloak account is currently linked to the authenticated identity provider account.", "type": "string" }, "hideOnLogin": { "title": "Hide on login page", "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.", "type": "boolean" }, "internalId": { "type": "string" }, "linkOnly": { "title": "Account linking only", "description": "If true, users cannot log in through this provider.  They can only link to this provider.  This is useful if you don't want to allow login from the provider, but want to integrate with a provider.", "type": "boolean" }, "organizationId": { "type": "string" }, "postBrokerLoginFlowAlias": { "title": "Post login flow", "description": "Alias of authentication flow, which is triggered after each login with this identity provider. Useful if you want additional verification of each user authenticated with this identity provider (for example OTP). Leave this to \"None\" if you need no any additional authenticators to be triggered after login with this identity provider. Also note that authenticator implementations must assume that user is already set in ClientSession as identity provider already set it.", "type": "string" }, "providerId": { "type": "string" }, "storeToken": { "title": "Store tokens", "description": "Enable/disable if tokens must be stored after authenticating users.", "type": "boolean" }, "trustEmail": { "title": "Trust Email", "description": "If enabled, email provided by this provider is not verified even if verification is enabled for the realm.", "type": "boolean" }, "updateProfileFirstLogin": { "type": "boolean" }, "updateProfileFirstLoginMode": { "type": "string" } }, "additionalProperties": false } ``` </details>

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

|Property|Type|Required|
|:-------|:---|:------:|
|[allowCreate](#specdefinitionconfigallowcreate)|string||
|[allowedClockSkew](#specdefinitionconfigallowedclockskew)|string||
|[attributeConsumingServiceIndex](#specdefinitionconfigattributeconsumingserviceindex)|string||
|[authnContextClassRefs](#specdefinitionconfigauthncontextclassrefs)|string||
|[authnContextComparisonType](#specdefinitionconfigauthncontextcomparisontype)|string||
|[authnContextDeclRefs](#specdefinitionconfigauthncontextdeclrefs)|string||
|[backchannelSupported](#specdefinitionconfigbackchannelsupported)|string||
|[encryptionAlgorithm](#specdefinitionconfigencryptionalgorithm)|string||
|[entityId](#specdefinitionconfigentityid)|string||
|[forceAuthn](#specdefinitionconfigforceauthn)|string||
|[hideOnLoginPage](#specdefinitionconfighideonloginpage)|string||
|[idpEntityId](#specdefinitionconfigidpentityid)|string||
|[loginHint](#specdefinitionconfigloginhint)|string||
|[nameIDPolicyFormat](#specdefinitionconfignameidpolicyformat)|string||
|[postBindingAuthnRequest](#specdefinitionconfigpostbindingauthnrequest)|string||
|[postBindingLogout](#specdefinitionconfigpostbindinglogout)|string||
|[postBindingResponse](#specdefinitionconfigpostbindingresponse)|string||
|[principalAttribute](#specdefinitionconfigprincipalattribute)|string||
|[principalType](#specdefinitionconfigprincipaltype)|string||
|[signSpMetadata](#specdefinitionconfigsignspmetadata)|string||
|[signatureAlgorithm](#specdefinitionconfigsignaturealgorithm)|string||
|[signingCertificate](#specdefinitionconfigsigningcertificate)|string||
|[singleSignOnServiceUrl](#specdefinitionconfigsinglesignonserviceurl)|string||
|[syncMode](#specdefinitionconfigsyncmode)|string||
|[validateSignature](#specdefinitionconfigvalidatesignature)|string||
|[wantAssertionsEncrypted](#specdefinitionconfigwantassertionsencrypted)|string||
|[wantAssertionsSigned](#specdefinitionconfigwantassertionssigned)|string||
|[wantAuthnRequestsSigned](#specdefinitionconfigwantauthnrequestssigned)|string||
|[xmlSigKeyInfoKeyNameTransformer](#specdefinitionconfigxmlsigkeyinfokeynametransformer)|string||

IdentityProviderRepresentationConfig

<details><summary>JSON schema</summary>

```json { "type": "object", "properties": { "allowCreate": { "title": "Allow create", "description": "Allow the external identity provider to create a new identifier to represent the principal.", "type": "string", "enum": [ "true", "false", "" ] }, "allowedClockSkew": { "title": "Allowed clock skew", "description": "Clock skew in seconds that is tolerated when validating identity provider tokens. Default value is zero.", "type": "string", "pattern": "^[0-9]*$" }, "attributeConsumingServiceIndex": { "title": "Attribute Consuming Service Index", "description": "Index of the Attribute Consuming Service profile to request during authentication.", "type": "string", "pattern": "^[0-9]*$" }, "authnContextClassRefs": { "title": "AuthnContext ClassRefs", "description": "Ordered list of requested AuthnContext ClassRefs.", "type": "string" }, "authnContextComparisonType": { "title": "Comparison", "description": "Specifies the comparison method used to evaluate the requested context classes or statements. The default is \"Exact\".", "type": "string", "enum": [ "exact", "minimum", "maximum", "better" ] }, "authnContextDeclRefs": { "title": "AuthnContext DeclRefs", "description": "Ordered list of requested AuthnContext DeclRefs.", "type": "string" }, "backchannelSupported": { "title": "Backchannel logout", "description": "Does the external IDP support backchannel logout?", "type": "string", "enum": [ "true", "false", "" ] }, "encryptionAlgorithm": { "title": "Encryption Algorithm", "description": "Encryption algorithm, which is used by SAML IDP for encryption of SAML documents, assertions or IDs. The corresponding decryption key for decrypt SAML document parts will be chosen based on this configured algorithm and should be available in realm keys for the encryption (ENC) usage. If algorithm is not configured, then any supported algorithm is allowed and decryption key will be chosen based on the algorithm configured in SAML document itself.", "type": "string" }, "entityId": { "title": "Service provider entity ID", "description": "The Entity ID that will be used to uniquely identify this SAML Service Provider.", "type": "string" }, "forceAuthn": { "title": "Force authentication", "description": "Indicates whether the identity provider must authenticate the presenter directly rather than rely on a previous security context.", "type": "string", "enum": [ "true", "false", "" ] }, "hideOnLoginPage": { "title": "Hide on login page", "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.", "type": "string", "enum": [ "true", "false", "" ] }, "idpEntityId": { "title": "Identity provider entity ID", "description": "The Entity ID used to validate the Issuer for received SAML assertions. If empty, no Issuer validation is performed.", "type": "string" }, "loginHint": { "title": "Pass subject", "description": "During login phase, forward an optional login_hint query parameter to SAML AuthnRequest's Subject.", "type": "string", "enum": [ "true", "false", "" ] }, "nameIDPolicyFormat": { "title": "NameID policy format", "description": "Specifies the URI reference corresponding to a name identifier format.", "type": "string" }, "postBindingAuthnRequest": { "title": "HTTP-POST binding for AuthnRequest", "description": "Indicates whether the AuthnRequest must be sent using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "postBindingLogout": { "title": "HTTP-POST binding logout", "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "postBindingResponse": { "title": "HTTP-POST binding response", "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.", "type": "string", "enum": [ "true", "false", "" ] }, "principalAttribute": { "title": "Principal attribute", "description": "Name or Friendly Name of the attribute used to identify external users.", "type": "string" }, "principalType": { "title": "Principal type", "description": "Way to identify and track external users from the assertion. Default is using Subject NameID, alternatively you can set up identifying attribute.", "type": "string", "enum": [ "SUBJECT", "ATTRIBUTE", "FRIENDLY_ATTRIBUTE" ] }, "signSpMetadata": { "title": "Sign service provider metadata", "description": "Enable/disable signature of the provider SAML metadata.", "type": "string", "enum": [ "true", "false", "" ] }, "signatureAlgorithm": { "title": "Signature algorithm", "description": "The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.", "type": "string" }, "signingCertificate": { "title": "Validating X509 certificates", "description": "The public certificates Keycloak uses to validate the signatures of SAML requests and responses from the external IDP when Use metadata descriptor URL is OFF. Multiple certificates can be entered separated by comma (,). The certificates can be re-imported from the Metadata descriptor URL clicking the Import Keys action in the identity provider page. The action downloads the current certificates in the metadata endpoint and assigns them to the config in this same option. You need to click Save to definitely store the re-imported certificates.", "type": "string" }, "singleSignOnServiceUrl": { "title": "Single Sign-On service URL", "description": "The Url that must be used to send authentication requests (SAML AuthnRequest).", "type": "string" }, "syncMode": { "title": "Sync mode", "description": "Default sync mode for all mappers. The sync mode determines when user data will be synced using the mappers. Possible values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider.", "type": "string", "enum": [ "IMPORT", "LEGACY", "FORCE" ] }, "validateSignature": { "title": "Validate Signatures", "description": "Enable/disable signature validation of external IDP signatures.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAssertionsEncrypted": { "title": "Want Assertions encrypted", "description": "Indicates whether this service provider expects an encrypted Assertion.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAssertionsSigned": { "title": "Want Assertions signed", "description": "Indicates whether this service provider expects a signed Assertion.", "type": "string", "enum": [ "true", "false", "" ] }, "wantAuthnRequestsSigned": { "title": "Want AuthnRequests signed", "description": "Indicates whether the identity provider expects a signed AuthnRequest.", "type": "string", "enum": [ "true", "false", "" ] }, "xmlSigKeyInfoKeyNameTransformer": { "title": "SAML signature key name", "description": "Signed SAML documents contain identification of signing key in KeyName element. For Keycloak / RH-SSO counter-party, use KEY_ID, for MS AD FS use CERT_SUBJECT, for others check and use NONE if no other option works.", "type": "string", "enum": [ "NONE", "KEY_ID", "CERT_SUBJECT" ] } }, "additionalProperties": { "type": "string" } } ``` </details>

---

### spec.definition.config.allowCreate

Type: string

Allow the external identity provider to create a new identifier to represent the principal.

---

### spec.definition.config.allowedClockSkew

Type: string

Clock skew in seconds that is tolerated when validating identity provider tokens. Default value is zero.

---

### spec.definition.config.attributeConsumingServiceIndex

Type: string

Index of the Attribute Consuming Service profile to request during authentication.

---

### spec.definition.config.authnContextClassRefs

Type: string

Ordered list of requested AuthnContext ClassRefs.

---

### spec.definition.config.authnContextComparisonType

Type: string

Specifies the comparison method used to evaluate the requested context classes or statements. The default is "Exact".

---

### spec.definition.config.authnContextDeclRefs

Type: string

Ordered list of requested AuthnContext DeclRefs.

---

### spec.definition.config.backchannelSupported

Type: string

Does the external IDP support backchannel logout?

---

### spec.definition.config.encryptionAlgorithm

Type: string

Encryption algorithm, which is used by SAML IDP for encryption of SAML documents, assertions or IDs. The corresponding decryption key for decrypt SAML document parts will be chosen based on this configured algorithm and should be available in realm keys for the encryption (ENC) usage. If algorithm is not configured, then any supported algorithm is allowed and decryption key will be chosen based on the algorithm configured in SAML document itself.

---

### spec.definition.config.entityId

Type: string

The Entity ID that will be used to uniquely identify this SAML Service Provider.

---

### spec.definition.config.forceAuthn

Type: string

Indicates whether the identity provider must authenticate the presenter directly rather than rely on a previous security context.

---

### spec.definition.config.hideOnLoginPage

Type: string

If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.

---

### spec.definition.config.idpEntityId

Type: string

The Entity ID used to validate the Issuer for received SAML assertions. If empty, no Issuer validation is performed.

---

### spec.definition.config.loginHint

Type: string

During login phase, forward an optional login_hint query parameter to SAML AuthnRequest's Subject.

---

### spec.definition.config.nameIDPolicyFormat

Type: string

Specifies the URI reference corresponding to a name identifier format.

---

### spec.definition.config.postBindingAuthnRequest

Type: string

Indicates whether the AuthnRequest must be sent using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.

---

### spec.definition.config.postBindingLogout

Type: string

Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.

---

### spec.definition.config.postBindingResponse

Type: string

Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.

---

### spec.definition.config.principalAttribute

Type: string

Name or Friendly Name of the attribute used to identify external users.

---

### spec.definition.config.principalType

Type: string

Way to identify and track external users from the assertion. Default is using Subject NameID, alternatively you can set up identifying attribute.

---

### spec.definition.config.signSpMetadata

Type: string

Enable/disable signature of the provider SAML metadata.

---

### spec.definition.config.signatureAlgorithm

Type: string

The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.

---

### spec.definition.config.signingCertificate

Type: string

The public certificates Keycloak uses to validate the signatures of SAML requests and responses from the external IDP when Use metadata descriptor URL is OFF. Multiple certificates can be entered separated by comma (,). The certificates can be re-imported from the Metadata descriptor URL clicking the Import Keys action in the identity provider page. The action downloads the current certificates in the metadata endpoint and assigns them to the config in this same option. You need to click Save to definitely store the re-imported certificates.

---

### spec.definition.config.singleSignOnServiceUrl

Type: string

The Url that must be used to send authentication requests (SAML AuthnRequest).

---

### spec.definition.config.syncMode

Type: string

Default sync mode for all mappers. The sync mode determines when user data will be synced using the mappers. Possible values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider.

---

### spec.definition.config.validateSignature

Type: string

Enable/disable signature validation of external IDP signatures.

---

### spec.definition.config.wantAssertionsEncrypted

Type: string

Indicates whether this service provider expects an encrypted Assertion.

---

### spec.definition.config.wantAssertionsSigned

Type: string

Indicates whether this service provider expects a signed Assertion.

---

### spec.definition.config.wantAuthnRequestsSigned

Type: string

Indicates whether the identity provider expects a signed AuthnRequest.

---

### spec.definition.config.xmlSigKeyInfoKeyNameTransformer

Type: string

Signed SAML documents contain identification of signing key in KeyName element. For Keycloak / RH-SSO counter-party, use KEY_ID, for MS AD FS use CERT_SUBJECT, for others check and use NONE if no other option works.

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

The name of the realm to which this object belongs to

---

### status

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clusterInstanceRef](#statusclusterinstanceref)|string||
|[conditions[]](#statusconditions)|object||
|[instanceRef](#statusinstanceref)|string||
|[message](#statusmessage)|string||
|[ready](#statusready)|boolean|✅|
|[resourcePath](#statusresourcepath)|string||
|[status](#statusstatus)|string||

*missing*

---

### status.clusterInstanceRef

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

The name of the cluster instance to which this object belongs to.

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

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

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