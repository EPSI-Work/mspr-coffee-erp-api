# if [[ -z "${PROJECT_ID}" || -z "${API_ID}"  || -z "${CONFIG_ID}" || -z "${API_DEFINITION}" || -z "${GATEWAY_ID}"  || -z "${GCP_REGION}" ]]; then
#  echo "Les variables d'environnement PROJECT_ID, API_ID, CONFIG_ID, API_DEFINITION, GATEWAY_ID et GCP_REGION doivent être définies."
#   exit 1
# fi

gcloud config set project $PROJECT_ID

gcloud api-gateway api-configs create $CONFIG_ID --api=$API_ID --openapi-spec=$API_DEFINITION 

gcloud api-gateway gateways update $GATEWAY_ID --api=$API_ID --api-config=$CONFIG_ID --location=$GCP_REGION

# export PROJECT_ID=mspr-epsi-coffee
# export API_ID=gateway-coffee
# export CONFIG_ID=api-config-8
# export API_DEFINITION=openapi.yaml
# export GATEWAY_ID=gateway-coffee
# export GCP_REGION=europe-west2