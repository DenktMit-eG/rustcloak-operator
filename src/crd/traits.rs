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

    fn options(&self) -> Option<&KeycloakApiObjectOptions>;

    fn prefix() -> &'static str;
}

// sed 's/\$ref.*//; s/^\* spec\.validation\.openAPIV3Schema\.properties\[spec\]\.properties\[definition\]/s/; s/\.properties\[\([^]]*\)\]/.prop("\1")/g; s/\.items\./.array_item()./g; s/\.prop("\([^"]*\)")\.array_items()\.$/.remove("\1");/'
#[macro_export]
macro_rules! api_object_impl {
    ($name:ty, $def:ty, $prefix:literal) => {
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

            fn prefix() -> &'static str {
                $prefix
            }
        }
    };
}

pub trait HasRoute: Resource + Sized {
    type ParentType;
    type ParentRefType;
    fn id_ident() -> &'static str;
    fn mount_point(&self) -> &'static str;
    fn id_option(&self) -> Option<&str>;

    fn id(&self) -> String {
        self.id_option()
            .map_or_else(|| self.uid().unwrap(), |v| v.to_string())
    }

    fn parent_ref(&self) -> &Self::ParentRefType;
}

#[macro_export]
macro_rules! route_impl {

    ($parent_ty:ident / |$self_p:ident| $route:block / $id:ident: $self_ty:ident .. $ref:ident: $ref_ty:ty) => {
        impl $crate::crd::HasRoute for $self_ty {
            type ParentType = $parent_ty;
            type ParentRefType = $ref_ty;

            fn id_ident() -> &'static str {
                stringify!($id)
            }

            fn id_option(&self) -> Option<&str> {
                use kube::core::object::HasSpec;
                self.spec().definition.$id.as_deref()
            }

            fn mount_point(&self) -> &'static str {
                let $self_p = self;
                $route
            }

            fn parent_ref(&self) -> &Self::ParentRefType {
                use kube::core::object::HasSpec;
                &self.spec().$ref
            }
        }
    };
    ($parent_ty:ident / $route:literal / $id:ident: $self_ty:ident .. $ref:ident: $ref_ty:ty) => {
        $crate::route_impl!($parent_ty / |_x| { $route } / $id: $self_ty .. $ref: $ref_ty);
    };
}

pub use api_object_impl;
pub use route_impl;
