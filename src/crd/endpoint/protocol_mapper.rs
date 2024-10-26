use super::{KeycloakClient, KeycloakClientScope};
use crate::crd::{
    endpoint_impl, schema_patch, ChildOf, HasEndpoint,
    KeycloakApiObjectOptions, KeycloakApiStatus,
};
use either::Either;
use keycloak::types::ProtocolMapperRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientRef {
    /// This indicates that the ProtocolMapper is a child of a KeycloakClient. Mutually exclusive
    /// with `clientScopeRef`
    pub client_ref: String,
}
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeRef {
    /// This indicates that the ProtocolMapper is a child of a KeycloakClientScope. Mutually
    /// exclusive with `clientRef`
    pub client_scope_ref: String,
}
#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakProtocolMapper",
    shortname = "kcpm",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakProtocolMapperSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    #[serde(flatten)]
    pub parent_ref: Either<ClientRef, ClientScopeRef>,
    #[schemars(schema_with = "schema")]
    pub definition: ProtocolMapperRepresentation,
}

endpoint_impl!(KeycloakProtocolMapper, ProtocolMapperRepresentation, id, pm);

impl ChildOf for KeycloakProtocolMapper {
    type ParentType = Either<KeycloakClient, KeycloakClientScope>;
    type ParentRefType = Either<String, String>;
    fn sub_path(&self) -> &'static str {
        "protocol-mappers/models"
    }

    fn parent_ref(&self) -> Self::ParentRefType {
        self.spec
            .parent_ref
            .clone()
            .map_either(|x| x.client_ref, |x| x.client_scope_ref)
    }
}

schema_patch!(KeycloakProtocolMapper);
