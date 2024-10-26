use super::{KeycloakClient, KeycloakClientScope};
use crate::crd::{
    api_object_impl, schema_patch, ChildOf, ClientRef, ClientScopeRef,
    HasApiObject, KeycloakApiObjectOptions, KeycloakApiStatus,
};
use either::Either;
use keycloak::types::ProtocolMapperRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

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

api_object_impl!(KeycloakProtocolMapper, ProtocolMapperRepresentation, id, pm);

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

crate::crd::route_impl!(<Either<KeycloakClient, KeycloakClientScope>> / "protocol-mappers/models" / id: KeycloakProtocolMapper .. parent_ref: Either<ClientRef, ClientScopeRef>);

schema_patch!(KeycloakProtocolMapper);
