use crate::{
    impl_object, schema_patch,
    traits::{impl_instance_ref, SecretKeyNames},
    ImmutableString, KeycloakApiObjectOptions, KeycloakApiPatchList,
    KeycloakApiStatus, KeycloakRealm,
};
use keycloak::types::UserRepresentation;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakUserSecretReference {
    pub secret_name: String,
    pub username_key: Option<String>,
    pub password_key: Option<String>,
}

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakUser",
    shortname = "kcu",
    doc = "resource to define a User within a [KeyclaokRealm](./keycloakrealm.md)",
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
/// the KeycloakUser resource
pub struct KeycloakUserSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// the name of the kubernetes object that created the realm.
    pub realm_ref: ImmutableString,
    #[schemars(schema_with = "schema")]
    pub definition: UserRepresentation,
    #[serde(default, flatten)]
    pub patches: Option<KeycloakApiPatchList>,
    pub user_secret: Option<KeycloakUserSecretReference>,
}

impl SecretKeyNames<2> for KeycloakUser {
    const DEFAULTS: [&'static str; 2] = ["username", "password"];

    fn secret_key_names_opts(&self) -> Option<[&Option<String>; 2]> {
        self.spec
            .user_secret
            .as_ref()
            .map(|s| [&s.username_key, &s.password_key])
    }
}

impl_object!("user" <realm_ref: String => KeycloakRealm> / |_d| {"users"} / id for KeycloakUser => UserRepresentation);

impl_instance_ref!(KeycloakUser);

schema_patch!(KeycloakUser);
