use super::{KeycloakApiObjectOptions, KeycloakApiStatus, WithStatus};
use crate::util::SchemaUtil;
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
pub struct KeycloakUserSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_name: String,
    #[schemars(schema_with = "client_representation")]
    pub definition: UserRepresentation,
}

// sed 's/\$ref.*//; s/^\* spec\.validation\.openAPIV3Schema\.properties\[spec\]\.properties\[definition\]/schema/; s/\.properties\[\([^]]*\)\]/.prop("\1")/g; s/\.items\./.items()./g; s/\.prop("\([^"]*\)")\.items()\.$/.remove("\1");/'
fn client_representation(generator: &mut SchemaGenerator) -> Schema {
    let mut schema = generator.clone().subschema_for::<UserRepresentation>();

    schema.prop("id").immutable();

    schema
}

impl WithStatus<KeycloakApiStatus> for KeycloakUser {
    fn status(&self) -> Option<&KeycloakApiStatus> {
        self.status.as_ref()
    }
}
