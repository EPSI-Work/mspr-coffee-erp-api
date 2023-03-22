
# TO DO 
# Creating GCP project
# Enabling APIs (gcloud services enable serviceusage.googleapis.com cloudresourcemanager.googleapis.com --project ${var.project_id})
# Create terraform-admin service account with the role editor
# Create bucket for storing Terraform state
gcloud storage buckets create gs://bucket_name --project=project_name --default-storage-class=COLDLINE --location=europe-west9  
# Création de la clé du compte de service
