pub trait WithStatus<T> {
    fn status(&self) -> Option<&T>;
}
