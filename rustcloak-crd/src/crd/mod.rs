mod api_object;
mod client_credentials;
mod instance;
mod patches;
mod representation;
mod role_mapping;
mod status;
mod user_credentials;

pub use api_object::*;
pub use client_credentials::*;
pub use instance::*;
pub use patches::*;
pub use representation::*;
pub use role_mapping::*;
pub use status::*;
pub use user_credentials::*;

macro_rules! both_scopes {
    ($kind:literal, $shortname:literal, $cluster_kind:literal, $cluster_shortname:literal, $cluster_name:ident { $(#[$meta:meta])* pub struct $name:ident { $($(#[$field_meta:meta])* $vis:vis $field:ident : $type:ty),* $(,)? } }) => {
        #[derive(
            CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema,
        )]
        #[serde(rename_all = "camelCase")]
        #[kube(kind = $cluster_kind, shortname = $cluster_shortname,
            printcolumn = r#"{
                    "name":"Ready",
                    "type":"boolean",
                    "description":"true if the realm is ready",
                    "jsonPath":".status.ready"
                }"#,
            printcolumn = r#"{
                    "name":"Status",
                    "type":"string",
                    "description":"Status String of the resource",
                    "jsonPath":".status.status"
                }"#,
            printcolumn = r#"{
                    "name":"Age",
                    "type":"date",
                    "description":"time since the realm was created",
                    "jsonPath":".metadata.creationTimestamp"
                }"#
        )]
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

        impl From<$name> for $cluster_name {
            fn from(spec: $name) -> Self {
                Self { spec }
            }
        }

        impl From<$cluster_name> for $name {
            fn from(from: $cluster_name) -> Self {
                from.spec
            }
        }

        $crate::crd::namespace_scope!{
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
        #[kube(kind = $kind, shortname = $shortname, namespaced,
            printcolumn = r#"{
                    "name":"Ready",
                    "type":"boolean",
                    "description":"true if the realm is ready",
                    "jsonPath":".status.ready"
                }"#,
            printcolumn = r#"{
                    "name":"Status",
                    "type":"string",
                    "description":"Status String of the resource",
                    "jsonPath":".status.status"
                }"#,
            printcolumn = r#"{
                    "name":"Age",
                    "type":"date",
                    "description":"time since the realm was created",
                    "jsonPath":".metadata.creationTimestamp"
                }"#
        )]
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
