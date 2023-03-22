gcloud config set project $PROJECT_ID

gcloud iam service-accounts create $SERVICE_ACCOUNT \
   --display-name="Terraform Admin"

gcloud projects add-iam-policy-binding $PROJECT_ID \
   --member="serviceAccount:$SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com" \
   --role="roles/owner"

gcloud iam service-accounts keys create key.json \
   --iam-account=$SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com

gcloud services enable serviceusage.googleapis.com \
    cloudresourcemanager.googleapis.com \
    servicemanagement.googleapis.com \
    iam.googleapis.com \
    artifactregistry.googleapis.com \
    apigateway.googleapis.com \
    servicecontrol.googleapis.com \
    cloudapis.googleapis.com \
    apikeys.googleapis.com

gcloud storage buckets create gs://$BUCKET_NAME --default-storage-class=COLDLINE --location=europe-west9

# Copy the BUCKET_NAME variable into the backend terraform state
# export PROJECT_ID=sandbox-381015
# export SERVICE_ACCOUNT=terraform-admin
# export BUCKET_NAME=terraform-state-5f96fc31-b260-40fc-8244-d3f208c41722