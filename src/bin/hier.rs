use std::sync::Arc;

use kube::Api;
use rustcloak_operator::{
    crd::{HasRoute, KeycloakClient, KeycloakProtocolMapper, KeycloakRealm},
    endpoint::hierarchy::{HasHierContainer, Hierarchy, Traversable},
};

pub async fn with_trait<T>(t: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: HasRoute + Sync + Send + HasHierContainer,
    Hierarchy<T>: Traversable<Object = T>,
    T::ParentType: HasHierContainer,
{
    let t = Arc::new(t);
    Hierarchy::query(t, kube::Client::try_default().await.unwrap())
        .await?
        .path();
    Ok(())
}

#[tokio::main]
async fn main() {
    println!(
        "{}",
        std::any::type_name::<<KeycloakRealm as HasRoute>::ParentType>()
    );
    println!(
        "{}",
        std::any::type_name::<<KeycloakProtocolMapper as HasRoute>::ParentType>(
        )
    );
    let client = kube::Client::try_default().await.unwrap();
    let api = Api::<KeycloakClient>::namespaced(client, "default");
    let obj = api.get("client-example-keycloakclient").await.unwrap();
    //let obj = Arc::new(obj);

    //Hierarchy::query(obj, kube::Client::try_default().await.unwrap())
    //    .await.unwrap()
    //    .path();
    with_trait(obj).await.unwrap();
}
