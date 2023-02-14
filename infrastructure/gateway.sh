if [[ -z "${PROJECT_ID}" || -z "${API_ID}"  || -z "${CONFIG_ID}"  || -z "${SERVICE_ACCOUNT}" || -z "${EMAIL_SA}" || -z "${API_DEFINITION}" || -z "${GATEWAY_ID}"  || -z "${GCP_REGION}" ]]; then
 echo "Les variables d'environnement PROJECT_ID, API_ID, CONFIG_ID, SERVICE_ACCOUNT, EMAIL_SA, API_DEFINITION, GATEWAY_ID et GCP_REGION doivent être définies."
  exit 1
fi

gcloud config set project $PROJECT_ID

gcloud services enable \
   apigateway.googleapis.com \
   servicemanagement.googleapis.com \
   servicecontrol.googleapis.com

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

# export PROJECT_ID=mspr-epsi-coffee
# export API_ID=gateway-coffee
# export CONFIG_ID=api-config
# export SERVICE_ACCOUNT=gateway-test-sa
# export EMAIL_SA=aimeric.sorin@gmail.com
# export API_DEFINITION=gateway.yaml
# export GATEWAY_ID=gateway-coffee
# export GCP_REGION=europe-west9
