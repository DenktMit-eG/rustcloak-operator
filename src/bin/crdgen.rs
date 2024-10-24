use kube::CustomResourceExt;
use rustcloak_operator::crd::*;

fn main() {
    let crds = [
        KeycloakInstance::crd(),
        KeycloakApiObject::crd(),
        KeycloakAuthenticationFlow::crd(),
        KeycloakAuthenticatorConfig::crd(),
        KeycloakClient::crd(),
        KeycloakClientScope::crd(),
        KeycloakComponent::crd(),
        KeycloakGroup::crd(),
        KeycloakIdentityProvider::crd(),
        KeycloakIdentityProviderMapper::crd(),
        KeycloakOrganization::crd(),
        KeycloakProtocolMapper::crd(),
        KeycloakRealm::crd(),
        KeycloakRequiredActionProvider::crd(),
        //KeycloakResource::crd(),
        KeycloakRole::crd(),
        KeycloakScope::crd(),
        KeycloakUser::crd(),
    ];
    for crd in crds.iter() {
        println!("---\n{}", serde_yaml::to_string(crd).unwrap());
    }
}
