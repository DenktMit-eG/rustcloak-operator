use rustcloak_operator::crd::KeycloakRealm;
use up_impl::{Container, HasContainer, HasUp};

#[tokio::main]
async fn main() {
    type O = <<<KeycloakRealm as HasUp>::Up as HasContainer>::Container as Container>::Output;
    println!("{}", std::any::type_name::<O>());
}
