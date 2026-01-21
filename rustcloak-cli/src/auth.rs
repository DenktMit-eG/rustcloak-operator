use anyhow::{Context, Result, bail};
use k8s_openapi::api::core::v1::Secret;
use kube::{Api, Client};

use crate::AuthArgs;

pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    /// Resolve credentials from the given sources in priority order:
    /// 1. K8s secret (--credential-secret)
    /// 2. CLI args (--username, --password)
    /// 3. Environment variables (KEYCLOAK_USER, KEYCLOAK_PASSWORD)
    pub async fn resolve(
        args: &AuthArgs,
        namespace: &str,
        k8s_client: Option<&Client>,
    ) -> Result<Self> {
        // Priority 1: K8s secret
        if let Some(secret_name) = &args.credential_secret {
            let client = k8s_client
                .context("K8s client required when using --credential-secret")?;
            return Self::from_k8s_secret(client, namespace, secret_name).await;
        }

        // Priority 2/3: CLI args or env vars (clap handles env fallback)
        if let (Some(username), Some(password)) = (&args.username, &args.password) {
            return Ok(Self {
                username: username.clone(),
                password: password.clone(),
            });
        }

        bail!("No credentials provided. Use --credential-secret, --username/--password, or KEYCLOAK_USER/KEYCLOAK_PASSWORD env vars")
    }

    async fn from_k8s_secret(
        client: &Client,
        namespace: &str,
        secret_name: &str,
    ) -> Result<Self> {
        let secrets: Api<Secret> = Api::namespaced(client.clone(), namespace);
        let secret = secrets
            .get(secret_name)
            .await
            .with_context(|| format!("Failed to get secret '{secret_name}' in namespace '{namespace}'"))?;

        let data = secret
            .data
            .context("Secret has no data")?;

        // Use keys "user" and "password" matching existing operator pattern
        let username = data
            .get("user")
            .context("Secret missing 'user' key")?
            .0
            .clone();
        let username = String::from_utf8(username)
            .context("Invalid UTF-8 in 'user' secret key")?;

        let password = data
            .get("password")
            .context("Secret missing 'password' key")?
            .0
            .clone();
        let password = String::from_utf8(password)
            .context("Invalid UTF-8 in 'password' secret key")?;

        Ok(Self { username, password })
    }
}
