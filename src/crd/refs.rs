use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientRef {
    pub client_ref: String,
}
impl AsRef<str> for ClientRef {
    fn as_ref(&self) -> &str {
        &self.client_ref
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeRef {
    pub client_scope_ref: String,
}
impl AsRef<str> for ClientScopeRef {
    fn as_ref(&self) -> &str {
        &self.client_scope_ref
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmRef {
    pub realm_ref: String,
}
impl AsRef<str> for RealmRef {
    fn as_ref(&self) -> &str {
        &self.realm_ref
    }
}
