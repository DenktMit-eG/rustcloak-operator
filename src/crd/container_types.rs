use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, ObjectValidation, Schema, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::ops::Deref;

macro_rules! container_type {
    ($outer:ident, $inner:ty, $schema_fn:tt) => {
        #[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
        pub struct $outer(#[schemars(schema_with = $schema_fn)] $inner);
        impl Deref for $outer {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl From<$inner> for $outer {
            fn from(t: $inner) -> Self {
                $outer(t)
            }
        }
    };
}

container_type!(ImmutableString, String, "immutable_string");
fn immutable_string(generator: &mut SchemaGenerator) -> Schema {
    let mut schema = generator.subschema_for::<String>();
    let Schema::Object(ref mut obj) = schema else {
        panic!("Expected an object schema");
    };
    obj.extensions.insert(
        "x-kubernetes-validations".to_owned(),
        json!([{
            "rule": "self == oldSelf",
            "message": "Value is immutable"
        }]),
    );
    schema
}

container_type!(JsonObject, Value, "json_object");
fn json_object(_generator: &mut SchemaGenerator) -> Schema {
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

container_type!(ImmutableJsonObject, Value, "immutable_json_object");
fn immutable_json_object(generator: &mut SchemaGenerator) -> Schema {
    let mut schema = json_object(generator);
    let Schema::Object(ref mut obj) = schema else {
        panic!("Expected an object schema");
    };
    obj.extensions.insert(
        "x-kubernetes-validations".to_owned(),
        json!([{
            "rule": "self == oldSelf",
            "message": "Value is immutable"
        }]),
    );
    schema
}
