use rustcloak_crd::Namespace;

trait HasNamespace {
    fn as_ns(&self) -> Option<&str>;
}

impl HasNamespace for () {
    fn as_ns(&self) -> Option<&str> {
        None
    }
}

impl HasNamespace for Namespace {
    fn as_ns(&self) -> Option<&str> {
        Some(&self.namespace)
    }
}
