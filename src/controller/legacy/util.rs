use crate::error::Error;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use k8s_openapi::NamespaceResourceScope;
use kube::api::ListParams;
use kube::Api;
use kube::Resource;
use kube::ResourceExt;
use serde::de::DeserializeOwned;

use crate::error::Result;

pub async fn find_name<T>(
    client: &kube::Client,
    namespace: &str,
    selector: &LabelSelector,
) -> Result<String>
where
    T: Resource<Scope = NamespaceResourceScope, DynamicType = ()>
        + Clone
        + std::fmt::Debug
        + DeserializeOwned,
{
    let api = Api::<T>::namespaced(client.clone(), namespace);
    let selector = selector.clone().try_into()?;

    let list = api
        .list_metadata(
            &ListParams::default().match_any().labels_from(&selector),
        )
        .await?;
    if list.items.is_empty() {
        Err(Error::LegacyInstanceNotFound)
    } else if list.items.len() > 1 {
        Err(Error::AmbiguousLegacyInstancesFound)
    } else {
        Ok(list.items[0].name_unchecked())
    }
}
