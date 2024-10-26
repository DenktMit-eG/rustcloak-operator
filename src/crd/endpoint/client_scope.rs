use crate::crd::{
    api_object_impl, schema_patch, ChildOf, HasApiObject,
    KeycloakApiObjectOptions, KeycloakApiStatus,
};
use keycloak::types::ClientScopeRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakClientScope",
    shortname = "kccs",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakClientScopeSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[schemars(schema_with = "schema")]
    pub definition: ClientScopeRepresentation,
}

api_object_impl!(KeycloakClientScope, ClientScopeRepresentation, id, cs);

impl ChildOf for KeycloakClientScope {
    type ParentRefType = String;
    type ParentType = KeycloakRealm;
    fn sub_path(&self) -> &'static str {
        if self.spec.is_template.unwrap_or(false) {
            "client-scopes"
        } else {
            "client-templates"
        }
    }

    fn parent_ref(&self) -> Self::ParentRefType {
        self.spec.realm_ref.clone()
    }
}

crate::crd::route_impl!(<KeycloakRealm> / |x| {
    if x.spec.is_template.unwrap_or(false) {
        "client-scopes"
    } else {
        "client-templates"
    }
} / id: KeycloakClientScope .. realm_ref: String);

schema_patch!(KeycloakClientScope);
