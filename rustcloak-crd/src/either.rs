use std::{fmt::{self, Display, Formatter}, ops::Deref};

use crate::{InstanceRef, traits::Endpoint};
use either::{Either, for_both};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct UntaggedEither<L, R>
where
    L: Serialize + DeserializeOwned + JsonSchema,
    R: Serialize + DeserializeOwned + JsonSchema,
{
    #[schemars(with = "Either<L, R>")]
    #[serde(with = "either::serde_untagged")]
    pub inner: Either<L, R>,
}

impl<L, R> AsRef<str> for UntaggedEither<L, R>
where
    L: Serialize + DeserializeOwned + JsonSchema + AsRef<str>,
    R: Serialize + DeserializeOwned + JsonSchema + AsRef<str>,
{
    fn as_ref(&self) -> &str {
        for_both!(self.inner, ref s => s.as_ref())
    }
}

impl<L, R> Display for UntaggedEither<L, R>
where
    L: Serialize + DeserializeOwned + JsonSchema + Display,
    R: Serialize + DeserializeOwned + JsonSchema + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for_both!(self.inner, ref s => write!(f, "{}", s))
    }
}

impl<L, R> Deref for UntaggedEither<L, R>
where
    L: Serialize + DeserializeOwned + JsonSchema,
    R: Serialize + DeserializeOwned + JsonSchema,
{
    type Target = Either<L, R>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<L: Endpoint, R: Endpoint> Endpoint for Either<L, R> {
    fn endpoint(&self) -> Option<&crate::KeycloakApiStatusEndpoint> {
        for_both!(self, s => s.endpoint())
    }

    fn instance_ref(&self) -> Option<&InstanceRef> {
        for_both!(self, s => s.instance_ref())
    }

    fn resource_path(&self) -> Option<&str> {
        for_both!(self, s => s.resource_path())
    }
}
