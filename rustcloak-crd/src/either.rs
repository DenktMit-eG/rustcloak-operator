use crate::traits::InstanceRef;
use either::{for_both, Either};

impl<L: InstanceRef, R: InstanceRef> InstanceRef for Either<L, R> {
    fn instance_ref(&self) -> Option<&str> {
        for_both!(self, s => s.instance_ref())
    }

    fn resource_path(&self) -> Option<&str> {
        for_both!(self, s => s.resource_path())
    }
}
