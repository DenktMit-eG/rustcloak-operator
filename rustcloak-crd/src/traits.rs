use crate::{instance::InstanceRef, realm::RealmRef};
use std::iter;

pub trait SecretKeyNames<const N: usize> {
    const DEFAULTS: [&'static str; N];
    fn secret_key_names_opts(&self) -> [&Option<String>; N];
    fn secret_key_names(&self) -> [&str; N] {
        let mut iter = iter::zip(self.secret_key_names_opts(), Self::DEFAULTS);
        [(); N]
            .map(|_| iter.next().unwrap())
            .map(|(opt, def)| opt.as_ref().map_or(def, |s| s))
    }
}

impl<T, const N: usize> SecretKeyNames<N> for Option<T>
where
    T: SecretKeyNames<N>,
{
    const DEFAULTS: [&'static str; N] = T::DEFAULTS;
    fn secret_key_names_opts(&self) -> [&Option<String>; N] {
        if let Some(s) = self.as_ref() {
            s.secret_key_names_opts()
        } else {
            [&None; N]
        }
    }
}

pub trait Endpoint {
    fn is_ready(&self) -> bool;
    fn instance_ref(&self) -> Option<&InstanceRef>;
    fn resource_path(&self) -> Option<&str>;
    fn realm_ref(&self) -> Option<RealmRef>;
}

macro_rules! impl_endpoint {
    ($type:ident, $cluster_type:ident) => {
        impl_instance_ref!($type);
        impl_instance_ref!($cluster_type);
    };
    ($type:ident) => {
        impl $type {
            fn endpoint(&self) -> Option<&$crate::KeycloakApiStatusEndpoint> {
                self.status.as_ref().and_then(|s| s.endpoint.as_ref())
            }
        }
        impl $crate::traits::Endpoint for $type {
            fn is_ready(&self) -> bool {
                self.status.as_ref().map_or(false, |s| s.ready)
            }
            fn instance_ref(&self) -> Option<&$crate::instance::InstanceRef> {
                self.endpoint().map(|e| &e.instance)
            }
            fn realm_ref(&self) -> Option<$crate::realm::RealmRef> {
                self.endpoint().and_then(|e| e.realm.clone())
            }
            fn resource_path(&self) -> Option<&str> {
                self.endpoint().map(|e| e.resource_path.as_str())
            }
        }
        impl $crate::marker::HasMarker for $type {
            type Marker = $crate::marker::ResourceMarker<
                <$type as ::kube::Resource>::Scope,
            >;
        }
    };
}

pub(crate) use impl_endpoint;
