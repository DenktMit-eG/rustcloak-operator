use crate::keycloak_types::RequiredActionProviderRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiStatus, crd::namespace_scope,
    impl_object, schema_patch, traits::impl_endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::RealmRef;

namespace_scope! {
    "KeycloakRequiredActionProvider", "kcrap" {
        #[kube(
            doc = "resource to define an Required Action Provider within a [KeyclaokRealm](./keycloakrealm.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1beta1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
        )]
        /// the KeycloakRequiredActionProvider resource
        pub struct KeycloakRequiredActionProviderSpec {
            #[serde(default, flatten)]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: RealmRef,
            #[schemars(schema_with = "schema")]
            pub definition: Option<RequiredActionProviderRepresentation>,
        }
    }
}

impl_object!("rap" <RealmRef> / |_d| {"authentication/required-actions"} / "alias" for KeycloakRequiredActionProviderSpec => RequiredActionProviderRepresentation);

impl_endpoint!(KeycloakRequiredActionProvider);

schema_patch!(KeycloakRequiredActionProviderSpec);
