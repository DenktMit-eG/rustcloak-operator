use kube::core::object::HasSpec;

pub trait HasInnerSpec {
    type InnerSpec;
    fn inner_spec(&self) -> &Self::InnerSpec;
}

impl<R> HasInnerSpec for R
where
    R: HasSpec<Spec: HasInnerSpec>,
{
    type InnerSpec = <R::Spec as HasInnerSpec>::InnerSpec;
    fn inner_spec(&self) -> &Self::InnerSpec {
        &self.spec().inner_spec()
    }
}
