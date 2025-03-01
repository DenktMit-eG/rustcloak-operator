use crate::traits::Endpoint;
use either::{Either, for_both};

impl<L: Endpoint, R: Endpoint> Endpoint for Either<L, R> {
    fn endpoint(&self) -> Option<&crate::KeycloakApiStatusEndpoint> {
        for_both!(self, s => s.endpoint())
    }

    fn instance_ref(&self) -> Option<&str> {
        for_both!(self, s => s.instance_ref())
    }

    fn resource_path(&self) -> Option<&str> {
        for_both!(self, s => s.resource_path())
    }
}
