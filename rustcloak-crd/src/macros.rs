macro_rules! both_scopes {
    ($kind:literal, $shortname:literal, $cluster_kind:literal, $cluster_shortname:literal, $cluster_name:ident { $(#[$meta:meta])* pub struct $name:ident { $($(#[$field_meta:meta])* $vis:vis $field:ident : $type:ty),* $(,)? } }) => {
        #[derive(
            CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema,
        )]
        #[serde(rename_all = "camelCase")]
        #[kube(kind = $cluster_kind, shortname = $cluster_shortname)]
        $(#[$meta])*
        pub struct $cluster_name {
            #[serde(flatten)]
            pub spec: $name,
        }

        impl $crate::inner_spec::HasInnerSpec for $cluster_name {
            type InnerSpec = $name;

            fn inner_spec(&self) -> &Self::InnerSpec {
                &self.spec
            }
        }

        $crate::macros::namespace_scope!{
            $kind, $shortname {
                $(#[$meta])* pub struct $name {
                    $($(#[$field_meta])* $vis $field: $type),*
                }
            }
        }
    };
}

macro_rules! namespace_scope {
    ($kind: literal, $shortname:literal { $(#[$meta:meta])* pub struct $name:ident { $($(#[$field_meta:meta])* $vis:vis $field:ident : $type:ty),* $(,)? } }) => {
        #[derive(
            CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema,
        )]
        #[serde(rename_all = "camelCase")]
        $(#[$meta])*
        #[kube(kind = $kind, shortname = $shortname, namespaced)]
        pub struct $name {
            $( $(#[$field_meta])* $vis $field: $type),*
        }

        impl $crate::inner_spec::HasInnerSpec for $name {
            type InnerSpec = $name;

            fn inner_spec(&self) -> &Self::InnerSpec {
                &self
            }
        }
    };
}

pub(crate) use both_scopes;
pub(crate) use namespace_scope;
