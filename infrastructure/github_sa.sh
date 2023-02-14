if [[ -z "${PROJECT_ID}" || -z "${SERVICE_ACCOUNT}" || -z "${ARTIFACT_REPO_ID}" || -z "${GCP_REGION}"]]; then
 echo "Les variables d'environnement PROJECT_ID, SERVICE_ACCOUNT, ARTIFACT_REPO_ID et GCP_REGION doivent être définies."
  exit 1
fi

gcloud config set project $PROJECT_ID

gcloud services enable \
   iamcredentials.googleapis.com \
   artifactregistry.googleapis.com \
   run.googleapis.com 

gcloud iam service-accounts create $SERVICE_ACCOUNT \
   --display-name="GitHub Actions Service Account"

gcloud projects add-iam-policy-binding $PROJECT_ID \
   --member="serviceAccount:$SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com" \
   --role="roles/iam.serviceAccountUser"

gcloud projects add-iam-policy-binding $PROJECT_ID \
   --member="serviceAccount:$SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com" \
   --role="roles/run.developer"

gcloud iam service-accounts keys create key.json \
   --iam-account=$SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com

gcloud artifacts repositories create $ARTIFACT_REPO_ID \
    --location=$GCP_REGION \
    --repository-format=docker \
    --description="Artifact Repository for ERP api"

gcloud artifacts repositories add-iam-policy-binding $ARTIFACT_REPO_ID \
    --location=$GCP_REGION \
    --member="serviceAccount:$SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com" \  
    --role="roles/artifactregistry.writer"

# export PROJECT_ID=mspr-epsi-coffee
# export SERVICE_ACCOUNT=github-action-sa
# export ARTIFACT_REPO_ID=erp-api
# export GCP_REGION=europe-west9

