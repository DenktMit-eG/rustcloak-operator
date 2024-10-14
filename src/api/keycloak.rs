use crate::error::Result;
use derive_builder::Builder;
use oauth2::basic::BasicClient;
use oauth2::TokenResponse;
use oauth2::{
    basic::BasicTokenType, AuthUrl, ClientId, EmptyExtraTokenFields,
    ResourceOwnerPassword, ResourceOwnerUsername, StandardTokenResponse,
    TokenUrl,
};

// Because of the stupidly overdesigned oauth2 library, we use this generator macro
// to not have to write idiotic amounts of Generic types.
macro_rules! basic_client {
    ($keycloak_login:expr) => {
        BasicClient::new($keycloak_login.client_id.clone())
            .set_auth_uri($keycloak_login.auth_url.clone())
            .set_token_uri($keycloak_login.token_url.clone())
    };
}

// Tell me that the authors never used their own library without telling me that the authors never
// used their own library.
pub type OAuth2Token =
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

#[derive(Debug, Builder)]
pub struct KeycloakAuth {
    #[builder(setter(into))]
    url: String,
    #[builder(setter(into), default = "self.default_realm()")]
    realm: String,
    #[builder(setter(into), default = "self.default_auth_url()")]
    auth_url: AuthUrl,
    #[builder(setter(into), default = "self.default_token_url()")]
    token_url: TokenUrl,
    #[builder(setter(into), default = "self.default_client_id()")]
    client_id: ClientId,
    #[builder(setter(into), default)]
    http_client: reqwest::Client,
}

impl KeycloakAuthBuilder {
    fn default_realm(&self) -> String {
        "master".to_string()
    }

    fn default_auth_url(&self) -> AuthUrl {
        let url = self.url.as_ref().unwrap();
        let realm = self.realm.clone().unwrap_or_else(|| self.default_realm());
        AuthUrl::new(format!(
            "{url}/auth/realms/{realm}/protocol/openid-connect/auth"
        ))
        .unwrap()
    }

    fn default_token_url(&self) -> TokenUrl {
        let url = self.url.as_ref().unwrap();
        let realm = self.realm.clone().unwrap_or_else(|| self.default_realm());
        TokenUrl::new(format!(
            "{url}/auth/realms/{realm}/protocol/openid-connect/token"
        ))
        .unwrap()
    }

    fn default_client_id(&self) -> ClientId {
        ClientId::new("admin-cli".to_string())
    }
}

impl KeycloakAuth {
    pub fn new(url: &str) -> Self {
        KeycloakAuthBuilder::default()
            .url(url.to_string())
            .build()
            .unwrap()
    }

    pub async fn login_with_credentials(
        self,
        user: &str,
        password: &str,
    ) -> Result<KeycloakClient> {
        let user = ResourceOwnerUsername::new(user.to_string());
        let password = ResourceOwnerPassword::new(password.to_string());
        let oauth_client = basic_client!(&self);
        let token = oauth_client
            .exchange_password(&user, &password)
            .request_async(&self.http_client)
            .await?;

        Ok(KeycloakClient::new(self, token))
    }

    pub fn into_client(self, token: OAuth2Token) -> KeycloakClient {
        KeycloakClient::new(self, token)
    }
}

pub struct KeycloakClient {
    auth: KeycloakAuth,
    token: OAuth2Token,
}

impl KeycloakClient {
    fn new(auth: KeycloakAuth, token: OAuth2Token) -> Self {
        KeycloakClient { auth, token }
    }

    pub fn request(
        &self,
        method: reqwest::Method,
        path: &str,
    ) -> reqwest::RequestBuilder {
        self.auth
            .http_client
            .request(method, format!("{}/{}", self.auth.url, path))
            .bearer_auth(self.token.access_token().secret())
    }

    pub async fn refresh(&mut self) -> Result<()> {
        let oauth_client = basic_client!(&self.auth);
        let token = oauth_client
            .exchange_refresh_token(self.token.refresh_token().unwrap())
            .request_async(&self.auth.http_client)
            .await?;

        self.token = token;
        Ok(())
    }

    pub fn token(&self) -> &OAuth2Token {
        &self.token
    }

    pub async fn logout(self) -> Result<()> {
        let path = format!(
            "{}/realms/{}/protocol/openid-connect/logout",
            self.auth.url, self.auth.realm
        );
        self.request(reqwest::Method::POST, &path)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}
