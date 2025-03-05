use crate::keycloak_types::ClientScopeRepresentation;
use crate::refs::ref_type;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    crd::namespace_scope, impl_object, schema_patch, traits::impl_endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::RealmRef;

namespace_scope! {
    "KeycloakClientScope", "kccs" {
        #[kube(
            doc = "resource to define a Scope within a [KeycloakClient](./keycloakclient.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1beta1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
        )]
        /// the KeycloakClientScope resource
        pub struct KeycloakClientScopeSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: RealmRef,
            // TODO: is_template should be immutable. We can't do immutable options yet.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub is_template: Option<bool>,
            #[schemars(schema_with = "schema")]
            pub definition: ClientScopeRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_endpoint!(KeycloakClientScope);

impl_object!("scope" <RealmRef> / |d| {
    if d.is_template == Some(true) {
        "client-scopes"
    } else {
        "client-templates"
    }
} / id for KeycloakClientScopeSpec => ClientScopeRepresentation);

schema_patch!(KeycloakClientScopeSpec);

ref_type!(ClientScopeRef, client_scope_ref, KeycloakClientScope);
