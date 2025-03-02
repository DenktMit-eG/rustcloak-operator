use either::Either;
use k8s_openapi::ResourceScope;

pub trait HasMarker {
    type Marker;
}

impl<L, R> HasMarker for Either<L, R> {
    type Marker = EitherMarker;
}

pub struct EitherMarker;
pub struct ResourceMarker<R: ResourceScope> (R);
