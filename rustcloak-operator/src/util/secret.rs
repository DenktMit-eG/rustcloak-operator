use k8s_openapi::api::core::v1::Secret;

use crate::error::{Error, Result};
use rustcloak_crd::traits::SecretKeyNames;

pub trait SecretUtils {
    fn extract<const N: usize>(
        self,
        key_names: &impl SecretKeyNames<N>,
    ) -> Result<[String; N]>;

    fn extract_opt<const N: usize>(
        self,
        key_names: &impl SecretKeyNames<N>,
    ) -> Result<[Option<String>; N]>;
}

impl SecretUtils for Secret {
    fn extract<const N: usize>(
        self,
        key_names: &impl SecretKeyNames<N>,
    ) -> Result<[String; N]> {
        Ok(self
            .extract_opt(key_names)?
            .into_iter()
            .map(|v| v.ok_or(Error::NoData))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .unwrap())
    }

    fn extract_opt<const N: usize>(
        self,
        key_names: &impl SecretKeyNames<N>,
    ) -> Result<[Option<String>; N]> {
        let key_names = key_names.secret_key_names();
        let Some(data) = self.data.as_ref() else {
            return Ok([const { None }; N]);
        };
        Ok(key_names
            .iter()
            .map(|k| data.get(*k))
            .map(|v| v.map(|v| String::from_utf8(v.0.clone())))
            .map(|v| v.transpose().map_err(Error::from))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .unwrap())
    }
}
