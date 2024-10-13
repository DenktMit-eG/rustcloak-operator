use std::collections::HashMap;

use super::{KeycloakAdminApiOptions, KeycloakApiStatus};
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakRealm",
    group = "rustcloak.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
pub struct KeycloakRealmSpec {
    pub api: KeycloakAdminApiOptions,
    pub definition: KeycloakRealmDefinition,
    pub extra_yaml: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct KeycloakRealmDefinition {
    id: Option<String>,
    realm: String,
    enabled: bool,
    display_name: Option<String>,
    display_name_html: Option<String>,
    password_policy: Option<String>,
    registration_allowed: Option<bool>,
    registration_email_as_username: Option<bool>,
    edit_username_allowed: Option<bool>,
    reset_password_allowed: Option<bool>,
    remember_me: Option<bool>,
    verify_email: Option<bool>,
    login_with_email_allowed: Option<bool>,
    duplicate_emails_allowed: Option<bool>,
    ssl_required: Option<String>,

    brute_force_protection: Option<bool>,
    permanent_lockout: Option<bool>,
    failure_factor: Option<i32>,
    wait_increment_seconds: Option<i32>,
    quick_login_check_milli_seconds: Option<i32>,
    minimum_quick_login_wait_seconds: Option<i32>,
    max_failure_wait_seconds: Option<i32>,
    max_delta_time_seconds: Option<i32>,

    smtp_server: Option<HashMap<String, String>>,

    login_theme: Option<String>,
    account_theme: Option<String>,
    admin_theme: Option<String>,
    email_theme: Option<String>,
    internationalization_enabled: Option<bool>,
    supported_locales: Option<Vec<String>>,
    default_locale: Option<String>,
}
