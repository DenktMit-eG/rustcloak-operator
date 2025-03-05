use crate::keycloak_types::OrganizationRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    crd::namespace_scope, impl_object, schema_patch, traits::impl_endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::RealmRef;

namespace_scope! {
    "KeycloakOrganization",
    "kcorg" {
        #[kube(
            doc = "resource to define an Organisation within a [KeyclaokRealm](./keycloakrealm.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1beta1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
        )]
        /// the KeycloakOrganization resource
        pub struct KeycloakOrganizationSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: RealmRef,
            #[schemars(schema_with = "schema")]
            pub definition: OrganizationRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_object!("org" <RealmRef> / |_d| {"organizations"} / id for KeycloakOrganizationSpec => OrganizationRepresentation);

impl_endpoint!(KeycloakOrganization);

schema_patch!(KeycloakOrganizationSpec);
