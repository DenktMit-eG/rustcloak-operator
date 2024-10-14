# rustcloak-operator

![Architecture](./arch.svg)

## TODO:

### Sessions

Currently rustcloak does not handle sessions well

* [ ] The operator must schedule session keepalives. The timing of this
  keepalives should be queried from the keycloak realm settings. The next
  session keepalive should be saved in the instance status object.
* [ ] Similiar to the keepalives, the operator must schedule renewals of the session. 
* [ ] Currently to retrieve a session token, we make use of The
     [keycloak crate](https://crates.io/crates/keycloak), which doesn't support 
     a proper oauth flow. We should switch to the
     [oauth2 crate](https://crates.io/crates/oauth2) instead, which supports a 
     proper oauth flow with refresh tokens.

Currently we have a session controller and an instance controller. Both controllers
do similiar things, maybe it's a good idea to have a single controller for both

