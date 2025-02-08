use std::collections::BTreeMap;

use crate::app_id;
use crate::error::Error;
use case_style::CaseStyle;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use k8s_openapi::NamespaceResourceScope;
use kube::api::ListParams;
use kube::api::ObjectMeta;
use kube::Api;
use kube::Resource;
use kube::ResourceExt;
use serde::de::DeserializeOwned;

use crate::error::Result;

pub fn should_handle_prudent(meta: &ObjectMeta, prudent: bool) -> bool {
    if !prudent {
        true
    } else if let Some(annotations) = &meta.annotations {
        annotations.contains_key(app_id!("handle"))
    } else {
        false
    }
}

fn find_variants(
    annotations: &BTreeMap<String, String>,
    snake_case: &str,
) -> Option<String> {
    let style = CaseStyle::from_snakecase(snake_case);
    let variants = [snake_case, &style.to_camelcase()]
        .map(|x| app_id!("").to_string() + x);

    annotations
        .iter()
        .find_map(|(k, v)| variants.contains(k).then(|| v.clone()))
}

pub async fn find_name<T>(
    client: &kube::Client,
    namespace: &str,
    selector: &LabelSelector,
    meta: &ObjectMeta,
    parent_ref_ident: &str,
) -> Result<String>
where
    T: Resource<Scope = NamespaceResourceScope, DynamicType = ()>
        + Clone
        + std::fmt::Debug
        + DeserializeOwned,
{
    if let Some(name) = meta
        .annotations
        .as_ref()
        .and_then(|x| find_variants(x, parent_ref_ident))
    {
        return Ok(name.clone());
    }
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
