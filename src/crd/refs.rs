use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientRef {
    pub client_ref: String,
}
impl Into<String> for ClientRef {
    fn into(self) -> String {
        self.client_ref
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeRef {
    pub client_scope_ref: String,
}
impl Into<String> for ClientScopeRef {
    fn into(self) -> String {
        self.client_scope_ref
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmRef {
    pub realm_ref: String,
}
impl Into<String> for RealmRef {
    fn into(self) -> String {
        self.realm_ref
    }
}
