use std::collections::BTreeMap;

use crate::app_id;
use crate::error::Error;
use crate::util::ApiExt;
use crate::util::ApiFactory;
use heck::ToLowerCamelCase;
use either::Either;
use k8s_openapi::NamespaceResourceScope;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use kube::Resource;
use kube::ResourceExt;
use kube::api::ListParams;
use kube::api::ObjectMeta;
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
    let camel_case = snake_case.to_lower_camel_case();
    let variants =
        [snake_case, &camel_case].map(|x| format!(app_id!("{}"), x));

    annotations
        .iter()
        .find_map(|(k, v)| variants.contains(k).then(|| v.clone()))
}

pub async fn find_name<T>(
    client: &kube::Client,
    namespace: &Option<String>,
    selector: &LabelSelector,
    meta: &ObjectMeta,
    parent_ref_ident: &str,
) -> Result<Either<String, String>>
where
    T: Resource<Scope = NamespaceResourceScope, DynamicType = ()>
        + Clone
        + std::fmt::Debug
        + DeserializeOwned,
    ApiExt<T>: ApiFactory<Resource = T>,
{
    if let Some(name) = meta
        .annotations
        .as_ref()
        .and_then(|x| find_variants(x, parent_ref_ident))
    {
        return Ok(Either::Left(name.clone()));
    }
    let parent_cluster_ref_ident = format!("cluster_{parent_ref_ident}");
    if let Some(name) = meta
        .annotations
        .as_ref()
        .and_then(|x| find_variants(x, &parent_cluster_ref_ident))
    {
        return Ok(Either::Right(name.clone()));
    }

    let api = ApiExt::<T>::api(client.clone(), namespace);
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
        Ok(Either::Left(list.items[0].name_unchecked()))
    }
}
