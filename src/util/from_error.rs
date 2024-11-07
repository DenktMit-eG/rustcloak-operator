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
