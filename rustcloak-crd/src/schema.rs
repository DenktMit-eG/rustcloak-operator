use std::ops::DerefMut;

use schemars::schema::{
    ArrayValidation, ObjectValidation, Schema, SchemaObject, SingleOrVec,
};
use serde_json::json;

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

fn make_k8s_happy_array(array: &mut ArrayValidation) {
    match &mut array.items {
        Some(SingleOrVec::Single(schema)) => make_k8s_happy(schema.deref_mut()),
        Some(SingleOrVec::Vec(schemas)) => {
            for schema in schemas {
                make_k8s_happy(schema);
            }
        }
        _ => return,
    };
}

fn make_k8s_happy_object(object: &mut ObjectValidation) {
    // We need to do that because Kubernetes has a very different
    // understanding of what additionalProperties means.
    //
    // Ref: https://github.com/kubernetes/kubernetes/issues/94593
    if !object.properties.is_empty() {
        object.additional_properties = None;
    }

    // TODO Remove this.
    // See: https://github.com/jirutka/keycloak-json-schema/issues/2
    object.properties.remove("bruteForceDetection");

    for (_, schema) in &mut object.properties {
        make_k8s_happy(schema);
    }
    if let Some(schema) = &mut object.additional_properties {
        make_k8s_happy(schema);
    }
}

/// This function works around a few quirks in the Kubernetes JSON schema validation.
/// And bugs in the schema itself.
/// See the comments for more information.
pub(crate) fn make_k8s_happy(schema: &mut Schema) {
    let Schema::Object(object) = schema else {
        return;
    };

    if let Some(array) = &mut object.array {
        make_k8s_happy_array(array);
    }

    if let Some(object) = &mut object.object {
        make_k8s_happy_object(object);
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
        use $crate::schema::SchemaUtil;

        pub(crate) fn schema(generator: &mut SchemaGenerator) -> Schema {
            let mut s = generator
                    .clone()
                    .subschema_for::<<$name as KeycloakRestObject>::Definition>()
                    .immutable_prop(<$name as KeycloakRestObject>::ID_FIELD)
                    .to_owned();

            $crate::schema::make_k8s_happy(&mut s);

            let func: fn(&mut Schema) -> () = $schema;
            func(&mut s);
            s
        }
    };
}
