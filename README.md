# rustcloak-operator

![Architecture](./arch.svg)

## TODO:

### Sessions

Currently rustcloak does not handle sessions well

* [ ] The operator must schedule session keepalives. The timing of this
  keepalives should be queried from the keycloak realm settings. The next
  session keepalive should be saved in the instance status object.
* [ ] Similiar to the keepalives, the operator must schedule renewals of the session. 

