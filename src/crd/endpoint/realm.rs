use crate::{
    api_object_impl,
    crd::{
        schema_patch, HasRoute, ImmutableString, KeycloakApiObjectOptions,
        KeycloakApiStatus, KeycloakInstance,
    },
};
use keycloak::types::RealmRepresentation;
use kube::CustomResource;
use kube::{core::object::HasSpec, ResourceExt};
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};
use up_impl::Root;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakRealm",
    shortname = "kcrm",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced,
    printcolumn = r#"{
            "name":"Ready",
            "type":"boolean",
            "description":"",
            "jsonPath":".status.ready"
        }"#,
    printcolumn = r#"{
            "name":"Status",
            "type":"string",
            "description":"",
            "jsonPath":".status.status"
        }"#,
    printcolumn = r#"{
            "name":"Age",
            "type":"date",
            "description":"",
            "jsonPath":".metadata.creationTimestamp"
        }"#
)]
#[serde(rename_all = "camelCase")]
/// the KeycloakRealm resource
pub struct KeycloakRealmSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// The name of the instance to which this realm belongs
    pub instance_ref: ImmutableString,
    #[schemars(schema_with = "schema")]
    pub definition: RealmRepresentation,
}

api_object_impl!(KeycloakRealm, RealmRepresentation, "realm");

impl up_impl::HasUp for KeycloakRealm {
    type Up = Root<KeycloakInstance>;
    type UpKey = String;
    fn key(&self) -> String {
        use kube::core::object::HasSpec;
        self.spec().instance_ref.clone().into()
    }
}

impl HasRoute for KeycloakRealm {
    type ParentType = String;
    type ParentRefType = String;
    fn id_ident() -> &'static str {
        "realm"
    }
    fn id_option(&self) -> Option<&str> {
        self.spec().definition.realm.as_deref()
    }
    fn id(&self) -> String {
        let name = self.name_unchecked();
        let namespace = self.namespace().unwrap();
        self.id_option()
            .map_or(format!("{namespace}_{name}",), str::to_string)
    }
    fn mount_point(&self) -> &'static str {
        "admin/realms"
    }
}

schema_patch!(KeycloakRealm: |s| {
    s.remove("groups")
        .remove("applications")
        .remove("clients")
        .remove("components")
        .remove("oauthClients");
});
