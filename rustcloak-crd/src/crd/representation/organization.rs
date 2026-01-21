use super::realm::RealmRef;
use crate::keycloak_types::OrganizationRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiStatus, crd::namespace_scope,
    impl_object, schema_patch, traits::impl_endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakOrganization",
    "kcorg" {
        #[kube(
            doc = "resource to define an Organisation within a [KeyclaokRealm](./keycloakrealm.md)",
        )]
        /// the KeycloakOrganization resource
        pub struct KeycloakOrganizationSpec {
            #[serde(default, flatten)]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: RealmRef,
            #[schemars(schema_with = "schema")]
            pub definition: Option<OrganizationRepresentation>,
        }
    }
}

impl_object!("org" <RealmRef> / |_d| {"organizations"} / "id" / |d| {
    d.definition.as_ref().and_then(|def| def.name.as_deref())
} for KeycloakOrganizationSpec => OrganizationRepresentation);

impl_endpoint!(KeycloakOrganization);

schema_patch!(KeycloakOrganizationSpec);
