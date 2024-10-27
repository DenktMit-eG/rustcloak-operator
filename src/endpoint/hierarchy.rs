use crate::{
    crd::{HasRoute, KeycloakInstance},
    error::{Error, Result},
};
use async_trait::async_trait;
use kube::{Resource, ResourceExt};
use std::sync::Arc;

use super::retriever::{Retrieve, Retriever};

pub type Root = String;

pub trait HasHierContainer {
    type HierContainer;
}

#[async_trait]
pub trait Traversable
where
    Self: Sized,
{
    type Object;
    async fn inner_query(
        object: Arc<Self::Object>,
        client: kube::Client,
    ) -> Result<Self>;

    fn inner_path(&self) -> String;

    fn inner_instance_ref(&self) -> &str;
}

impl HasHierContainer for Root {
    type HierContainer = Root;
}

impl<T, P> HasHierContainer for T
where
    T: HasRoute<ParentType = P>,
    P: HasHierContainer,
{
    type HierContainer = Hierarchy<T>;
}

pub struct Hierarchy<O>
where
    O: HasRoute,
    O::ParentType: HasHierContainer,
{
    pub object: Arc<O>,
    pub parent: <O::ParentType as HasHierContainer>::HierContainer,
}

impl<O> Hierarchy<O>
where
    Self: Traversable<Object = O>,
    O: HasRoute,
    O::ParentType: HasHierContainer,
{
    pub async fn query(
        object: Arc<<Self as Traversable>::Object>,
        client: kube::Client,
    ) -> Result<Self> {
        Self::inner_query(object, client).await
    }
    pub fn path(&self) -> String {
        self.inner_path()
    }
    pub fn instance_ref(&self) -> &str {
        self.inner_instance_ref()
    }
    pub async fn instance(
        &self,
        client: kube::Client,
    ) -> Result<KeycloakInstance> {
        let ns = self.object.namespace().ok_or(Error::NoNamespace)?;
        let api = kube::Api::<KeycloakInstance>::namespaced(client, &ns);

        Ok(api.get(self.instance_ref()).await?)
    }
}

#[async_trait]
impl<O> Traversable for Hierarchy<O>
where
    O: HasRoute + Resource + HasRoute + Send + Sync + 'static,
    O::ParentType: HasHierContainer,
    <O::ParentType as HasHierContainer>::HierContainer:
        Traversable<Object = O::ParentType>,
    Retriever<O::ParentType, O>: Retrieve<Object = O::ParentType, Child = O>,
{
    type Object = O;
    async fn inner_query(
        object: Arc<Self::Object>,
        client: kube::Client,
    ) -> Result<Self> {
        let ns = object.namespace().ok_or(Error::NoNamespace)?;
        let parent =
            Retriever::retrieve(client.clone(), &ns, object.as_ref()).await?;
        let parent =
            <O::ParentType as HasHierContainer>::HierContainer::inner_query(
                Arc::new(parent),
                client,
            )
            .await?;
        Ok(Hierarchy { object, parent })
    }

    fn inner_path(&self) -> String {
        format!(
            "{}/{}/{}",
            self.parent.inner_path(),
            self.object.mount_point(),
            self.object.id()
        )
    }

    fn inner_instance_ref(&self) -> &str {
        self.parent.inner_instance_ref()
    }
}

#[async_trait]
impl Traversable for Root {
    type Object = Root;
    async fn inner_query(
        object: Arc<Self::Object>,
        _client: kube::Client,
    ) -> Result<Self> {
        Ok(Arc::unwrap_or_clone(object))
    }

    fn inner_path(&self) -> String {
        "".to_string()
    }

    fn inner_instance_ref(&self) -> &str {
        self.as_str()
    }
}
