use super::KeycloakApiObjectOptions;
use kube::{Resource, ResourceExt};
use schemars::JsonSchema;
use serde::Serialize;

pub trait HasApiObject
where
    Self: Resource + Sized,
    Self::Definition: JsonSchema + Serialize,
{
    type Definition;
    fn definition(&self) -> &Self::Definition;
    fn primary_key() -> &'static str;

    fn primary_key_value_opt(&self) -> Option<&str>;

    fn primary_key_value(&self) -> String {
        self.primary_key_value_opt()
            .map_or_else(|| self.uid().unwrap(), |v| v.to_string())
    }

    fn options(&self) -> Option<&KeycloakApiObjectOptions>;

    fn prefix() -> &'static str;
}

// sed 's/\$ref.*//; s/^\* spec\.validation\.openAPIV3Schema\.properties\[spec\]\.properties\[definition\]/s/; s/\.properties\[\([^]]*\)\]/.prop("\1")/g; s/\.items\./.array_item()./g; s/\.prop("\([^"]*\)")\.array_items()\.$/.remove("\1");/'
#[macro_export]
macro_rules! api_object_impl {
    ($name:ty, $def:ty, $primary_key:ident, $prefix:ident) => {
        impl $crate::crd::HasApiObject for $name {
            type Definition = $def;
            fn definition(&self) -> &Self::Definition {
                &self.spec.definition
            }

            fn options(
                &self,
            ) -> Option<&$crate::crd::KeycloakApiObjectOptions> {
                self.spec.options.as_ref()
            }

            fn primary_key() -> &'static str {
                stringify!($primary_key)
            }

            fn primary_key_value_opt(&self) -> Option<&str> {
                self.definition().$primary_key.as_deref()
            }

            fn prefix() -> &'static str {
                stringify!($prefix)
            }
        }
    };
}

pub trait ChildOf {
    type ParentType;
    type ParentRefType;
    fn sub_path(&self) -> &'static str;
    fn parent_ref(&self) -> Self::ParentRefType;
}

pub trait HasParentType {
    type Parent;
}

#[macro_export]
macro_rules! child_of {
    ($name:ty, $parent:ty, $ref:ident, $sub_path:expr) => {
        use kube::core::object::HasSpec;
        impl $crate::crd::ChildOf for $name {
            type ParentType = $parent;
            type ParentRefType = String;
            fn sub_path(&self) -> &'static str {
                $sub_path
            }

            fn parent_ref(&self) -> Self::ParentRefType {
                self.spec().$ref.to_string()
            }
        }
        impl $crate::crd::HasParentType for $name {
            type Parent = $parent;
        }
    };
}

pub trait HasRoute {
    type ParentType;
    type ParentRefType;
    fn id_ident() -> &'static str;
    fn route(&self) -> &'static str;
    fn id(&self) -> Option<&String>;
    fn route_parent_ref(&self) -> &Self::ParentRefType;
}

#[macro_export]
macro_rules! route_impl {

    (<$parent_ty:ty> / |$self_p:ident| $route:block / $id:ident: $self_ty:ident .. $ref:ident: $ref_ty:ty) => {
        impl $crate::crd::HasRoute for $self_ty {
            type ParentType = $parent_ty;
            type ParentRefType = $ref_ty;

            fn id_ident() -> &'static str {
                stringify!($id)
            }

            fn id(&self) -> Option<&String> {
                use kube::core::object::HasSpec;
                self.spec().definition.$id.as_ref()
            }

            fn route(&self) -> &'static str {
                let $self_p = self;
                $route
            }

            fn route_parent_ref(&self) -> &Self::ParentRefType {
                use kube::core::object::HasSpec;
                &self.spec().$ref
            }
        }
    };
    (<$parent_ty:ty> / $route:literal / $id:ident: $self_ty:ident .. $ref:ident: $ref_ty:ty) => {
        $crate::route_impl!(<$parent_ty> / |_x| { $route } / $id: $self_ty .. $ref: $ref_ty);
    };
    ($parent_ty:ident / $route:literal / $id:ident: $self_ty:ident .. $ref:ident: $ref_ty:ty) => {
        $crate::route_impl!(<$parent_ty> / |_x| { $route } / $id: $self_ty .. $ref: $ref_ty);
    };
}

pub use api_object_impl;
pub use child_of;
pub use route_impl;
