use kube::CustomResourceExt;
use rustcloak_operator::crd::*;

fn main() {
    let crds = [
        KeycloakAdminApi::crd(),
        KeycloakInstance::crd(),
        KeycloakRealm::crd(),
    ];
    for crd in crds.iter() {
        println!("---\n{}", serde_yaml::to_string(crd).unwrap());
    }
}
