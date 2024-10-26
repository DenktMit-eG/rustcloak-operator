use crate::crd::{
    api_object_impl, child_of, schema_patch, HasApiObject,
    KeycloakApiObjectOptions, KeycloakApiStatus,
};
use keycloak::types::RequiredActionProviderRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakRequiredActionProvider",
    shortname = "kcrap",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakRequiredActionProviderSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: RequiredActionProviderRepresentation,
}

api_object_impl!(
    KeycloakRequiredActionProvider,
    RequiredActionProviderRepresentation,
    alias,
    rap
);

child_of!(
    KeycloakRequiredActionProvider,
    KeycloakRealm,
    realm_ref,
    "authentication/required-actions"
);

crate::crd::route_impl!(KeycloakRealm / "protocol-mappers/models" / alias: KeycloakRequiredActionProvider .. realm_ref: String);

schema_patch!(KeycloakRequiredActionProvider);
