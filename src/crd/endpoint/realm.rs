use crate::crd::{
    endpoint_impl, HasEndpoint, ImmutableString, KeycloakApiObjectOptions,
    KeycloakApiStatus,
};
use keycloak::types::RealmRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakRealm",
    shortname = "kcrm",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakRealmSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub instance_ref: ImmutableString,
    #[schemars(schema_with = "KeycloakRealmSpec::schema")]
    pub definition: RealmRepresentation,
}

endpoint_impl!(KeycloakRealmSpec, RealmRepresentation, realm, realm, |s| {
    s.remove("groups")
        .remove("applications")
        .remove("clients")
        .remove("components")
        .remove("oauthClients");
});
