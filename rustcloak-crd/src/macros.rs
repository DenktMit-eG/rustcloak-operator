macro_rules! meta_map_crds {
    { $meta_name:ident: $($meta_type_name:ident,)* } => {
        #[macro_export]
        macro_rules! $meta_name {
            { $type_name:ident => $type_expression:expr } => {
                [
                    $(
                        {
                            type $type_name = $crate::$meta_type_name;
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
                        .into_iter()
                )
    };
}

meta_map_crds!(map_plumbing_crds:
    KeycloakInstance,
    KeycloakApiObject,
);

meta_map_crds!(map_rest_crds:
//    ClusterKeycloakInstance,
//    ClusterKeycloakRealm,
    KeycloakAuthenticationFlow,
    KeycloakAuthenticatorConfig,
    KeycloakClient,
    KeycloakClientScope,
    KeycloakComponent,
    KeycloakGroup,
    KeycloakIdentityProvider,
    KeycloakIdentityProviderMapper,
    KeycloakOrganization,
    KeycloakProtocolMapper,
    KeycloakRealm,
    KeycloakRequiredActionProvider,
    KeycloakResource,
    KeycloakRole,
    KeycloakScope,
    KeycloakUser,
);

pub use map_all_crds;
pub use map_plumbing_crds;
pub use map_rest_crds;
