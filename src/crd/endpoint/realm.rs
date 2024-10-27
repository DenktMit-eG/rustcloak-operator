use crate::{
    crd::{
        schema_patch, HasApiObject, HasRoute, ImmutableString,
        KeycloakApiObjectOptions, KeycloakApiStatus,
    },
    endpoint::hierarchy::Root,
};
use keycloak::types::RealmRepresentation;
use kube::{core::object::HasSpec, ResourceExt};
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakRealm",
    shortname = "kcrm",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakRealmSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// The name of the instance to which this realm belongs
    pub instance_ref: ImmutableString,
    #[schemars(schema_with = "schema")]
    pub definition: RealmRepresentation,
}

impl HasApiObject for KeycloakRealm {
    type Definition = RealmRepresentation;
    fn definition(&self) -> &Self::Definition {
        &self.spec.definition
    }
    fn options(&self) -> Option<&KeycloakApiObjectOptions> {
        self.spec.options.as_ref()
    }
    fn primary_key() -> &'static str {
        "realm"
    }
    fn primary_key_value_opt(&self) -> Option<&str> {
        self.definition().realm.as_deref()
    }
    fn primary_key_value(&self) -> String {
        let name = self.name_unchecked();
        let namespace = self.namespace().unwrap();
        self.primary_key_value_opt()
            .map_or_else(|| format!("{namespace}_{name}",), str::to_string)
    }
    fn prefix() -> &'static str {
        "realm"
    }
}

impl HasRoute for KeycloakRealm {
    type ParentType = Root;
    type ParentRefType = String;
    fn id_ident() -> &'static str {
        "realm"
    }
    fn id_option(&self) -> Option<&String> {
        self.spec().definition.realm.as_ref()
    }
    fn id(&self) -> String {
        let name = self.name_unchecked();
        let namespace = self.namespace().unwrap();
        self.primary_key_value_opt()
            .map_or_else(|| format!("{namespace}_{name}",), str::to_string)
    }
    fn route(&self) -> &'static str {
        "admin/realms"
    }
    fn route_parent_ref(&self) -> &Self::ParentRefType {
        &self.spec().instance_ref
    }
}

schema_patch!(KeycloakRealm: |s| {
    s.remove("groups")
        .remove("applications")
        .remove("clients")
        .remove("components")
        .remove("oauthClients");
});
