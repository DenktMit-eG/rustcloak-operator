use super::realm::RealmRef;
use crate::keycloak_types::ClientRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiStatus,
    crd::namespace_scope,
    impl_object,
    refs::ref_type,
    schema_patch,
    traits::{SecretKeyNames, impl_endpoint},
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakClientSecretReference {
    pub secret_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret_key: Option<String>,
}

namespace_scope! {
    "KeycloakClient", "kcc" {
        #[kube(
            doc = "resource to define a Client within a [KeycloakRealm](./keycloakrealm.md)",
        )]
        /// the KeycloakClient resource
        pub struct KeycloakClientSpec {
            #[serde(default, flatten)]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: RealmRef,
            #[schemars(schema_with = "schema")]
            pub definition: Option<ClientRepresentation>,
            pub client_secret: Option<KeycloakClientSecretReference>,
        }
    }
}

impl SecretKeyNames<2> for KeycloakClientSecretReference {
    const DEFAULTS: [&'static str; 2] = ["client_id", "client_secret"];

    fn secret_key_names_opts(&self) -> [&Option<String>; 2] {
        [&self.client_id_key, &self.client_secret_key]
    }
}

impl_object!("client" <RealmRef> / |_d| {"clients"} / "id" / |d| {
    d.definition.as_ref().and_then(|def| def.client_id.as_deref())
} for KeycloakClientSpec => ClientRepresentation);

impl_endpoint!(KeycloakClient);

pub(crate) fn client_schema(s: &mut Schema) {
    s.prop("authorizationSettings")
        .prop("policies")
        .array_item()
        .prop("resourcesData")
        .array_item()
        .prop("scopes")
        .array_item()
        .remove("policies")
        .remove("resources")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("policies")
        .array_item()
        .prop("resourcesData")
        .array_item()
        .prop("scopesUma")
        .array_item()
        .remove("policies")
        .remove("resources")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("policies")
        .array_item()
        .prop("scopesData")
        .array_item()
        .prop("resources")
        .array_item()
        .remove("scopes")
        .remove("scopesUma")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("policies")
        .array_item()
        .prop("scopesData")
        .array_item()
        .remove("policies")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("resources")
        .array_item()
        .prop("scopes")
        .array_item()
        .prop("policies")
        .array_item()
        .remove("resourcesData")
        .remove("scopesData")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("resources")
        .array_item()
        .prop("scopes")
        .array_item()
        .remove("resources")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("resources")
        .array_item()
        .prop("scopesUma")
        .array_item()
        .prop("policies")
        .array_item()
        .remove("resourcesData")
        .remove("scopesData")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("resources")
        .array_item()
        .prop("scopesUma")
        .array_item()
        .remove("resources")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("scopes")
        .array_item()
        .prop("policies")
        .array_item()
        .prop("resourcesData")
        .array_item()
        .remove("scopes")
        .remove("scopesUma")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("scopes")
        .array_item()
        .prop("policies")
        .array_item()
        .remove("scopesData")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("scopes")
        .array_item()
        .prop("resources")
        .array_item()
        .remove("scopes")
        .additional_properties();
    s.prop("authorizationSettings")
        .prop("scopes")
        .array_item()
        .prop("resources")
        .array_item()
        .remove("scopesUma")
        .additional_properties();
}
schema_patch!(KeycloakClientSpec: |s| {
    client_schema(s);
});

ref_type!(
    ClientRef,
    client_ref,
    KeycloakClient,
    "The kubernetes resources name of a KeycloakClient object."
);
