mod api_object;
mod container_types;
mod endpoint;
mod instance;
mod refs;
mod status;
mod traits;

pub use api_object::*;
pub use container_types::*;
pub use endpoint::*;
pub use instance::*;
pub use refs::*;
pub use status::*;
pub use traits::*;

#[macro_export]
macro_rules! schema_patch {
    ($name:ident) => {
        schema_patch!($name: |_| ());
    };
    ($name:ty: $schema:expr) => {
        fn schema(generator: &mut SchemaGenerator) -> Schema {
            use $crate::util::SchemaUtil;
            let mut s = generator
                .clone()
                .subschema_for::<<$name as $crate::crd::traits::HasApiObject>::Definition>()
                .immutable_prop(<$name>::primary_key())
                .to_owned();

            let func: fn(&mut Schema) -> () = $schema;
            func(&mut s);
            s
        }
    };
}
pub use schema_patch;
