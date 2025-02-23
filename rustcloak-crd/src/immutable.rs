use crate::schema::SchemaUtil;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Deref;

macro_rules! container_type {
    ($outer:ident, $inner:ty, $schema_fn:tt) => {
        #[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
        pub struct $outer(#[schemars(schema_with = $schema_fn)] pub $inner);
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
        impl From<$outer> for $inner {
            fn from(t: $outer) -> Self {
                t.0
            }
        }
    };
}

container_type!(ImmutableString, String, "immutable_string");
fn immutable_string(generator: &mut SchemaGenerator) -> Schema {
    generator.subschema_for::<String>().immutable().to_owned()
}

container_type!(JsonObject, Value, "json_object");
fn json_object(generator: &mut SchemaGenerator) -> Schema {
    #[derive(JsonSchema)]
    struct Empty {}

    generator
        .subschema_for::<Empty>()
        .additional_properties()
        .to_owned()
}

container_type!(ImmutableJsonObject, Value, "immutable_json_object");
fn immutable_json_object(generator: &mut SchemaGenerator) -> Schema {
    let mut schema = json_object(generator);
    schema.immutable();
    schema
}
