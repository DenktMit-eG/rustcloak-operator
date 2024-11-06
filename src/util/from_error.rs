use crate::error::Error;

pub trait FromError {
    fn from_error(err: &Error) -> Self;
}

impl FromError for keycloak_crd::ExternalKeycloakStatus {
    fn from_error(_err: &Error) -> Self {
        // TODO
        Self {
            conditions: vec![],
            instances: 0,
            observed_generation: 0,
            selector: "".to_string(),
        }
    }
}
impl FromError for keycloak_crd::KeycloakRealmStatus {
    fn from_error(err: &Error) -> Self {
        // TODO
        Self {
            phase: "".to_string(),
            message: err.to_string(),
            ready: false,
        }
    }
}
impl FromError for keycloak_crd::KeycloakClientStatus {
    fn from_error(err: &Error) -> Self {
        // TODO
        Self {
            phase: "".to_string(),
            message: err.to_string(),
            ready: false,
        }
    }
}
impl FromError for keycloak_crd::KeycloakUserStatus {
    fn from_error(err: &Error) -> Self {
        // TODO
        Self {
            phase: "".to_string(),
            message: err.to_string(),
        }
    }
}
