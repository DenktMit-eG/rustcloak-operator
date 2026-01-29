use jsonschema::Validator;
use rustcloak_crd::api_object::{ClusterKeycloakApiObject, KeycloakApiObject};
use rustcloak_crd::authentication_flow::KeycloakAuthenticationFlow;
use rustcloak_crd::authenticator_config::KeycloakAuthenticatorConfig;
use rustcloak_crd::client::KeycloakClient;
use rustcloak_crd::client_scope::KeycloakClientScope;
use rustcloak_crd::component::KeycloakComponent;
use rustcloak_crd::group::KeycloakGroup;
use rustcloak_crd::identity_provider::KeycloakIdentityProvider;
use rustcloak_crd::identity_provider_mapper::KeycloakIdentityProviderMapper;
use rustcloak_crd::instance::{ClusterKeycloakInstance, KeycloakInstance};
use rustcloak_crd::organization::KeycloakOrganization;
use rustcloak_crd::protocol_mapper::KeycloakProtocolMapper;
use rustcloak_crd::realm::{ClusterKeycloakRealm, KeycloakRealm};
use rustcloak_crd::required_action_provider::KeycloakRequiredActionProvider;
use rustcloak_crd::resource::KeycloakResource;
use rustcloak_crd::role::KeycloakRole;
use rustcloak_crd::scope::KeycloakScope;
use rustcloak_crd::user::KeycloakUser;
use rustcloak_crd::KeycloakClientCredential;
use rustcloak_crd::KeycloakRoleMapping;
use rustcloak_crd::KeycloakUserCredential;
use schemars::{schema_for, JsonSchema};
use serde_json::Value;
use std::fs;
use std::path::Path;

fn schema_for_type<T: JsonSchema>() -> Value {
    let schema = schema_for!(T);
    serde_json::to_value(schema).unwrap()
}

fn load_yaml(path: &Path) -> Value {
    let content = fs::read_to_string(path).expect("read file");
    let yaml: serde_yaml::Value = serde_yaml::from_str(&content).expect("parse yaml");
    serde_json::to_value(yaml).expect("convert to json")
}

fn validate_manifest<T: JsonSchema>(manifest: &Value) -> Result<(), String> {
    let schema = schema_for_type::<T>();
    let validator = Validator::new(&schema).map_err(|e| e.to_string())?;

    match validator.validate(manifest) {
        Ok(()) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

fn validate_by_kind(manifest: &Value) -> Result<(), String> {
    let kind = manifest["kind"].as_str().ok_or("missing kind")?;

    match kind {
        "KeycloakInstance" => validate_manifest::<KeycloakInstance>(manifest),
        "ClusterKeycloakInstance" => validate_manifest::<ClusterKeycloakInstance>(manifest),
        "KeycloakRealm" => validate_manifest::<KeycloakRealm>(manifest),
        "ClusterKeycloakRealm" => validate_manifest::<ClusterKeycloakRealm>(manifest),
        "KeycloakClient" => validate_manifest::<KeycloakClient>(manifest),
        "KeycloakUser" => validate_manifest::<KeycloakUser>(manifest),
        "KeycloakRole" => validate_manifest::<KeycloakRole>(manifest),
        "KeycloakGroup" => validate_manifest::<KeycloakGroup>(manifest),
        "KeycloakClientScope" => validate_manifest::<KeycloakClientScope>(manifest),
        "KeycloakIdentityProvider" => validate_manifest::<KeycloakIdentityProvider>(manifest),
        "KeycloakIdentityProviderMapper" => {
            validate_manifest::<KeycloakIdentityProviderMapper>(manifest)
        }
        "KeycloakAuthenticationFlow" => validate_manifest::<KeycloakAuthenticationFlow>(manifest),
        "KeycloakAuthenticatorConfig" => {
            validate_manifest::<KeycloakAuthenticatorConfig>(manifest)
        }
        "KeycloakComponent" => validate_manifest::<KeycloakComponent>(manifest),
        "KeycloakOrganization" => validate_manifest::<KeycloakOrganization>(manifest),
        "KeycloakProtocolMapper" => validate_manifest::<KeycloakProtocolMapper>(manifest),
        "KeycloakRequiredActionProvider" => {
            validate_manifest::<KeycloakRequiredActionProvider>(manifest)
        }
        "KeycloakResource" => validate_manifest::<KeycloakResource>(manifest),
        "KeycloakScope" => validate_manifest::<KeycloakScope>(manifest),
        "KeycloakRoleMapping" => validate_manifest::<KeycloakRoleMapping>(manifest),
        "KeycloakClientCredential" => validate_manifest::<KeycloakClientCredential>(manifest),
        "KeycloakUserCredential" => validate_manifest::<KeycloakUserCredential>(manifest),
        "KeycloakApiObject" => validate_manifest::<KeycloakApiObject>(manifest),
        "ClusterKeycloakApiObject" => validate_manifest::<ClusterKeycloakApiObject>(manifest),
        _ => Err(format!("unknown kind: {}", kind)),
    }
}

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

#[test]
fn test_valid_manifests() {
    let dir = fixtures_dir().join("valid");
    for entry in fs::read_dir(&dir).expect("read valid fixtures") {
        let path = entry.unwrap().path();
        if path
            .extension()
            .map_or(false, |e| e == "yaml" || e == "yml")
        {
            let manifest = load_yaml(&path);
            let result = validate_by_kind(&manifest);
            assert!(
                result.is_ok(),
                "Valid manifest {:?} failed: {:?}",
                path,
                result
            );
        }
    }
}

#[test]
fn test_invalid_manifests() {
    let dir = fixtures_dir().join("invalid");
    for entry in fs::read_dir(&dir).expect("read invalid fixtures") {
        let path = entry.unwrap().path();
        if path
            .extension()
            .map_or(false, |e| e == "yaml" || e == "yml")
        {
            let manifest = load_yaml(&path);
            let result = validate_by_kind(&manifest);
            assert!(
                result.is_err(),
                "Invalid manifest {:?} should have failed",
                path
            );
        }
    }
}
