use crate::crd::{
    child_of, endpoint_impl, HasEndpoint, KeycloakApiObjectOptions,
    KeycloakApiStatus,
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
    #[schemars(schema_with = "KeycloakComponent::schema")]
    pub definition: ComponentRepresentation,
}

endpoint_impl!(
    KeycloakComponent,
    ComponentRepresentation,
    id,
    component,
    |_| {}
);

child_of!(KeycloakComponent, KeycloakRealm, realm_ref, "components");
