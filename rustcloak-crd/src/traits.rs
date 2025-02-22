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

pub trait InstanceRef {
    fn instance_ref(&self) -> Option<&str>;
    fn resource_path(&self) -> Option<&str>;
}

macro_rules! impl_instance_ref {
    ($type:ident) => {
        impl $crate::traits::InstanceRef for $type {
            fn instance_ref(&self) -> Option<&str> {
                self.status
                    .as_ref()
                    .and_then(|status| status.instance_ref.as_deref())
            }
            fn resource_path(&self) -> Option<&str> {
                self.status
                    .as_ref()
                    .and_then(|status| status.resource_path.as_deref())
            }
        }
    };
}

pub(crate) use impl_instance_ref;
