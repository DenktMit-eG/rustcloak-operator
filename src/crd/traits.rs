use super::KeycloakApiObjectOptions;
use kube::{core::object::HasSpec, Resource};
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::Serialize;

pub trait HasEndpoint
where
    Self::Definition: JsonSchema + Serialize,
{
    type Definition;
    fn definition(&self) -> &Self::Definition;
    fn primary_key() -> &'static str;

    fn primary_key_value(&self) -> Option<&str>;

    fn schema(generator: &mut SchemaGenerator) -> Schema;

    fn options(&self) -> Option<&KeycloakApiObjectOptions>;

    fn prefix() -> &'static str;
}

impl<'a, R, S, D> HasEndpoint for R
where
    D: 'a + JsonSchema + Serialize,
    R: 'a + HasSpec<Spec = S> + Resource,
    S: 'static + HasEndpoint<Definition = D>,
{
    type Definition = D;
    fn definition(&self) -> &Self::Definition {
        self.spec().definition()
    }

    fn primary_key() -> &'static str {
        S::primary_key()
    }

    fn primary_key_value(&self) -> Option<&str> {
        let uid = self.meta().uid.as_deref();
        self.spec().primary_key_value().or(uid)
    }

    fn schema(generator: &mut SchemaGenerator) -> Schema {
        S::schema(generator)
    }

    fn options(&self) -> Option<&KeycloakApiObjectOptions> {
        self.spec().options()
    }

    fn prefix() -> &'static str {
        S::prefix()
    }
}

// sed 's/\$ref.*//; s/^\* spec\.validation\.openAPIV3Schema\.properties\[spec\]\.properties\[definition\]/s/; s/\.properties\[\([^]]*\)\]/.prop("\1")/g; s/\.items\./.array_item()./g; s/\.prop("\([^"]*\)")\.array_items()\.$/.remove("\1");/'
#[macro_export]
macro_rules! endpoint_impl {
    ($name:ty, $def:ty, $primary_key:ident, $prefix:ident, $schema:expr) => {
        impl $crate::crd::HasEndpoint for $name {
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
                //eprintln!("Generating schema for {}", serde_yaml::to_string(&generator.clone().subschema_for::<Self::Definition>()).unwrap());
                let mut s = generator
                    .clone()
                    .subschema_for::<Self::Definition>()
                    .immutable_prop(Self::primary_key())
                    .to_owned();

                let func: fn(&mut Schema) -> () = $schema;
                func(&mut s);
                s
            }

            fn prefix() -> &'static str {
                stringify!($prefix)
            }
        }
    };
}

pub trait ChildOf {
    type Parent;
    type ParentRefType;
    fn sub_path(&self) -> &'static str;
    fn parent_ref(&self) -> Self::ParentRefType;
}

#[macro_export]
macro_rules! child_of {
    ($name:ty, $parent:ty, $ref:ident, $sub_path:expr) => {
        use kube::core::object::HasSpec;
        impl $crate::crd::ChildOf for $name {
            type Parent = $parent;
            type ParentRefType = String;
            fn sub_path(&self) -> &'static str {
                $sub_path
            }

            fn parent_ref(&self) -> Self::ParentRefType {
                self.spec().$ref.to_string()
            }
        }
    };
}

pub use child_of;
pub use endpoint_impl;
