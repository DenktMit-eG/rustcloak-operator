use crate::InstanceRef;
use crate::keycloak_types::RealmRepresentation;
use crate::refs::ref_type;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    KeycloakApiStatusEndpoint, crd::namespace_scope, impl_object,
    inner_spec::HasInnerSpec, schema_patch, traits::Endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::client_schema;

namespace_scope! {
    "KeycloakRealm", "kcrm" {
///both_scopes! {
//    "KeycloakRealm", "kcrm", "ClusterKeycloakRealm", "ckcrm", ClusterKeycloakRealmSpec {
        #[kube(
            doc = "resource to define an Realm within a [KeyclaokInstance](./keycloakinstance.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
        )]
        /// the KeycloakRealm resource
        pub struct KeycloakRealmSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: InstanceRef,
            #[schemars(schema_with = "schema")]
            pub definition: RealmRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_object!("realm" <InstanceRef> / |_d| {"admin/realms"} / realm for KeycloakRealmSpec => RealmRepresentation);

impl Endpoint for KeycloakRealm {
    fn endpoint(&self) -> Option<&KeycloakApiStatusEndpoint> {
        self.status.as_ref().and_then(|s| s.endpoint.as_ref())
    }
    fn instance_ref(&self) -> Option<&InstanceRef> {
        Some(&self.inner_spec().parent_ref)
    }
}

//impl Endpoint for ClusterKeycloakRealm {
//    fn endpoint(&self) -> Option<&KeycloakApiStatusEndpoint> {
//        self.status.as_ref().and_then(|s| s.endpoint.as_ref())
//    }
//    fn instance_ref(&self) -> Option<&InstanceRef> {
//        Some(&self.inner_spec().parent_ref)
//    }
//}

schema_patch!(KeycloakRealmSpec: |s| {
    s.remove("groups")
        .remove("applications")
        .remove("clients")
        .remove("components")
        .remove("oauthClients");
    client_schema(s.prop("adminPermissionsClient"));
});
ref_type!(RealmRef, instance_ref, KeycloakRealm);
//ref_type!(NamespacedRealmRef, instance_ref, KeycloakRealm);
//ref_type!(ClusterRealmRef, cluster_instance_ref, ClusterKeycloakRealm);
//pub type RealmRef = Either<NamespacedRealmRef, ClusterRealmRef>;
