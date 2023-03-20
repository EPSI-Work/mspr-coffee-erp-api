# Configuration du fournisseur GCP
provider "google" {
  project = var.project_id
  region  = var.gcp_region
}


resource "google_artifact_registry_repository" "erp_api" {
  description   = "Artifact Repository for ERP api"
  format        = "DOCKER"
  location      = "europe-west9"
  project       = "mspr-epsi-coffee"
  repository_id = "erp-api"
}



resource "google_service_account" "gateway_sa" {
  account_id   = "gateway-sa"
  display_name = "Gateway Service Account"
  project      = "mspr-epsi-coffee"
}

resource "google_service_account" "github_action_sa" {
  account_id   = "github-action-sa"
  display_name = "GitHub Actions Service Account"
  project      = "mspr-epsi-coffee"
}



# terraform import google_project_service.bigquery_googleapis_com 996044745806/bigquery.googleapis.com
resource "google_project_service" "apigateway_googleapis_com" {
  project = "996044745806"
  service = "apigateway.googleapis.com"
}

# terraform import google_project_service.apikeys_googleapis_com 996044745806/apikeys.googleapis.com
resource "google_project_service" "artifactregistry_googleapis_com" {
  project = "996044745806"
  service = "artifactregistry.googleapis.com"
}

# terraform import google_project_service.pubsub_googleapis_com 996044745806/pubsub.googleapis.com
resource "google_project_service" "iamcredentials_googleapis_com" {
  project = "996044745806"
  service = "iamcredentials.googleapis.com"
}

# terraform import google_project_service.fcm_googleapis_com 996044745806/fcm.googleapis.com
resource "google_project_service" "run_googleapis_com" {
  project = "996044745806"
  service = "run.googleapis.com"
}




# terraform import google_project_service.cloudasset_googleapis_com 996044745806/cloudasset.googleapis.com
resource "google_project_service" "gateway_coffee_0bs2w9fb1lddt_apigateway_mspr_epsi_coffee_cloud_goog" {
  project = "996044745806"
  service = "gateway-coffee-0bs2w9fb1lddt.apigateway.mspr-epsi-coffee.cloud.goog"
}

