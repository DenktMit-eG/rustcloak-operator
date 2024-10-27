use std::sync::Arc;

use keycloak::types::RealmRepresentation;
use kube::api::ObjectMeta;
use rustcloak_operator::{
    crd::{KeycloakRealm, KeycloakRealmSpec},
    endpoint::hierarchy::Hierarchy,
};

#[tokio::main]
async fn main() {
    let kc = KeycloakRealm {
        metadata: ObjectMeta {
            name: Some("myrealm".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        },
        spec: KeycloakRealmSpec {
            options: None,
            instance_ref: "asd".to_string().into(),
            definition: RealmRepresentation {
                ..Default::default()
            },
        },
        status: None,
    };

    Hierarchy::query(Arc::new(kc), kube::Client::try_default().await.unwrap())
        .await
        .unwrap()
        .path();
}
