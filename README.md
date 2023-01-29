TODO init the GCP project : 

gcloud config set project $PROJECT_ID

gcloud services enable \
   artifactregistry.googleapis.com \
   iamcredentials.googleapis.com \
   containerregistry.googleapis.com \
   run.googleapis.com 

gcloud iam service-accounts create $SERVICE_ACCOUNT \
   --display-name="GitHub Actions Service Account"

gcloud projects add-iam-policy-binding $PROJECT_ID \
   --member="serviceAccount:$SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com" \
   --role="roles/iam.serviceAccountUser"

gcloud projects add-iam-policy-binding $PROJECT_ID \
   --member="serviceAccount:$SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com" \
   --role="roles/run.developer"

gcloud projects add-iam-policy-binding $PROJECT_ID \
   --member="serviceAccount:$SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com" \
   --role="roles/storage.admin"

gcloud iam service-accounts keys create key.json \
   --iam-account=$SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com

TODO add secret to Github Repo : GCP_SERVICE_ACCOUNT_KEY and GCP_PROJECT_ID



