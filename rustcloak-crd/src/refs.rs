use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

macro_rules! ref_type {
    ($type:ident, $field:ident) => {
        #[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
        #[serde(rename_all = "camelCase")]
        pub struct $type {
            pub $field: String,
        }
        impl From<$type> for String {
            fn from(val: $type) -> Self {
                val.$field
            }
        }
        impl From<String> for $type {
            fn from(val: String) -> $type {
                $type { $field: val }
            }
        }
        impl AsRef<str> for $type {
            fn as_ref(&self) -> &str {
                &self.$field
            }
        }
    };
}

ref_type!(RealmRef, realm_ref);
ref_type!(ClientRef, client_ref);
ref_type!(ClientScopeRef, client_scope_ref);
ref_type!(ComponentRef, component_ref);
ref_type!(SubGroupRef, parent_group_ref);
