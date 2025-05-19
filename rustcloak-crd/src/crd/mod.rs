pub mod api_object;
mod client_credentials;
pub mod instance;
mod options;
mod representation;
mod role_mapping;
mod status;
mod user_credentials;

pub use client_credentials::*;
pub use options::*;
pub use representation::*;
pub use role_mapping::*;
pub use status::*;
pub use user_credentials::*;

macro_rules! __crd_struct {
    ($kind:literal, $shortname:literal, $(#[$meta:meta])* pub struct $name:ident $body:tt) => {
        #[derive(
            CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema,
        )]
        #[serde(rename_all = "camelCase")]
        $(#[$meta])*
        #[kube(kind = $kind, shortname = $shortname,
            group = "rustcloak.k8s.eboland.de",
            version = "v1beta1",
            category = "keycloak",
            category = "all",
            status = "KeycloakApiStatus",
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
        pub struct $name $body
    }
}

macro_rules! both_scopes {
    ($kind:literal, $shortname:literal, $cluster_kind:literal, $cluster_shortname:literal, $cluster_name:ident { $(#[$meta:meta])* pub struct $name:ident $body:tt}) => {
        $crate::crd::__crd_struct!{
            $cluster_kind, $cluster_shortname, $(#[$meta])*
            pub struct $cluster_name {
                #[serde(flatten)]
                pub spec: $name,
            }
        }

        impl $crate::inner_spec::HasInnerSpec for $cluster_name {
            type InnerSpec = $name;

            fn inner_spec(&self) -> &Self::InnerSpec {
                &self.spec
            }
        }

        $crate::crd::namespace_scope!{
            $kind, $shortname {
                $(#[$meta])* pub struct $name $body
            }
        }
    };
}

macro_rules! namespace_scope {
    ($kind: literal, $shortname:literal { $(#[$meta:meta])* pub struct $name:ident $body:tt }) => {
        $crate::crd::__crd_struct!{
            $kind, $shortname,
            $(#[$meta])*
            #[kube(namespaced)]
            pub struct $name $body
        }

        impl $crate::inner_spec::HasInnerSpec for $name {
            type InnerSpec = $name;

            fn inner_spec(&self) -> &Self::InnerSpec {
                &self
            }
        }
    };
}

pub(crate) use __crd_struct;
pub(crate) use both_scopes;
pub(crate) use namespace_scope;
