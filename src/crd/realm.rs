use super::{
    ImmutableString, KeycloakApiObjectOptions, KeycloakApiStatus, WithStatus,
};
use crate::util::SchemaUtil;
use keycloak::types::RealmRepresentation;
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
pub struct KeycloakRealmSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub instance_ref: ImmutableString,
    #[schemars(schema_with = "realm_representation")]
    pub definition: RealmRepresentation,
}

fn realm_representation(generator: &mut SchemaGenerator) -> Schema {
    let mut schema = generator.clone().subschema_for::<RealmRepresentation>();
    // Make the realm field required
    schema.prop("realm").non_null().immutable();
    // Remove fields that trigger $ref in the schema which is not supported by
    // kubernetes
    schema
        .remove("groups")
        .remove("applications")
        .remove("clients")
        .remove("components")
        .remove("oauthClients")
        .to_owned()
}

impl WithStatus<KeycloakApiStatus> for KeycloakRealm {
    fn status(&self) -> Option<&KeycloakApiStatus> {
        self.status.as_ref()
    }
}
