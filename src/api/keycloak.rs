use crate::error::Result;
use derive_builder::Builder;
use log::info;
use oauth2::basic::BasicClient;
use oauth2::{
    basic::BasicTokenType, AuthUrl, ClientId, EmptyExtraTokenFields,
    ResourceOwnerPassword, ResourceOwnerUsername, StandardTokenResponse,
    TokenUrl,
};
use oauth2::{ClientSecret, TokenResponse};

// Because of the stupidly overdesigned oauth2 library, we use this generator macro
// to not have to write idiotic amounts of Generic types.
macro_rules! basic_client {
    ($auth:expr) => {{
        let builder = BasicClient::new($auth.client_id.clone())
            .set_auth_uri($auth.auth_url.clone())
            .set_token_uri($auth.token_url.clone());
        let builder = if let Some(secret) = &$auth.client_secret {
            builder.set_client_secret(secret.clone())
        } else {
            builder
        };
        builder
    }};
}

#[derive(Debug, Clone)]
pub struct OAuth2Token {
    pub token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    pub expires: Option<chrono::DateTime<chrono::Utc>>,
}

impl OAuth2Token {
    fn create(
        token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) -> Self {
        let expires = token.expires_in().map(|e| chrono::Utc::now() + e);
        OAuth2Token { token, expires }
    }

    pub fn access_token(&self) -> &oauth2::AccessToken {
        self.token.access_token()
    }

    pub fn refresh_token(&self) -> Option<&oauth2::RefreshToken> {
        self.token.refresh_token()
    }
}

#[derive(Debug, Builder, Clone)]
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
    #[builder(setter(into, strip_option), default)]
    client_secret: Option<ClientSecret>,
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
            "{url}/realms/{realm}/protocol/openid-connect/auth"
        ))
        .unwrap()
    }

    fn default_token_url(&self) -> TokenUrl {
        let url = self.url.as_ref().unwrap();
        let realm = self.realm.clone().unwrap_or_else(|| self.default_realm());
        TokenUrl::new(format!(
            "{url}/realms/{realm}/protocol/openid-connect/token"
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
        user_name: &str,
        password: &str,
    ) -> Result<KeycloakClient> {
        info!("Logging in with user {}", user_name);

        let user = ResourceOwnerUsername::new(user_name.to_string());
        let password = ResourceOwnerPassword::new(password.to_string());

        let oauth_client = basic_client!(&self);
        let token = oauth_client
            .exchange_password(&user, &password)
            .request_async(&self.http_client)
            .await?;
        info!("Successfully logged in with user {}", user_name);

        Ok(KeycloakClient::new(self, OAuth2Token::create(token)))
    }

    pub fn into_client(self, token: OAuth2Token) -> KeycloakClient {
        KeycloakClient::new(self, token)
    }
}

#[derive(Debug, Clone)]
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

    pub async fn refresh(&mut self) -> Result<&OAuth2Token> {
        let oauth_client = basic_client!(&self.auth);
        let token = oauth_client
            .exchange_refresh_token(self.token.refresh_token().unwrap())
            .request_async(&self.auth.http_client)
            .await?;

        self.token = OAuth2Token::create(token);
        Ok(&self.token)
    }

    pub fn token(&self) -> &OAuth2Token {
        &self.token
    }

    pub fn realm(&self) -> &str {
        &self.auth.realm
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
