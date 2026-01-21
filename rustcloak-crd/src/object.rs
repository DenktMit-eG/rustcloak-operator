use crate::{InitWorkflow, KeycloakApiObjectOptions, refs::HasParent};

pub trait KeycloakRestObject: HasParent {
    type ParentObject;
    type Definition;
    const ID_FIELD: &'static str;
    const API_PREFIX: &'static str;

    fn init_workflow(&self) -> InitWorkflow;
    fn definition(&self) -> Option<&Self::Definition>;
    fn options(&self) -> Option<&KeycloakApiObjectOptions>;

    /// Get human-readable name for K8s resource naming (returns reference to avoid allocation)
    fn human_readable_name(&self) -> Option<&str>;
}

macro_rules! impl_object {
    ($api_prefix:literal <$parent_ref_type:ty> / |$def_v:ident| $mount_path:block / $id_lit:literal / |$name_v:ident| $name_expr:block for $object_type:ty => $definition_type:ty) => {
        impl $crate::object::KeycloakRestObject for $object_type {
            type ParentObject = <$parent_ref_type as $crate::refs::Ref>::Target;
            type Definition = $definition_type;

            const ID_FIELD: &'static str = $id_lit;
            const API_PREFIX: &'static str = $api_prefix;

            fn init_workflow(&self) -> $crate::InitWorkflow {
                let $def_v = self;
                $mount_path.into()
            }

            fn definition(&self) -> Option<&Self::Definition> {
                self.definition.as_ref()
            }

            fn options(&self) -> Option<&$crate::KeycloakApiObjectOptions> {
                self.options.as_ref()
            }

            fn human_readable_name(&self) -> Option<&str> {
                let $name_v = self;
                $name_expr
            }
        }

        impl $crate::refs::HasParent for $object_type {
            type ParentRef = $parent_ref_type;
            fn parent_ref(&self) -> &Self::ParentRef {
                &self.parent_ref
            }
        }
    };
}

pub(crate) use impl_object;
