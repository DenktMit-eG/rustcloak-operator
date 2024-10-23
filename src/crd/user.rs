use crate::{morph::ToApiObject, util::SchemaUtil};

use super::{KeycloakApiObjectOptions, KeycloakApiStatus};
use keycloak::types::UserRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakUser",
    shortname = "kcu",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakUserSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[schemars(schema_with = "user_representation")]
    pub definition: UserRepresentation,
}

fn user_representation(generator: &mut SchemaGenerator) -> Schema {
    let mut schema = generator.clone().subschema_for::<UserRepresentation>();
    schema.immutable_prop(KeycloakUser::PRIMARY_KEY).to_owned()
}
