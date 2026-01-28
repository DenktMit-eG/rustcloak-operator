use crate::error::Error;

pub trait FromError {
    fn from_error(err: &Error) -> Self;
}

impl FromError for keycloak_crd::ExternalKeycloakStatus {
    fn from_error(err: &Error) -> Self {
        // TODO
        Self {
            message: err.to_string(),
            ..Default::default()
        }
    }
}
impl FromError for keycloak_crd::KeycloakRealmStatus {
    fn from_error(err: &Error) -> Self {
        // TODO
        Self {
            message: err.to_string(),
            ..Default::default()
        }
    }
}
impl FromError for keycloak_crd::KeycloakClientStatus {
    fn from_error(err: &Error) -> Self {
        // TODO
        Self {
            message: err.to_string(),
            ..Default::default()
        }
    }
}
impl FromError for keycloak_crd::KeycloakUserStatus {
    fn from_error(err: &Error) -> Self {
        // TODO
        Self {
            message: err.to_string(),
            ..Default::default()
        }
    }
}

impl FromError for rustcloak_crd::KeycloakApiStatus {
    fn from_error(err: &Error) -> Self {
        Self {
            ready: false,
            status: "Error".to_string(),
            message: err.to_string(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use crate::error::Error;
    use rustcloak_crd::KeycloakApiStatus;

    #[test]
    fn test_from_error__io_error__returns_status_with_error_message() {
        let io_err = std::io::Error::other("io failure");
        let err = Error::from(io_err);

        let status = KeycloakApiStatus::from_error(&err);

        assert!(!status.ready);
        assert_eq!(status.status, "Error");
        assert!(status.message.contains("io failure"));
    }

    #[test]
    fn test_from_error__no_token_error__returns_status_with_no_token_message() {
        let err = Error::NoToken;

        let status = KeycloakApiStatus::from_error(&err);

        assert!(!status.ready);
        assert_eq!(status.status, "Error");
        assert_eq!(status.message, "No token");
    }

    #[test]
    fn test_from_error__external_keycloak_status__returns_status_with_message()
    {
        let err = Error::NoToken;

        let status = keycloak_crd::ExternalKeycloakStatus::from_error(&err);

        assert_eq!(status.message, "No token");
    }
}
