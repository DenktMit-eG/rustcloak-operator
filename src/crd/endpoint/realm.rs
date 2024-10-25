use crate::crd::{
    schema_patch, HasEndpoint, ImmutableString, KeycloakApiObjectOptions,
    KeycloakApiStatus,
};
use keycloak::types::RealmRepresentation;
use kube::ResourceExt;
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

impl HasEndpoint for KeycloakRealm {
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

schema_patch!(KeycloakRealm: |s| {
    s.remove("groups")
        .remove("applications")
        .remove("clients")
        .remove("components")
        .remove("oauthClients");
});
