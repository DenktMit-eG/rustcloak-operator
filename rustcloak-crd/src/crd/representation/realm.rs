use super::client::client_schema;
use crate::either::UntaggedEither;
use crate::keycloak_types::RealmRepresentation;
use crate::marker::ResourceMarker;
use crate::refs::ref_type;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiStatus, KeycloakApiStatusEndpoint,
    impl_object, inner_spec::HasInnerSpec, schema_patch, traits::Endpoint,
};
use crate::{both_scopes, instance::InstanceRef};
use either::Either;
use kube::{CustomResource, ResourceExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

both_scopes! {
   "KeycloakRealm", "kcrm", "ClusterKeycloakRealm", "ckcrm", ClusterKeycloakRealmSpec {
        #[kube(
            doc = "resource to define an Realm within a [KeyclaokInstance](./keycloakinstance.md)",
        )]
        /// the KeycloakRealm resource
        pub struct KeycloakRealmSpec {
            #[serde(default, flatten)]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: InstanceRef,
            #[schemars(schema_with = "schema")]
            pub definition: Option<RealmRepresentation>,
        }
    }
}

impl_object!("realm" <InstanceRef> / |_d| {"admin/realms"} / "realm" for KeycloakRealmSpec => RealmRepresentation);

impl Endpoint for KeycloakRealm {
    fn endpoint(&self) -> Option<&KeycloakApiStatusEndpoint> {
        self.status.as_ref().and_then(|s| s.endpoint.as_ref())
    }
    fn instance_ref(&self) -> Option<&InstanceRef> {
        Some(&self.inner_spec().parent_ref)
    }
    fn realm_ref(&self) -> Option<RealmRef> {
        Some(Either::Left(NamespacedRealmRef::from(self.name_any())).into())
    }
}

impl Endpoint for ClusterKeycloakRealm {
    fn endpoint(&self) -> Option<&KeycloakApiStatusEndpoint> {
        self.status.as_ref().and_then(|s| s.endpoint.as_ref())
    }
    fn instance_ref(&self) -> Option<&InstanceRef> {
        Some(&self.inner_spec().parent_ref)
    }
    fn realm_ref(&self) -> Option<RealmRef> {
        Some(Either::Right(ClusterRealmRef::from(self.name_any())).into())
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
        .remove("protocolMappers")
        .remove("authenticationFlows")
        .remove("authenticatorConfig")
        .remove("requiredActions")
        .remove("organizations")
        .remove("applications")
        .remove("components")
        .remove("oauthClients");
    client_schema(s.prop("adminPermissionsClient"));
});
ref_type!(
    NamespacedRealmRef,
    realm_ref,
    KeycloakRealm,
    "The name of the realm to which this object belongs to"
);
ref_type!(
    ClusterRealmRef,
    cluster_realm_ref,
    ClusterKeycloakRealm,
    "The name of the cluster realm to which this object belongs to"
);
pub type RealmRef = UntaggedEither<NamespacedRealmRef, ClusterRealmRef>;
