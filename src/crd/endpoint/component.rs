use crate::crd::{
    api_object_impl, child_of, schema_patch, HasApiObject,
    KeycloakApiObjectOptions, KeycloakApiStatus,
};
use keycloak::types::ComponentRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakComponent",
    shortname = "kcco",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakComponentSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: ComponentRepresentation,
}

api_object_impl!(KeycloakComponent, ComponentRepresentation, id, component);

child_of!(KeycloakComponent, KeycloakRealm, realm_ref, "components");

crate::crd::route_impl!(KeycloakRealm / "components" / id: KeycloakComponent .. realm_ref: String);

schema_patch!(KeycloakComponent);
