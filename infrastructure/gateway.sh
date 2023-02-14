#!/usr/bin/env bash
set -x
set -eo pipefail

PROJECT_ID=mspr-epsi-coffee

API_ID=gateway-coffee

CONFIG_ID=api-config
SERVICE_ACCOUNT=gateway-test-sa
PROJECT_ID=mspr-epsi-coffee
EMAIL=aimeric.sorin@gmail.com
API_DEFINITION=gateway.yaml

GATEWAY_ID=gateway-coffee
GCP_REGION=europe-west2

gcloud config set project $PROJECT_ID

gcloud services enable \
   apigateway.googleapis.com \
   servicemanagement.googleapis.com \
   servicecontrol.googleapis.com

gcloud api-gateway apis create $API_ID

gcloud iam service-accounts create $SERVICE_ACCOUNT \
   --display-name="Gateway Service Account Test"

gcloud iam service-accounts add-iam-policy-binding $SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com \
  --member user:$EMAIL \
  --role roles/iam.serviceAccountUser

gcloud api-gateway api-configs create $CONFIG_ID \
  --api=$API_ID --openapi-spec=$API_DEFINITION \
  --backend-auth-service-account=$SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com

gcloud api-gateway gateways create $GATEWAY_ID \
  --api-config=$CONFIG_ID \
  --api=$API_ID  \
  --location=$GCP_REGION
