use rustcloak_crd::KeycloakApiStatus;

pub trait StatusTrait {
    fn message(&self) -> &str;
    fn reconcile_attempts(&self) -> Option<u64>;
    fn set_reconcile_attempts(&mut self, attempts: Option<u64>);
    fn inc_reconcile_attempts(&mut self) -> u64 {
        let attempts = self.reconcile_attempts().unwrap_or(0) + 1;
        self.set_reconcile_attempts(Some(attempts));
        attempts
    }
}

impl StatusTrait for KeycloakApiStatus {
    fn message(&self) -> &str {
        &self.message
    }

    fn reconcile_attempts(&self) -> Option<u64> {
        self.reconcile_attempts
    }

    fn set_reconcile_attempts(&mut self, attempts: Option<u64>) {
        self.reconcile_attempts = attempts;
    }
}

macro_rules! impl_dumb_status_trait {
    ($type:ty) => {
        impl StatusTrait for $type {
            fn message(&self) -> &str {
                &self.message
            }
            fn reconcile_attempts(&self) -> Option<u64> {
                None
            }
            fn set_reconcile_attempts(&mut self, _attempts: Option<u64>) {}
        }
    };
}
impl_dumb_status_trait!(keycloak_crd::ExternalKeycloakStatus);
impl_dumb_status_trait!(keycloak_crd::KeycloakRealmStatus);
impl_dumb_status_trait!(keycloak_crd::KeycloakClientStatus);
impl_dumb_status_trait!(keycloak_crd::KeycloakUserStatus);
