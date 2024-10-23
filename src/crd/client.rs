use super::{KeycloakApiObjectOptions, KeycloakApiStatus};
use crate::util::SchemaUtil;
use keycloak::types::ClientRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakClient",
    shortname = "kcc",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakClientSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[schemars(schema_with = "client_representation")]
    pub definition: ClientRepresentation,
}

// sed 's/\$ref.*//; s/^\* spec\.validation\.openAPIV3Schema\.properties\[spec\]\.properties\[definition\]/schema/; s/\.properties\[\([^]]*\)\]/.prop("\1")/g; s/\.items\./.items()./g; s/\.prop("\([^"]*\)")\.items()\.$/.remove("\1");/'
fn client_representation(generator: &mut SchemaGenerator) -> Schema {
    let mut schema = generator.clone().subschema_for::<ClientRepresentation>();

    schema
        .prop("authorizationSettings")
        .prop("policies")
        .items()
        .prop("resourcesData")
        .items()
        .prop("scopes")
        .items()
        .remove("policies")
        .remove("resources")
        .additional_properties();
    schema
        .prop("authorizationSettings")
        .prop("policies")
        .items()
        .prop("resourcesData")
        .items()
        .prop("scopesUma")
        .items()
        .remove("policies")
        .remove("resources")
        .additional_properties();
    schema
        .prop("authorizationSettings")
        .prop("policies")
        .items()
        .prop("scopesData")
        .items()
        .prop("resources")
        .items()
        .remove("scopes")
        .remove("scopesUma")
        .additional_properties();
    schema
        .prop("authorizationSettings")
        .prop("policies")
        .items()
        .prop("scopesData")
        .items()
        .remove("policies")
        .additional_properties();
    schema
        .prop("authorizationSettings")
        .prop("resources")
        .items()
        .prop("scopes")
        .items()
        .prop("policies")
        .items()
        .remove("resourcesData")
        .remove("scopesData")
        .additional_properties();
    schema
        .prop("authorizationSettings")
        .prop("resources")
        .items()
        .prop("scopes")
        .items()
        .remove("resources")
        .additional_properties();
    schema
        .prop("authorizationSettings")
        .prop("resources")
        .items()
        .prop("scopesUma")
        .items()
        .prop("policies")
        .items()
        .remove("resourcesData")
        .remove("scopesData")
        .additional_properties();
    schema
        .prop("authorizationSettings")
        .prop("resources")
        .items()
        .prop("scopesUma")
        .items()
        .remove("resources")
        .additional_properties();
    schema
        .prop("authorizationSettings")
        .prop("scopes")
        .items()
        .prop("policies")
        .items()
        .prop("resourcesData")
        .items()
        .remove("scopes")
        .remove("scopesUma")
        .additional_properties();
    schema
        .prop("authorizationSettings")
        .prop("scopes")
        .items()
        .prop("policies")
        .items()
        .remove("scopesData")
        .additional_properties();
    schema
        .prop("authorizationSettings")
        .prop("scopes")
        .items()
        .prop("resources")
        .items()
        .remove("scopes")
        .additional_properties();
    schema
        .prop("authorizationSettings")
        .prop("scopes")
        .items()
        .prop("resources")
        .items()
        .remove("scopesUma")
        .additional_properties();

    schema
}
