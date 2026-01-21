/// Clients created on realm creation
pub const DEFAULT_CLIENTS: &[&str] = &[
    "account",
    "account-console",
    "admin-cli",
    "broker",
    "realm-management",
    "security-admin-console",
];

/// Default client scopes
pub const DEFAULT_CLIENT_SCOPES: &[&str] = &[
    "acr",
    "address",
    "basic",
    "email",
    "microprofile-jwt",
    "offline_access",
    "phone",
    "profile",
    "role_list",
    "roles",
    "saml_organization",
    "web-origins",
];

/// Default authentication flows
pub const DEFAULT_AUTH_FLOWS: &[&str] = &[
    "browser",
    "direct grant",
    "registration",
    "reset credentials",
    "clients",
    "first broker login",
    "docker auth",
    "http challenge",
];

/// Default realm roles
pub const DEFAULT_REALM_ROLES: &[&str] = &["offline_access", "uma_authorization"];

/// Excluded realms
pub const EXCLUDED_REALMS: &[&str] = &["master"];

/// Check if a client ID is a default client
pub fn is_default_client(client_id: &str) -> bool {
    DEFAULT_CLIENTS.contains(&client_id)
}

/// Check if a client scope name is a default scope
pub fn is_default_client_scope(name: &str) -> bool {
    DEFAULT_CLIENT_SCOPES.contains(&name)
}

/// Check if an authentication flow alias is a default flow
pub fn is_default_auth_flow(alias: &str) -> bool {
    DEFAULT_AUTH_FLOWS.contains(&alias)
}

/// Check if a role name is a default realm role
pub fn is_default_realm_role(name: &str) -> bool {
    DEFAULT_REALM_ROLES.contains(&name)
}

/// Check if a realm should be excluded from import
pub fn is_excluded_realm(realm: &str) -> bool {
    EXCLUDED_REALMS.contains(&realm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_clients() {
        assert!(is_default_client("account"));
        assert!(is_default_client("admin-cli"));
        assert!(!is_default_client("my-custom-client"));
    }

    #[test]
    fn test_default_client_scopes() {
        assert!(is_default_client_scope("email"));
        assert!(is_default_client_scope("profile"));
        assert!(!is_default_client_scope("my-custom-scope"));
    }

    #[test]
    fn test_excluded_realms() {
        assert!(is_excluded_realm("master"));
        assert!(!is_excluded_realm("my-realm"));
    }
}
