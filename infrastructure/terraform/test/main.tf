// TO DO 
// Creating GCP project
// Enabling APIs
// Create terraform-admin service account with the role editor


// enable cloudresourcemanager.googleapis.com
// add terraform-admin sa with owner role

terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "4.51.0"
    }
    google-beta = {
      source  = "hashicorp/google-beta"
      version = "4.51.0"
    }
  }
}

provider "google" {
  credentials = file(var.credentials_file)

  project = var.project_id
  region  = var.gcp_region
}

provider "google-beta" {
  credentials = file(var.credentials_file)

  project = var.project_id
  region  = var.gcp_region
}

# Création du compte de service Github Actions
resource "google_service_account" "github_action_service_account" {
  account_id   = var.github_action_sa
  display_name = "GitHub Actions Service Account"
}

# Création du compte de service Gateway
resource "google_service_account" "api_gateway_service_account" {
  account_id   = var.gateway_sa
  display_name = "Gateway Service Account"
}

# Roles
resource "google_project_iam_binding" "service_account_user" {
  project = var.project_id
  role    = "roles/iam.serviceAccountUser"
  members = ["serviceAccount:${google_service_account.github_action_service_account.email}",
  "serviceAccount:${google_service_account.api_gateway_service_account.email}"]
}
resource "google_project_iam_binding" "run_developer" {
  project = var.project_id
  role    = "roles/run.developer"
  members = ["serviceAccount:${google_service_account.github_action_service_account.email}",
  "serviceAccount:${google_service_account.api_gateway_service_account.email}"]
}


# Création du dépôt d'artefacts
resource "google_artifact_registry_repository" "api_repository" {
  repository_id = var.artifact_repo_id
  location      = var.gcp_region
  format        = "docker"
  description   = "Artifact Repository for ERP api"
}

# Attribution des rôles pour le dépôt d'artefacts
resource "google_artifact_registry_repository_iam_binding" "api_repository_writer" {
  repository = google_artifact_registry_repository.api_repository.name
  location   = var.gcp_region
  role       = "roles/artifactregistry.writer"
  members    = ["serviceAccount:${google_service_account.github_action_service_account.email}"]
}

# Création de l'API Gateway
resource "google_api_gateway_api" "api_gateway" {
  provider = google-beta
  api_id   = var.api_id
}

resource "google_api_gateway_api_config" "api_config" {
  provider = google-beta
  api        = google_api_gateway_api.api_gateway.api_id
  api_config_id = var.api_config_id

  openapi_documents {
    document {
      path = "openapi.yaml"
      contents = filebase64("openapi.yaml")
    }
  }
}

resource "google_api_gateway_gateway" "api_gateway_gateway" {
  provider = google-beta
  gateway_id = google_api_gateway_api.api_gateway.api_id
  api_config = google_api_gateway_api_config.api_config.id
  region = var.gateway_region
}













# Création de la clé du compte de service
# resource "google_service_account_key" "api_gateway_service_account_key" {
#   service_account_id = google_service_account.api_gateway_service_account.id
#   private_key_type   = "json"
# }
