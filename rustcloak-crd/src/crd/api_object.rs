use crate::{
    ImmutableString, KeycloakApiStatus,
    crd::both_scopes,
    either::UntaggedEither,
    marker::{HasMarker, ResourceMarker},
    refs::{HasParent, ref_type},
};
use k8s_openapi::api::core::v1::EnvVar;
use kube::{CustomResource, Resource};
use schemars::{JsonSchema, SchemaGenerator, schema::Schema};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::DisplayFromStr;

use super::InstanceRef;

both_scopes! {
    "KeycloakApiObject", "kcapi", "ClusterKeycloakApiObject", "ckcapi", ClusterKeycloakApiObjectSpec {
        #[kube(
            doc = "Custom Resource for Keycloak API requests. The user should not use this resource directly.",
            group = "rustcloak.k8s.eboland.de",
            version = "v1beta1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
            printcolumn = r#"{
                    "name":"Instance",
                    "type":"string",
                    "description":"Instance that API request is sent to",
                    "jsonPath":".spec.endpoint.instanceRef"
                }"#,
        )]
        /// defines an API request to the Keycloak Admin API.
        pub struct KeycloakApiObjectSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            pub endpoint: KeycloakApiEndpoint,
            pub immutable_payload: ImmutableString,
            pub payload: String,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub vars: Vec<EnvVar>,
        }
    }
}
impl HasMarker for KeycloakApiObject {
    type Marker = ResourceMarker<<Self as Resource>::Scope>;
}

impl HasMarker for ClusterKeycloakApiObject {
    type Marker = ResourceMarker<<Self as Resource>::Scope>;
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct KeycloakApiEndpointParent {
    #[serde(flatten)]
    pub parent_ref: ApiObjectRef,
    pub sub_path: ImmutableString,
}
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub enum KeycloakApiEndpointPath {
    // BUG: while the values of Path and Parent variants are both ImmutableString, there's
    // there's currently no guard in place prevent the user from replacing the Parent variant with
    // a Path variant. This is a potential source of bugs.
    #[serde(rename = "path")]
    Path(ImmutableString),
    #[serde(rename = "parent")]
    Parent(KeycloakApiEndpointParent),
}

#[serde_with::serde_as]
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakApiEndpoint {
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[schemars(schema_with = "http_method_schema")]
    pub init_workflow: Option<http::Method>,

    #[serde(flatten)]
    pub instance_ref: InstanceRef,
    #[serde(flatten)]
    pub path_def: KeycloakApiEndpointPath,
}
fn http_method_schema(generator: &mut SchemaGenerator) -> Schema {
    let mut schema = String::json_schema(generator);
    let Schema::Object(ref mut schema_obj) = schema else {
        unreachable!();
    };
    schema_obj.enum_values = Some(vec![
        Value::String(http::Method::GET.to_string()),
        Value::String(http::Method::POST.to_string()),
    ]);

    schema
}

impl KeycloakApiEndpoint {
    pub fn new(instance_ref: &InstanceRef, path: &str) -> Self {
        let path = path.to_string().into();
        let instance_ref = instance_ref.clone();
        Self {
            instance_ref,
            init_workflow: Some(http::Method::POST),
            path_def: KeycloakApiEndpointPath::Path(path),
        }
    }
}

impl HasParent for KeycloakApiEndpointParent {
    type ParentRef = ApiObjectRef;

    fn parent_ref(&self) -> &Self::ParentRef {
        &self.parent_ref
    }
}

ref_type!(
    NamespacedApiObjectRef,
    parent_ref,
    KeycloakApiObject,
    "The name of the API Object to which this object belongs to."
);
ref_type!(
    ClusterApiObjectRef,
    cluster_parent_ref,
    ClusterKeycloakApiObject,
    "The name of the cluster API Object to which this object belongs to."
);
pub type ApiObjectRef =
    UntaggedEither<NamespacedApiObjectRef, ClusterApiObjectRef>;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
/// Options for the request to the Keycloak Admin API.
pub struct KeycloakApiObjectOptions {}
