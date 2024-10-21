use super::KeycloakApiStatus;
use super::KeycloakInstanceSelector;
use super::WithStatus;
use k8s_openapi::api::core::v1::EnvVar;
use kube_derive::CustomResource;
use schemars::gen::SchemaGenerator;
use schemars::schema::InstanceType;
use schemars::schema::ObjectValidation;
use schemars::schema::Schema;
use schemars::schema::SchemaObject;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(
    CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema, Default,
)]
#[kube(
    kind = "KeycloakApiObject",
    shortname = "kcao",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    namespaced,
    status = "KeycloakApiStatus"
)]
/// defines an API request to the Keycloak Admin API.
pub struct KeycloakApiObjectSpec {
    pub api: KeycloakApiObjectOptions,
    pub path: String,
    #[schemars(schema_with = "schema_payload")]
    pub payload: Value,
    pub vars: Option<Vec<EnvVar>>,
}

fn schema_payload(_generator: &mut SchemaGenerator) -> Schema {
    Schema::Object(SchemaObject {
        instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
            InstanceType::Object,
        ))),
        object: Some(Box::new(ObjectValidation {
            additional_properties: Some(Box::new(Schema::Bool(true))),
            ..Default::default()
        })),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
pub struct KeycloakApiObjectOptions {
    pub keycloak_selector: KeycloakInstanceSelector,
    pub must_create: Option<bool>,
}

impl WithStatus<KeycloakApiStatus> for KeycloakApiObject {
    fn status(&self) -> Option<&KeycloakApiStatus> {
        self.status.as_ref()
    }
}
