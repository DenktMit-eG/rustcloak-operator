#!/bin/sh -e
cd "$(dirname "$0")"

PASSWORD=$(pwgen)
echo "$PASSWORD" | wl-copy
kubectl apply -f keycloak-instance.yaml
kubectl delete secret keycloak-credentials || true

kubectl create secret generic keycloak-credentials \
	--from-literal=user=admin \
	--from-literal=password="${PASSWORD}"

podman run --rm --name rustcloak-keycloak -p 8080:8080 \
        -e KC_BOOTSTRAP_ADMIN_USERNAME=admin -e KC_BOOTSTRAP_ADMIN_PASSWORD=${PASSWORD} \
        quay.io/keycloak/keycloak:latest \
        start-dev
