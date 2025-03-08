pub mod controller;
pub mod error;
pub mod morph;
pub mod opts;
pub mod util;

macro_rules! app_id {
    ($($name:expr)?) => {
        concat!("rustcloak.k8s.eboland.de", $("/", $name)?)
    };
}

pub(crate) use app_id;
