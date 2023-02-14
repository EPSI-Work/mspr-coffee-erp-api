#!/usr/bin/env bash
set -x
set -eo pipefail

CONFIG_ID=api-config-3
API_ID=gateway-coffee
API_DEFINITION=gateway.yaml

GATEWAY_ID=gateway-coffee
GCP_REGION=europe-west2

gcloud api-gateway api-configs create $CONFIG_ID --api=$API_ID --openapi-spec=$API_DEFINITION 

gcloud api-gateway gateways update $GATEWAY_ID --api=$API_ID --api-config=$CONFIG_ID --location=$GCP_REGION