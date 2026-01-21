use crate::either::UntaggedEither;
use crate::keycloak_types::ProtocolMapperRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiStatus, crd::namespace_scope,
    impl_object, schema_patch, traits::impl_endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{client::ClientRef, client_scope::ClientScopeRef};

namespace_scope! {
    "KeycloakProtocolMapper",
    "kcpm" {
        #[kube(
            doc = "resource to define a Protocol Mapper within either a [KeycloakClient](./keycloakclient.md) or a [KeycloakClientScope](./keycloakclientscope.md)",
        )]
        /// the KeycloakProtocolMapper resource
        pub struct KeycloakProtocolMapperSpec {
            #[serde(default, flatten)]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            /// the name of the kubernetes object that created the parent resource.
            pub parent_ref: ParentRef,
            #[schemars(schema_with = "schema")]
            pub definition: Option<ProtocolMapperRepresentation>,
        }
    }
}

type ParentRef = UntaggedEither<ClientRef, ClientScopeRef>;

impl_object!("pm" <ParentRef> / |_d| {"protocol-mappers/models"} / "id" / |d| {
    d.definition.as_ref().and_then(|def| def.name.as_deref())
} for KeycloakProtocolMapperSpec => ProtocolMapperRepresentation);

impl_endpoint!(KeycloakProtocolMapper);

schema_patch!(KeycloakProtocolMapperSpec);
