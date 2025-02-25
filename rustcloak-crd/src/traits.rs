use crate::KeycloakApiStatusEndpoint;
use std::iter;

pub trait SecretKeyNames<const N: usize> {
    const DEFAULTS: [&'static str; N];
    fn secret_key_names_opts(&self) -> Option<[&Option<String>; N]>;
    fn secret_key_names(&self) -> [&str; N] {
        if let Some(key_names) = self.secret_key_names_opts() {
            let mut iter = iter::zip(key_names, Self::DEFAULTS);
            [(); N]
                .map(|_| iter.next().unwrap())
                .map(|(opt, def)| opt.as_ref().map_or(def, |s| s))
        } else {
            Self::DEFAULTS
        }
    }
}

pub trait Endpoint {
    fn endpoint(&self) -> Option<&KeycloakApiStatusEndpoint>;
    fn instance_ref(&self) -> Option<&str>;
    fn resource_path(&self) -> Option<&str>;
}

macro_rules! impl_instance_ref {
    ($type:ident, $cluster_type:ident) => {
        impl_instance_ref!($type);
        impl_instance_ref!($cluster_type);
    };
    ($type:ident) => {
        impl $crate::traits::Endpoint for $type {
            fn endpoint(&self) -> Option<&$crate::KeycloakApiStatusEndpoint> {
                self.status.as_ref().and_then(|s| s.endpoint.as_ref())
            }
            fn instance_ref(&self) -> Option<&str> {
                self.endpoint().map(|e| e.instance_ref.as_str())
            }
            fn resource_path(&self) -> Option<&str> {
                self.endpoint().map(|e| e.resource_path.as_str())
            }
        }
    };
}

pub(crate) use impl_instance_ref;
