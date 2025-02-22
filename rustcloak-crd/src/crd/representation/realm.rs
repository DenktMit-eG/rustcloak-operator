use crate::{
    impl_object, schema_patch, traits::InstanceRef, KeycloakApiObjectOptions,
    KeycloakApiPatchList, KeycloakApiStatus, KeycloakInstance,
};
use keycloak::types::RealmRepresentation;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakRealm",
    shortname = "kcrm",
    doc = "resource to define an Realm within a [KeyclaokInstance](./keycloakinstance.md)",
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
/// the KeycloakRealm resource
pub struct KeycloakRealmSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// The name of the instance to which this realm belongs
    pub instance_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: RealmRepresentation,
    #[serde(default, flatten)]
    pub patches: Option<KeycloakApiPatchList>,
}

impl_object!("realm" <instance_ref: String => KeycloakInstance> / |_d| {"admin/realms"} / realm for KeycloakRealm => RealmRepresentation);

impl InstanceRef for KeycloakRealm {
    fn instance_ref(&self) -> Option<&str> {
        Some(self.spec.instance_ref.as_str())
    }
    fn resource_path(&self) -> Option<&str> {
        self.status
            .as_ref()
            .and_then(|status| status.resource_path.as_deref())
    }
}

schema_patch!(KeycloakRealm: |s| {
    s.remove("groups")
        .remove("applications")
        .remove("clients")
        .remove("components")
        .remove("oauthClients")
        .remove("adminPermissionsClient");
});
