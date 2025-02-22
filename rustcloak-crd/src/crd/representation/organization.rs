use crate::{
    impl_object, schema_patch, traits::impl_instance_ref,
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    KeycloakRealm,
};
use keycloak::types::OrganizationRepresentation;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakOrganization",
    shortname = "kcorg",
    doc = "resource to define an Organisation within a [KeyclaokRealm](./keycloakrealm.md)",
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
/// the KeycloakOrganization resource
pub struct KeycloakOrganizationSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// the name of the kubernetes object that created the realm.
    pub realm_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: OrganizationRepresentation,
    #[serde(default, flatten)]
    pub patches: Option<KeycloakApiPatchList>,
}

impl_object!("org" <realm_ref: String => KeycloakRealm> / |_d| {"organizations"} / id for KeycloakOrganization => OrganizationRepresentation);

impl_instance_ref!(KeycloakOrganization);

schema_patch!(KeycloakOrganization);
