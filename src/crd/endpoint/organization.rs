use crate::crd::{
    child_of, endpoint_impl, HasEndpoint, KeycloakApiObjectOptions,
    KeycloakApiStatus,
};
use keycloak::types::OrganizationRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakOrganization",
    shortname = "kcc",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakOrganizationSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[schemars(schema_with = "KeycloakOrganizationSpec::schema")]
    pub definition: OrganizationRepresentation,
}

endpoint_impl!(
    KeycloakOrganizationSpec,
    OrganizationRepresentation,
    id,
    organization,
    |_| {}
);

child_of!(
    KeycloakOrganization,
    KeycloakRealm,
    realm_ref,
    "organizations"
);
