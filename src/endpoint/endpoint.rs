use crate::morph::ToApiObject;
use schemars::JsonSchema;

struct Endpoint<T>(T);

trait SimpleSubPath {
    fn sub_path() -> String;
}

impl<T> Endpoint<T: JsonSchema + ToApiObject> {
    fn new(t: T) -> Self {
        Endpoint(t)
    }
}

impl<T> Endpoint<T: JsonSchema + ToApiObject + SimpleSubPath> {
    fn path(&self) -> String {
        format!("{}/", T::sub_path(), self.)
    }
}
