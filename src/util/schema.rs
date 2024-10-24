use schemars::schema::{Schema, SchemaObject, SingleOrVec};
use serde_json::json;

pub trait SchemaUtil {
    fn field<F>(&mut self, name: &str, func: F) -> &mut Self
    where
        F: FnOnce(&mut Self) -> &mut Self,
        Self: Sized;
    fn prop(&mut self, name: &str) -> &mut Self;
    fn array_item(&mut self) -> &mut Self;
    fn object(&mut self) -> &mut SchemaObject;
    fn remove(&mut self, name: &str) -> &mut Self;
    fn additional_properties(&mut self) -> &mut Self;
    fn non_null(&mut self) -> &mut Self;
    fn immutable(&mut self) -> &mut Self;
    fn immutable_prop(&mut self, name: &str) -> &mut Self;
}

impl SchemaUtil for Schema {
    fn field<F>(&mut self, name: &str, func: F) -> &mut Self
    where
        F: FnOnce(&mut Self) -> &mut Self,
        Self: Sized,
    {
        func(self.prop(name));
        self
    }

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
        if let Some(SingleOrVec::Single(ref mut schema)) = array {
            schema.as_mut()
        } else {
            panic!("Expected array schema for RealmRepresentation")
        }
    }

    fn object(&mut self) -> &mut SchemaObject {
        if let Schema::Object(ref mut schema_object) = self {
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

    fn non_null(&mut self) -> &mut Self {
        self.object().extensions.remove("nullable");
        self
    }

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
