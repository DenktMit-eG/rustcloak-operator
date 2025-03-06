use k8s_openapi::api::core::v1::Secret;

use crate::error::{Error, Result};
use rustcloak_crd::traits::SecretKeyNames;

pub trait SecretUtils {
    fn extract_opt<const N: usize>(
        self,
        key_names: &impl SecretKeyNames<N>,
    ) -> [Option<String>; N];
    fn extract<const N: usize>(
        self,
        key_names: &impl SecretKeyNames<N>,
    ) -> Result<[String; N]>;
}

fn extract_inner<const N: usize>(
    secret: Secret,
    key_names: &impl SecretKeyNames<N>,
) -> [Result<String>; N] {
    let key_names = key_names.secret_key_names();
    let Some(data) = secret.data.as_ref() else {
        return key_names.map(|k| Err(Error::MissingField(k.to_string())));
    };
    key_names.map(|k| {
        let v = data
            .get(k)
            .ok_or_else(|| Error::MissingField(k.to_string()))?;
        String::from_utf8(v.0.clone()).map_err(Error::from)
    })
}

impl SecretUtils for Secret {
    fn extract<const N: usize>(
        self,
        key_names: &impl SecretKeyNames<N>,
    ) -> Result<[String; N]> {
        extract_inner(self, key_names)
            .into_iter()
            .collect::<Result<Vec<_>>>()
            .map(|x| x.try_into().unwrap())
    }

    fn extract_opt<const N: usize>(
        self,
        key_names: &impl SecretKeyNames<N>,
    ) -> [Option<String>; N] {
        extract_inner(self, key_names).map(|r| r.ok())
    }
}
