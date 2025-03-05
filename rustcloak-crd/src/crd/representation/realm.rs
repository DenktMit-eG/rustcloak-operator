use crate::keycloak_types::RealmRepresentation;
use crate::marker::ResourceMarker;
use crate::refs::ref_type;
use crate::{InstanceRef, both_scopes};
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    KeycloakApiStatusEndpoint, impl_object, inner_spec::HasInnerSpec,
    schema_patch, traits::Endpoint,
};
use either::Either;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::client_schema;

both_scopes! {
   "KeycloakRealm", "kcrm", "ClusterKeycloakRealm", "ckcrm", ClusterKeycloakRealmSpec {
        #[kube(
            doc = "resource to define an Realm within a [KeyclaokInstance](./keycloakinstance.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1beta1",
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

impl Endpoint for ClusterKeycloakRealm {
    fn endpoint(&self) -> Option<&KeycloakApiStatusEndpoint> {
        self.status.as_ref().and_then(|s| s.endpoint.as_ref())
    }
    fn instance_ref(&self) -> Option<&InstanceRef> {
        Some(&self.inner_spec().parent_ref)
    }
}

impl crate::marker::HasMarker for KeycloakRealm {
    type Marker = ResourceMarker<<Self as kube::Resource>::Scope>;
}

impl crate::marker::HasMarker for ClusterKeycloakRealm {
    type Marker = ResourceMarker<<Self as kube::Resource>::Scope>;
}

schema_patch!(KeycloakRealmSpec: |s| {
    s.remove("groups")
        .remove("users")
        .remove("federatedUsers")
        .remove("clients")
        .remove("clientScopes")
        .remove("identityProviders")
        .remove("identityProviderMappers")
        .remove("protocolMappers")
        .remove("authenticationFlows")
        .remove("authenticatorConfig")
        .remove("requiredActions")
        .remove("organizations")
        .remove("applications")
        .remove("components")
        .remove("oauthClients")
        .remove("roles");
    client_schema(s.prop("adminPermissionsClient"));
});
ref_type!(
    NamespacedRealmRef,
    instance_ref,
    KeycloakRealm,
    "The name of the realm to which this object belongs to"
);
ref_type!(
    ClusterRealmRef,
    cluster_instance_ref,
    ClusterKeycloakRealm,
    "The name of the cluster realm to which this object belongs to"
);
pub type RealmRef = Either<NamespacedRealmRef, ClusterRealmRef>;
