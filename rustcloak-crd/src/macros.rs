macro_rules! define_crd_maps {
    (
        $( $map_macro:ident => [
            $( $($type_path:ident)::+ ),* $(,)?
        ] ),* $(,)?
    ) => {
        $(
            #[macro_export]
            macro_rules! $map_macro {
                { $type_name:ident => $type_expression:expr } => {
                    [
                        $(
                            {
                                type $type_name = $crate $( :: $type_path )*;
                                $type_expression
                            },
                        )*
                    ]
                };
            }
        )*

        #[macro_export]
        macro_rules! map_all_crds {
            { $type_name:ident => $type_expression:expr } => {
                [
                    $(
                        $(
                            {
                                type $type_name = $crate $( :: $type_path )*;
                                $type_expression
                            },
                        )*
                    )*
                ]
            };
        }
    };
}

define_crd_maps! {
    map_plumbing_crds => [
        KeycloakRoleMapping,
        api_object::ClusterKeycloakApiObject,
        api_object::KeycloakApiObject,
        instance::ClusterKeycloakInstance,
        instance::KeycloakInstance,
    ],
    map_converter_crds => [
        KeycloakClientCredential,
        KeycloakUserCredential,
    ],
    map_rest_crds => [
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
    ],
}


pub use map_all_crds;
pub use map_plumbing_crds;
pub use map_rest_crds;
