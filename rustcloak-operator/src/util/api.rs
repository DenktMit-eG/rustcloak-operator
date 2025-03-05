use k8s_openapi::{ClusterResourceScope, NamespaceResourceScope};
use kube::{Api, Resource};
use serde::de::DeserializeOwned;

shorter_bounds::alias!(
    pub trait ApiResource: Resource<DynamicType = ()> + DeserializeOwned
);

pub trait ApiFactory {
    type Resource: ApiResource;
    fn api(client: kube::Client, ns: &Option<String>) -> Api<Self::Resource>;
}

impl<R> ApiFactory for (R, NamespaceResourceScope)
where
    R: Resource<Scope = NamespaceResourceScope> + ApiResource,
{
    type Resource = R;

    fn api(client: kube::Client, ns: &Option<String>) -> Api<Self::Resource> {
        let ns = if let Some(ns) = ns {
            ns.to_string()
        } else {
            client.default_namespace().to_string()
        };
        Api::<Self::Resource>::namespaced(client, &ns)
    }
}

impl<R> ApiFactory for (R, ClusterResourceScope)
where
    R: Resource<Scope = ClusterResourceScope> + ApiResource,
{
    type Resource = R;

    fn api(client: kube::Client, _ns: &Option<String>) -> Api<Self::Resource> {
        Api::<Self::Resource>::all(client)
    }
}

pub type ApiExt<R> = (R, <R as Resource>::Scope);
