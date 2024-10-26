use kube::{CustomResourceExt, ResourceExt};
use rustcloak_operator::crd::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crds = [
        //KeycloakInstance::crd(),
        //KeycloakApiObject::crd(),
        //KeycloakAuthenticationFlow::crd(),
        //KeycloakAuthenticatorConfig::crd(),
        //KeycloakClient::crd(),
        //KeycloakClientScope::crd(),
        //KeycloakComponent::crd(),
        //KeycloakGroup::crd(),
        //KeycloakIdentityProvider::crd(),
        //KeycloakIdentityProviderMapper::crd(),
        KeycloakOrganization::crd(),
        //KeycloakProtocolMapper::crd(),
        //KeycloakRealm::crd(),
        //KeycloakRequiredActionProvider::crd(),
        ////KeycloakResource::crd(),
        //KeycloakRole::crd(),
        //KeycloakScope::crd(),
        //KeycloakUser::crd(),
    ];

    let dir = std::env::args().nth(1);
    for crd in crds.iter() {
        let str = serde_yaml::to_string(crd)?;
        if let Some(ref dir) = dir {
            let path = format!("{}/{}.yaml", dir, crd.name_unchecked());
            std::fs::write(path, str)?;
        } else {
            println!("---\n{}", str);
        }
    }
    Ok(())
}
