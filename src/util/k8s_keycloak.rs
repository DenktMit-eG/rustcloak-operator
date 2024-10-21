use k8s_openapi::api::core::v1::Secret;
use kube::{Api, ResourceExt};
use log::debug;
use oauth2::{ClientId, ClientSecret};

use crate::{
    api::{KeycloakAuth, KeycloakAuthBuilder, KeycloakClient},
    crd::KeycloakInstance,
    error::{Error, Result},
};

use super::SecretUtils;

pub struct K8sKeycloakBuilder<'a> {
    auth: KeycloakAuth,
    instance: &'a KeycloakInstance,
    client: &'a kube::Client,
}

impl<'a> K8sKeycloakBuilder<'a> {
    pub fn new(
        instance: &'a KeycloakInstance,
        client: &'a kube::Client,
    ) -> Self {
        let mut builder = KeycloakAuthBuilder::default();
        builder.url(instance.spec.base_url.clone());
        if let Some(realm) = &instance.spec.realm {
            builder.realm(realm.clone());
        }

        if let Some(client) = &instance.spec.client {
            builder.client_id(ClientId::new(client.id.clone()));
            if let Some(secret) = &client.secret {
                builder.client_secret(ClientSecret::new(secret.clone()));
            }
        }

        K8sKeycloakBuilder {
            auth: builder.build().unwrap(),
            instance,
            client,
        }
    }

    pub async fn with_credentials(self) -> Result<KeycloakClient> {
        let ns = self.instance.namespace().ok_or(Error::NoNamespace)?;
        let secret_api = Api::<Secret>::namespaced(self.client.clone(), &ns);
        let credential_secret_name =
            self.instance.credential_secret_name().to_string();
        debug!( "Using keycloak with credential secret {ns}/{credential_secret_name}");

        let (user, password) = secret_api
            .get(&credential_secret_name)
            .await?
            .credentials(&self.instance.spec.credentials)?;

        self.auth.login_with_credentials(&user, &password).await
    }

    pub async fn with_token(self) -> Result<KeycloakClient> {
        let ns = self.instance.namespace().ok_or(Error::NoNamespace)?;
        let secret_api = Api::<Secret>::namespaced(self.client.clone(), &ns);
        let token_secret_name = self.instance.token_secret_name().to_string();
        debug!("Using keycloak with token secret {ns}/{token_secret_name}");

        let token = secret_api
            .get(&token_secret_name)
            .await?
            .token(self.instance)?;

        Ok(self.auth.into_client(token))
    }
}
