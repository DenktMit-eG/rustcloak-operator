use std::collections::HashMap;

use super::{KeycloakApiObjectOptions, KeycloakApiStatus, WithStatus};
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakRealm",
    shortname = "kcrm",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
pub struct KeycloakRealmSpec {
    pub api: KeycloakApiObjectOptions,
    pub definition: KeycloakRealmDefinition,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct KeycloakRealmDefinition {
    pub id: Option<String>,
    pub realm: String,
    pub enabled: bool,
    pub display_name: Option<String>,
    pub display_name_html: Option<String>,
    pub password_policy: Option<String>,
    pub registration_allowed: Option<bool>,
    pub registration_email_as_username: Option<bool>,
    pub edit_username_allowed: Option<bool>,
    pub reset_password_allowed: Option<bool>,
    pub remember_me: Option<bool>,
    pub verify_email: Option<bool>,
    pub login_with_email_allowed: Option<bool>,
    pub duplicate_emails_allowed: Option<bool>,
    pub ssl_required: Option<String>,

    pub brute_force_protection: Option<bool>,
    pub permanent_lockout: Option<bool>,
    pub failure_factor: Option<i32>,
    pub wait_increment_seconds: Option<i32>,
    pub quick_login_check_milli_seconds: Option<i32>,
    pub minimum_quick_login_wait_seconds: Option<i32>,
    pub max_failure_wait_seconds: Option<i32>,
    pub max_delta_time_seconds: Option<i32>,

    pub smtp_server: Option<HashMap<String, String>>,

    pub login_theme: Option<String>,
    pub account_theme: Option<String>,
    pub admin_theme: Option<String>,
    pub email_theme: Option<String>,
    pub internationalization_enabled: Option<bool>,
    pub supported_locales: Option<Vec<String>>,
    pub default_locale: Option<String>,
}

impl WithStatus<KeycloakApiStatus> for KeycloakRealm {
    fn status(&self) -> Option<&KeycloakApiStatus> {
        self.status.as_ref()
    }
}
