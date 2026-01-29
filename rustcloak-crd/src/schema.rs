use schemars::Schema;
use serde_json::{json, Map, Value};

pub trait SchemaUtil {
    fn prop(&mut self, name: &str) -> &mut Schema;
    fn array_item(&mut self) -> &mut Schema;
    fn remove_prop(&mut self, name: &str) -> &mut Self;
    fn additional_properties(&mut self) -> &mut Self;
    fn immutable(&mut self) -> &mut Self;
    fn immutable_prop(&mut self, name: &str) -> &mut Self;
}

impl SchemaUtil for Schema {
    fn prop(&mut self, name: &str) -> &mut Schema {
        let obj = self
            .as_object_mut()
            .expect("Expected object schema");

        let properties = obj
            .entry("properties")
            .or_insert_with(|| Value::Object(Map::new()))
            .as_object_mut()
            .expect("Expected properties to be an object");

        let prop_value = properties
            .get_mut(name)
            .unwrap_or_else(|| panic!("Property '{}' not found", name));

        // SAFETY: Schema is a newtype wrapper around Value with repr(transparent)
        // so &mut Value and &mut Schema have the same memory layout
        unsafe { &mut *(prop_value as *mut Value as *mut Schema) }
    }

    fn array_item(&mut self) -> &mut Schema {
        let obj = self
            .as_object_mut()
            .expect("Expected object schema");

        let items = obj
            .get_mut("items")
            .expect("Expected items field in array schema");

        // SAFETY: Schema is a newtype wrapper around Value with repr(transparent)
        unsafe { &mut *(items as *mut Value as *mut Schema) }
    }

    fn remove_prop(&mut self, name: &str) -> &mut Self {
        let obj = self
            .as_object_mut()
            .expect("Expected object schema");

        let properties = obj
            .get_mut("properties")
            .and_then(|v| v.as_object_mut())
            .expect("Expected properties in schema");

        properties
            .remove(name)
            .unwrap_or_else(|| panic!("Property '{}' not found", name));

        self
    }

    fn additional_properties(&mut self) -> &mut Self {
        self.insert("additionalProperties".to_owned(), Value::Bool(true));
        self
    }

    fn immutable(&mut self) -> &mut Self {
        self.insert(
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
        self.insert(
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
        use $crate::object::KeycloakRestObject;
        use $crate::schema::SchemaUtil;

        pub(crate) fn schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
            let mut s = <<$name as KeycloakRestObject>::Definition as schemars::JsonSchema>::json_schema(generator);
            s.immutable_prop(<$name as KeycloakRestObject>::ID_FIELD);

            let func: fn(&mut schemars::Schema) -> () = $schema;
            func(&mut s);
            s
        }
    };
}
