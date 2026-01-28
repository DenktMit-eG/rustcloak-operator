use rustcloak_crd::KeycloakApiStatus;

pub trait StatusTrait {
    fn message(&self) -> &str;
}

impl StatusTrait for KeycloakApiStatus {
    fn message(&self) -> &str {
        &self.message
    }
}

macro_rules! impl_dumb_status_trait {
    ($type:ty) => {
        impl StatusTrait for $type {
            fn message(&self) -> &str {
                &self.message
            }
        }
    };
}
impl_dumb_status_trait!(keycloak_crd::ExternalKeycloakStatus);
impl_dumb_status_trait!(keycloak_crd::KeycloakRealmStatus);
impl_dumb_status_trait!(keycloak_crd::KeycloakClientStatus);
impl_dumb_status_trait!(keycloak_crd::KeycloakUserStatus);

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use rustcloak_crd::KeycloakApiStatus;

    #[test]
    fn test_message__keycloak_api_status__returns_correct_message() {
        let status = KeycloakApiStatus {
            message: "All good".to_string(),
            ..Default::default()
        };

        let result = status.message();

        assert_eq!(result, "All good");
    }

    #[test]
    fn test_message__keycloak_api_status_empty__returns_empty_string() {
        let status = KeycloakApiStatus {
            message: "".to_string(),
            ..Default::default()
        };

        let result = status.message();

        assert_eq!(result, "");
    }

    #[test]
    fn test_message__external_keycloak_status__returns_correct_message() {
        let status = keycloak_crd::ExternalKeycloakStatus {
            message: "External error".to_string(),
            ..Default::default()
        };

        let result = status.message();

        assert_eq!(result, "External error");
    }

    #[test]
    fn test_message__keycloak_realm_status__returns_correct_message() {
        let status = keycloak_crd::KeycloakRealmStatus {
            message: "Realm ready".to_string(),
            ..Default::default()
        };

        let result = status.message();

        assert_eq!(result, "Realm ready");
    }
}
