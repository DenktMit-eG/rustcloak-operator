macro_rules! meta_map_crds {
    { $meta_name:ident: $( $($meta_type_name:ident)::* ,)* } => {
        #[macro_export]
        macro_rules! $meta_name {
            { $type_name:ident => $type_expression:expr } => {
                [
                    $(
                        {
                            type $type_name = $crate $(:: $meta_type_name)*;
                            $type_expression
                        },
                    )*
                ]
            }
        }
    };
}

#[macro_export]
macro_rules! map_all_crds {
    { $type_name:ident => $type_expression:expr } => {
            $crate::map_rest_crds!($type_name => $type_expression).into_iter()
                .chain(
                    $crate::map_plumbing_crds!($type_name => $type_expression)
                )
    };
}

meta_map_crds!(foobar:
    ClusterKeycloakApiObject,
);
meta_map_crds!(map_plumbing_crds:
    KeycloakClientCredential,
    KeycloakRoleMapping,
    KeycloakUserCredential,
    api_object::ClusterKeycloakApiObject,
    api_object::KeycloakApiObject,
    instance::ClusterKeycloakInstance,
    instance::KeycloakInstance,
);

meta_map_crds!(map_rest_crds:
    realm::ClusterKeycloakRealm,
    authentication_flow::KeycloakAuthenticationFlow,
    authenticator_config::KeycloakAuthenticatorConfig,
    client::KeycloakClient,
    client_scope::KeycloakClientScope,
    component::KeycloakComponent,
    group::KeycloakGroup,
    identity_provider::KeycloakIdentityProvider,
    identity_provider_mapper::KeycloakIdentityProviderMapper,
    organization::KeycloakOrganization,
    protocol_mapper::KeycloakProtocolMapper,
    realm::KeycloakRealm,
    required_action_provider::KeycloakRequiredActionProvider,
    resource::KeycloakResource,
    role::KeycloakRole,
    scope::KeycloakScope,
    user::KeycloakUser,
);

pub use map_all_crds;
pub use map_plumbing_crds;
pub use map_rest_crds;
