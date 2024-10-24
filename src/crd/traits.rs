use kube::core::object::HasSpec;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};

pub trait HasKeycloakEndpoint
where
    Self::Definition: JsonSchema + Serialize,
{
    type Definition;
    fn definition(&self) -> &Self::Definition;
    fn primary_key() -> &'static str;

    fn primary_key_value(&self) -> Option<&str>;

    fn schema(generator: &mut SchemaGenerator) -> Schema;

    fn options(&self) -> Option<&KeycloakApiObjectOptions>;
}

impl<'a, R, S, D> HasKeycloakEndpoint for R
where
    D: 'a + JsonSchema + Serialize,
    R: 'a + HasSpec<Spec = S>,
    S: 'static + HasKeycloakEndpoint<Definition = D>,
{
    type Definition = D;
    fn definition(&self) -> &Self::Definition {
        self.spec().definition()
    }

    fn primary_key() -> &'static str {
        S::primary_key()
    }

    fn primary_key_value(&self) -> Option<&str> {
        self.spec().primary_key_value()
    }

    fn schema(generator: &mut SchemaGenerator) -> Schema {
        S::schema(generator)
    }

    fn options(&self) -> Option<&KeycloakApiObjectOptions> {
        self.spec().options()
    }
}

// sed 's/\$ref.*//; s/^\* spec\.validation\.openAPIV3Schema\.properties\[spec\]\.properties\[definition\]/s/; s/\.properties\[\([^]]*\)\]/.prop("\1")/g; s/\.items\./.array_item()./g; s/\.prop("\([^"]*\)")\.array_items()\.$/.remove("\1");/'
#[macro_export]
macro_rules! keycloak_endpoint_impl {
    ($name:ty, $def:ty, $primary_key:ident, $schema:expr) => {
        impl $crate::crd::HasKeycloakEndpoint for $name {
            type Definition = $def;
            fn definition(&self) -> &Self::Definition {
                &self.definition
            }

            fn options(
                &self,
            ) -> Option<&$crate::crd::KeycloakApiObjectOptions> {
                self.options.as_ref()
            }

            fn primary_key() -> &'static str {
                stringify!($primary_key)
            }

            fn primary_key_value(&self) -> Option<&str> {
                self.definition().$primary_key.as_deref()
            }

            fn schema(generator: &mut SchemaGenerator) -> Schema {
                use crate::util::SchemaUtil;
                let mut s = generator
                    .clone()
                    .subschema_for::<Self::Definition>()
                    .immutable_prop(Self::primary_key())
                    .to_owned();

                let func: fn(&mut Schema) -> () = $schema;
                func(&mut s);
                s
            }
        }
    };
}
pub use keycloak_endpoint_impl;
use serde::Serialize;

use super::KeycloakApiObjectOptions;
