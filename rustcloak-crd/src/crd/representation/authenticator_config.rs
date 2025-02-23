use crate::{
    impl_object, schema_patch, traits::impl_instance_ref, ImmutableString,
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    KeycloakRealm,
};

use keycloak::types::AuthenticatorConfigRepresentation;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakAuthenticatorConfig",
    shortname = "kcac",
    doc = "resource to define an Authenticator Config within a [KeycloakRealm](./keycloakrealm.md)",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    category = "keycloak",
    category = "all",
    namespaced,
    printcolumn = r#"{
            "name":"Ready",
            "type":"boolean",
            "description":"true if the realm is ready",
            "jsonPath":".status.ready"
        }"#,
    printcolumn = r#"{
            "name":"Status",
            "type":"string",
            "description":"Status String of the resource",
            "jsonPath":".status.status"
        }"#,
    printcolumn = r#"{
            "name":"Age",
            "type":"date",
            "description":"time since the realm was created",
            "jsonPath":".metadata.creationTimestamp"
        }"#
)]
#[serde(rename_all = "camelCase")]
/// the KeycloakAuthenticatorConfig resource
pub struct KeycloakAuthenticatorConfigSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// the name of the kubernetes object that created the realm.
    pub realm_ref: ImmutableString,
    #[schemars(schema_with = "schema")]
    pub definition: AuthenticatorConfigRepresentation,
    #[serde(default, flatten)]
    pub patches: Option<KeycloakApiPatchList>,
}

impl_object!("authconfig" <realm_ref: String => KeycloakRealm> / |_d| {"authentication/config"} / id for KeycloakAuthenticatorConfig => AuthenticatorConfigRepresentation);

impl_instance_ref!(KeycloakAuthenticatorConfig);

schema_patch!(KeycloakAuthenticatorConfig);
