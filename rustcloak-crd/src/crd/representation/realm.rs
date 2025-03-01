use crate::keycloak_types::RealmRepresentation;
use crate::{
    ImmutableString, KeycloakApiObjectOptions, KeycloakApiPatchList,
    KeycloakApiStatus, KeycloakApiStatusEndpoint, KeycloakInstance,
    impl_object, inner_spec::HasInnerSpec, macros::both_scopes, schema_patch,
    traits::Endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

both_scopes! {
    "KeycloakRealm", "kcrm", "ClusterKeycloakRealm", "ckcrm", ClusterKeycloakRealmSpec {
        #[kube(
            doc = "resource to define an Realm within a [KeyclaokInstance](./keycloakinstance.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
            printcolumn = r#"{
                    "name":"Ready",
                    "type":"boolean",
                    "description":"true if the realm is ready",
                    "jsonPath":".status.ready"
                }"#,
            printcolumn = r#"{
                    "name":"Status",
                    "type":"string",
                    "description":"Status String of the resource",
                    "jsonPath":".status.status"
                }"#,
            printcolumn = r#"{
                    "name":"Age",
                    "type":"date",
                    "description":"time since the realm was created",
                    "jsonPath":".metadata.creationTimestamp"
                }"#
        )]
        /// the KeycloakRealm resource
        pub struct KeycloakRealmSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            /// The name of the instance to which this realm belongs
            pub instance_ref: ImmutableString,
            #[schemars(schema_with = "schema")]
            pub definition: RealmRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_object!("realm" <instance_ref: String => KeycloakInstance> / |_d| {"admin/realms"} / realm for KeycloakRealmSpec => RealmRepresentation);

impl Endpoint for KeycloakRealm {
    fn endpoint(&self) -> Option<&KeycloakApiStatusEndpoint> {
        self.status.as_ref().and_then(|s| s.endpoint.as_ref())
    }
    fn instance_ref(&self) -> Option<&str> {
        Some(self.inner_spec().instance_ref.as_str())
    }
    fn resource_path(&self) -> Option<&str> {
        self.endpoint().map(|e| e.resource_path.as_str())
    }
}

impl Endpoint for ClusterKeycloakRealm {
    fn endpoint(&self) -> Option<&KeycloakApiStatusEndpoint> {
        self.status.as_ref().and_then(|s| s.endpoint.as_ref())
    }
    fn instance_ref(&self) -> Option<&str> {
        Some(self.inner_spec().instance_ref.as_str())
    }
    fn resource_path(&self) -> Option<&str> {
        self.endpoint().map(|e| e.resource_path.as_str())
    }
}

schema_patch!(KeycloakRealmSpec: |s| {
    s.remove("groups")
        .remove("applications")
        .remove("clients")
        .remove("components")
        .remove("oauthClients")
        .remove("adminPermissionsClient");
});
