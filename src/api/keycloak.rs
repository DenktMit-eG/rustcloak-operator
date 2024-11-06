use crate::error::Result;
use derive_builder::Builder;
use log::debug;
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

/// A wrapper around the OAuth2 token response that does safe the expiration time as an absolute
/// timestime.
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

/// This struct represents an unauthenticated Keycloak Client. This struct can be turned into an
/// authenticated client by either providing an [`OAuth2Token`] or providing a username and
/// password.
#[derive(Debug, Builder, Clone)]
pub struct KeycloakApiAuth {
    #[builder(setter(into))]
    url: String,
    #[builder(setter(into), default = "self.default_realm()")]
    realm: String,
    #[builder(setter(into), default = "self.default_auth_url()")]
    auth_url: AuthUrl,
    #[builder(setter(into), default = "self.default_token_url()")]
    token_url: TokenUrl,
    #[builder(setter(into), default = "self.default_logout_url()")]
    logout_url: String,
    #[builder(setter(into), default = "self.default_client_id()")]
    client_id: ClientId,
    #[builder(setter(into, strip_option), default)]
    client_secret: Option<ClientSecret>,
    #[builder(setter(into), default)]
    http_client: reqwest::Client,
}

impl KeycloakApiAuthBuilder {
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

    fn default_logout_url(&self) -> String {
        let url = self.url.as_ref().unwrap();
        let realm = self.realm.clone().unwrap_or_else(|| self.default_realm());
        format!("{url}/realms/{realm}/protocol/openid-connect/logout",)
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

impl KeycloakApiAuth {
    /// Creates a new KeycloakAuth struct with the given URL. If you need more customization, you
    /// can use the [`KeycloakAuthBuilder`] struct.
    pub fn new(url: &str) -> Self {
        KeycloakApiAuthBuilder::default()
            .url(url.to_string())
            .build()
            .unwrap()
    }

    /// Tries to login with given credentials. On success a new [`KeycloakAuthBuilder`] is
    /// returned.
    pub async fn login_with_credentials(
        self,
        user_name: &str,
        password: &str,
    ) -> Result<KeycloakApiClient> {
        debug!("Logging in with user {}", user_name);

        let user = ResourceOwnerUsername::new(user_name.to_string());
        let password = ResourceOwnerPassword::new(password.to_string());

        let oauth_client = basic_client!(&self);
        let token = oauth_client
            .exchange_password(&user, &password)
            .request_async(&self.http_client)
            .await?;
        debug!("Successfully logged in with user {}", user_name);

        Ok(KeycloakApiClient::new(self, OAuth2Token::create(token)))
    }

    /// Creates a new [`KeycloakClient`] using a given [`OAuth2Token`]. There is no checking done
    /// to verify that this token is valid.
    pub fn into_client(self, token: OAuth2Token) -> KeycloakApiClient {
        KeycloakApiClient::new(self, token)
    }
}

/// This struct provides the functionality to interact with a Keycloak server. For the creation of
/// this client, see [`KeycloakAuth`].
#[derive(Debug, Clone)]
pub struct KeycloakApiClient {
    auth: KeycloakApiAuth,
    token: OAuth2Token,
}

impl KeycloakApiClient {
    fn new(auth: KeycloakApiAuth, token: OAuth2Token) -> Self {
        KeycloakApiClient { auth, token }
    }

    fn request_url(
        &self,
        method: reqwest::Method,
        url: &str,
    ) -> reqwest::RequestBuilder {
        self.auth
            .http_client
            .request(method, url)
            .bearer_auth(self.token.access_token().secret())
    }
    /// Prepares a request to the configured keycloak server. KeycloakClient prepares the
    /// authentication headers for you.
    pub fn request(
        &self,
        method: reqwest::Method,
        path: &str,
    ) -> reqwest::RequestBuilder {
        self.request_url(
            method,
            &format!(
                "{}/{}",
                self.auth.url.trim_end_matches('/'),
                path.trim_start_matches('/')
            ),
        )
        .bearer_auth(self.token.access_token().secret())
    }

    /// Uses the refresh token to get a new access token. The old token is invalidated.
    pub async fn refresh(&mut self) -> Result<&OAuth2Token> {
        let oauth_client = basic_client!(&self.auth);
        let token = oauth_client
            .exchange_refresh_token(self.token.refresh_token().unwrap())
            .request_async(&self.auth.http_client)
            .await?;

        self.token = OAuth2Token::create(token);
        Ok(&self.token)
    }

    /// Returns the current token.
    pub fn token(&self) -> &OAuth2Token {
        &self.token
    }

    /// Returns the realm of the current client.
    pub fn realm(&self) -> &str {
        &self.auth.realm
    }

    /// Terminates the current session.
    pub async fn logout(self) -> Result<()> {
        let logout_url = self.auth.logout_url.clone();
        self.request_url(reqwest::Method::POST, &logout_url)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}
