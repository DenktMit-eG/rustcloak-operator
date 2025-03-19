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
