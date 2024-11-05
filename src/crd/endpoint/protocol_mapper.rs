use crate::crd::{
    api_object_impl, schema_patch, ClientRef, ClientScopeRef,
    KeycloakApiObjectOptions, KeycloakApiStatus, KeycloakClient,
    KeycloakClientScope,
};
use either::Either;
use keycloak::types::ProtocolMapperRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};
use up_impl::OneOf;

type Parents = Either<KeycloakClient, KeycloakClientScope>;
type ParentRef = Either<ClientRef, ClientScopeRef>;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakProtocolMapper",
    shortname = "kcpm",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced,
    printcolumn = r#"{
            "name":"Ready",
            "type":"boolean",
            "description":"",
            "jsonPath":".status.ready"
        }"#,
    printcolumn = r#"{
            "name":"Status",
            "type":"string",
            "description":"",
            "jsonPath":".status.status"
        }"#,
    printcolumn = r#"{
            "name":"Age",
            "type":"date",
            "description":"",
            "jsonPath":".metadata.creationTimestamp"
        }"#
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakProtocolMapperSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    #[serde(flatten)]
    pub parent_ref: ParentRef,
    #[schemars(schema_with = "schema")]
    pub definition: ProtocolMapperRepresentation,
}

api_object_impl!(KeycloakProtocolMapper, ProtocolMapperRepresentation, "pm");

crate::crd::route_impl!(Parents / "protocol-mappers/models"
    / id: KeycloakProtocolMapper .. parent_ref: OneOf<String, String>);

schema_patch!(KeycloakProtocolMapper);
