use std::path::PathBuf;

use clap::Parser;
use crd2md::ToMarkdown;
use kube::{CustomResourceExt, ResourceExt};
use rustcloak_operator::crd::*;

static MD_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../docs/src/crds");
static CRD_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../charts/",
    env!("CARGO_PKG_NAME"),
    "/crds"
);

#[derive(Debug, Parser)]
struct Opts {
    #[clap(long)]
    update: bool,
    #[clap(long)]
    markdown: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

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
        KeycloakResource::crd(),
        KeycloakRole::crd(),
        KeycloakScope::crd(),
        KeycloakUser::crd(),
    ];

    for crd in crds.iter() {
        let yaml = serde_yaml::to_string(crd)?;
        let md = crd.to_markdown();
        if opts.markdown {
            println!("---\n{}", md);
        } else if opts.update {
            std::fs::create_dir_all(MD_DIR)?;
            std::fs::create_dir_all(CRD_DIR)?;
            let crd_path = format!("{}/{}.yaml", CRD_DIR, crd.name_unchecked());
            std::fs::write(crd_path, yaml)?;
            let md_path = PathBuf::from(format!(
                "{}/{}.md",
                MD_DIR,
                crd.spec.names.kind.to_lowercase()
            ));
            std::fs::write(&md_path, md)?;
            println!(
                "- [{}](crds/{})",
                crd.spec.names.kind,
                md_path.file_name().unwrap().to_str().unwrap()
            );
        } else {
            println!("---\n{}", yaml);
        }
    }
    Ok(())
}
