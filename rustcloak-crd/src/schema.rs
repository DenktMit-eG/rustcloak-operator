use schemars::schema::{Schema, SchemaObject, SingleOrVec};
use serde_json::json;

/// Recursively remove int32/int64 format annotations from a schema.
/// kubectl warns about these formats even though they're valid OpenAPI.
pub fn strip_integer_formats(schema: &mut Schema) {
    if let Schema::Object(obj) = schema {
        strip_integer_formats_obj(obj);
    }
}

fn strip_integer_formats_obj(obj: &mut SchemaObject) {
    // Remove format if it's int32 or int64
    if let Some(format) = &obj.format {
        if format == "int32" || format == "int64" {
            obj.format = None;
        }
    }

    // Recurse into object properties
    if let Some(object) = &mut obj.object {
        for prop in object.properties.values_mut() {
            strip_integer_formats(prop);
        }
        if let Some(additional) = &mut object.additional_properties {
            strip_integer_formats(additional);
        }
    }

    // Recurse into array items
    if let Some(array) = &mut obj.array {
        if let Some(items) = &mut array.items {
            match items {
                SingleOrVec::Single(item) => strip_integer_formats(item),
                SingleOrVec::Vec(items) => {
                    for item in items {
                        strip_integer_formats(item);
                    }
                }
            }
        }
    }

    // Recurse into subschemas
    if let Some(subschemas) = &mut obj.subschemas {
        if let Some(all_of) = &mut subschemas.all_of {
            for s in all_of {
                strip_integer_formats(s);
            }
        }
        if let Some(any_of) = &mut subschemas.any_of {
            for s in any_of {
                strip_integer_formats(s);
            }
        }
        if let Some(one_of) = &mut subschemas.one_of {
            for s in one_of {
                strip_integer_formats(s);
            }
        }
        if let Some(not) = &mut subschemas.not {
            strip_integer_formats(not);
        }
    }
}

pub trait SchemaUtil {
    //fn field<F>(&mut self, name: &str, func: F) -> &mut Self
    //where
    //    F: FnOnce(&mut Self) -> &mut Self,
    //    Self: Sized;
    fn prop(&mut self, name: &str) -> &mut Self;
    fn array_item(&mut self) -> &mut Self;
    fn object(&mut self) -> &mut SchemaObject;
    fn remove(&mut self, name: &str) -> &mut Self;
    fn additional_properties(&mut self) -> &mut Self;
    //fn non_null(&mut self) -> &mut Self;
    fn immutable(&mut self) -> &mut Self;
    fn immutable_prop(&mut self, name: &str) -> &mut Self;
}

impl SchemaUtil for Schema {
    //fn field<F>(&mut self, name: &str, func: F) -> &mut Self
    //where
    //    F: FnOnce(&mut Self) -> &mut Self,
    //    Self: Sized,
    //{
    //    func(self.prop(name));
    //    self
    //}

    fn prop(&mut self, name: &str) -> &mut Self {
        self.object()
            .object
            .as_mut()
            .unwrap()
            .properties
            .get_mut(name)
            .expect(name)
    }

    fn array_item(&mut self) -> &mut Self {
        let array = &mut self.object().array.as_mut().unwrap().items;
        if let Some(SingleOrVec::Single(schema)) = array {
            schema.as_mut()
        } else {
            panic!("Expected array schema for RealmRepresentation")
        }
    }

    fn object(&mut self) -> &mut SchemaObject {
        if let Schema::Object(schema_object) = self {
            schema_object
        } else {
            panic!("Expected object schema for RealmRepresentation")
        }
    }

    fn remove(&mut self, name: &str) -> &mut Self {
        self.object()
            .object
            .as_mut()
            .unwrap()
            .properties
            .remove(name)
            .expect(name);
        self
    }

    fn additional_properties(&mut self) -> &mut Self {
        self.object().object.as_mut().unwrap().additional_properties =
            Some(Box::new(Schema::Bool(true)));
        self
    }

    //fn non_null(&mut self) -> &mut Self {
    //    self.object().extensions.remove("nullable");
    //    self
    //}

    fn immutable(&mut self) -> &mut Self {
        self.object().extensions.insert(
            "x-kubernetes-validations".to_owned(),
            json!([{
                "rule": "self == oldSelf",
                "message": "Value is immutable"
            }]),
        );
        self
    }

    fn immutable_prop(&mut self, name: &str) -> &mut Self {
        self.prop(name).immutable();
        self.object().extensions.insert(
            "x-kubernetes-validations".to_owned(),
            json!([{
                "rule": format!("has(self.{0}) == has(oldSelf.{0})", name),
                "message": "Value is immutable"
            }]),
        );

        self
    }
}

#[macro_export]
macro_rules! schema_patch {
    ($name:ident) => {
        $crate::schema_patch!($name: |_| ());
    };
    ($name:ty: $schema:expr) => {
        use schemars::{r#gen::SchemaGenerator, schema::Schema};
        use $crate::object::KeycloakRestObject;
        use $crate::schema::{SchemaUtil, strip_integer_formats};

        pub(crate) fn schema(generator: &mut SchemaGenerator) -> Schema {
            let mut s = generator
                    .clone()
                    .subschema_for::<<$name as KeycloakRestObject>::Definition>()
                    .immutable_prop(<$name as KeycloakRestObject>::ID_FIELD)
                    .to_owned();

            let func: fn(&mut Schema) -> () = $schema;
            func(&mut s);
            strip_integer_formats(&mut s);
            s
        }
    };
}
