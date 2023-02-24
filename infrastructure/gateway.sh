if [[ -z "${PROJECT_ID}" || -z "${API_ID}"  || -z "${CONFIG_ID}"  || -z "${SERVICE_ACCOUNT}" || -z "${EMAIL_SA}" || -z "${API_DEFINITION}" || -z "${GATEWAY_ID}"  || -z "${GCP_REGION}" || -z "${FIREWALL_ID}" || -z "${TAG}"  || -z "${CLOUD_RUN_ID}" || -z "${CLOUD_RUN_REGION}" ]]; then
 echo "Les variables d'environnement PROJECT_ID, API_ID, CONFIG_ID, SERVICE_ACCOUNT, EMAIL_SA, API_DEFINITION, GATEWAY_ID, GCP_REGION, FIREWALL_ID, TAG, CLOUD_RUN_ID et CLOUD_RUN_REGION doivent être définies."  
  exit 1
fi

gcloud config set project $PROJECT_ID

gcloud services enable \
   apigateway.googleapis.com \
   servicemanagement.googleapis.com \
   servicecontrol.googleapis.com \
   compute.googleapis.com

gcloud api-gateway apis create $API_ID

gcloud iam service-accounts create $SERVICE_ACCOUNT \
   --display-name="Gateway Service Account"

gcloud iam service-accounts add-iam-policy-binding $SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com \
  --member user:$EMAIL_SA \
  --role roles/iam.serviceAccountUser

gcloud api-gateway api-configs create $CONFIG_ID \
  --api=$API_ID --openapi-spec=$API_DEFINITION \
  --backend-auth-service-account=$SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com

gcloud api-gateway gateways create $GATEWAY_ID \
  --api-config=$CONFIG_ID \
  --api=$API_ID  \
  --location=$GCP_REGION

# gcloud compute firewall-rules create $FIREWALL_ID \
#   --direction=INGRESS \
#   --priority=1000 \
#   --network=default \
#   --action=DENY \
#   --rules=all \
#   --source-ranges=0.0.0.0/0 \
#   --target-tags=$TAG \
#   --description="Deny ingress to Cloud Run from external networks"

# gcloud run services update $CLOUD_RUN_ID \
#   --platform=managed \
#   --region=$CLOUD_RUN_REGION\
#   --remove-labels=$TAG=enabled




  gcloud run services describe $CLOUD_RUN_ID --platform=managed --region=$CLOUD_RUN_REGION 




# export PROJECT_ID=mspr-epsi-coffee
# export API_ID=gateway-coffee
# export CONFIG_ID=api-config
# export SERVICE_ACCOUNT=gateway-test-sa
# export EMAIL_SA=aimeric.sorin@gmail.com
# export API_DEFINITION=gateway.yaml
# export GATEWAY_ID=gateway-coffee
# export GCP_REGION=europe-west2
# export FIREWALL_ID=deny-erp-connection
# export TAG=only-from-gateway
# export CLOUD_RUN_ID=erp-api-runner
# export CLOUD_RUN_REGION=europe-west9
