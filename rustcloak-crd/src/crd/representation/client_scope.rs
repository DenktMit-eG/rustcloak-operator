use crate::{
    impl_object, schema_patch, traits::impl_instance_ref, ImmutableString,
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    KeycloakRealm,
};
use keycloak::types::ClientScopeRepresentation;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakClientScope",
    shortname = "kcss",
    doc = "resource to define a Scope within a [KeycloakClient](./keycloakclient.md)",
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
/// the KeycloakClientScope resource
pub struct KeycloakClientScopeSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// the name of the kubernetes object that created the realm.
    pub realm_ref: ImmutableString,
    // TODO: is_template should be immutable. We can't do immutable options yet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[schemars(schema_with = "schema")]
    pub definition: ClientScopeRepresentation,
    #[serde(default, flatten)]
    pub patches: Option<KeycloakApiPatchList>,
}

impl_instance_ref!(KeycloakClientScope);

impl_object!("scopespec" <realm_ref: String => KeycloakRealm> / |d| {
    if d.is_template == Some(true) {
        "client-scopes"
    } else {
        "client-templates"
    }
} / id for KeycloakClientScope => ClientScopeRepresentation);

schema_patch!(KeycloakClientScope);
