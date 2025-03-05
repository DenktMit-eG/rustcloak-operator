pub mod controller;
pub mod error;
pub mod morph;
pub mod opts;
pub mod util;

macro_rules! app_id {
    () => {
        "rustcloak.k8s.eboland.de"
    };
    ($name:tt) => {
        concat!(app_id!(), "/", $name)
    };
}

pub(crate) use app_id;
pub use rustcloak_crd as crd;
