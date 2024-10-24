use crate::crd::{
    child_of, endpoint_impl, HasEndpoint, KeycloakApiObjectOptions,
    KeycloakApiStatus,
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
    #[schemars(schema_with = "KeycloakRequiredActionProviderSpec::schema")]
    pub definition: RequiredActionProviderRepresentation,
}

endpoint_impl!(
    KeycloakRequiredActionProviderSpec,
    RequiredActionProviderRepresentation,
    alias,
    rap,
    |_| {}
);

child_of!(
    KeycloakRequiredActionProvider,
    KeycloakRealm,
    realm_ref,
    "authentication/required-actions"
);
