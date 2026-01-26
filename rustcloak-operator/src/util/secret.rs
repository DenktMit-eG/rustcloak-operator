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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use k8s_openapi::ByteString;
    use std::collections::BTreeMap;

    struct TestKeyNames<const N: usize>([&'static str; N]);
    impl<const N: usize> SecretKeyNames<N> for TestKeyNames<N> {
        const DEFAULTS: [&'static str; N] = [""; N];
        fn secret_key_names_opts(&self) -> [&Option<String>; N] {
            [const { &None }; N]
        }
        fn secret_key_names(&self) -> [&str; N] {
            self.0
        }
    }

    #[test]
    fn test_extract__valid_secret__returns_ok_with_values() {
        let mut data = BTreeMap::new();
        data.insert("key1".to_string(), ByteString(b"val1".to_vec()));
        data.insert("key2".to_string(), ByteString(b"val2".to_vec()));
        let secret = Secret {
            data: Some(data),
            ..Default::default()
        };
        let keys = TestKeyNames(["key1", "key2"]);

        let result = secret.extract(&keys);

        assert!(result.is_ok());
        let values = result.unwrap();
        assert_eq!(values, ["val1".to_string(), "val2".to_string()]);
    }

    #[test]
    fn test_extract__missing_key__returns_err_missing_field() {
        let mut data = BTreeMap::new();
        data.insert("key1".to_string(), ByteString(b"val1".to_vec()));
        let secret = Secret {
            data: Some(data),
            ..Default::default()
        };
        let keys = TestKeyNames(["key1", "key2"]);

        let result = secret.extract(&keys);

        match result {
            Err(Error::MissingField(field)) => assert_eq!(field, "key2"),
            _ => panic!("Expected Error::MissingField(key2), got {:?}", result),
        }
    }

    #[test]
    fn test_extract_opt__partial_secret__returns_some_and_none() {
        let mut data = BTreeMap::new();
        data.insert("key1".to_string(), ByteString(b"val1".to_vec()));
        let secret = Secret {
            data: Some(data),
            ..Default::default()
        };
        let keys = TestKeyNames(["key1", "key2"]);

        let result = secret.extract_opt(&keys);

        assert_eq!(result, [Some("val1".to_string()), None]);
    }
}
