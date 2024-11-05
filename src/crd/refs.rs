use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientRef {
    pub client_ref: String,
}
impl From<ClientRef> for String {
    fn from(val: ClientRef) -> Self {
        val.client_ref
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeRef {
    pub client_scope_ref: String,
}
impl From<ClientScopeRef> for String {
    fn from(val: ClientScopeRef) -> Self {
        val.client_scope_ref
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmRef {
    pub realm_ref: String,
}
impl From<RealmRef> for String {
    fn from(val: RealmRef) -> Self {
        val.realm_ref
    }
}
