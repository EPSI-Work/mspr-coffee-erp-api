// TO DO 
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

# Enabling API
resource "google_project_service" "enable_api_gateway" {
  project  = var.project_id
  service  = "apigateway.googleapis.com"
}

resource "google_project_service" "enable_service_management" {
  project  = var.project_id
  service  = "servicemanagement.googleapis.com"
}

resource "google_project_service" "enable_iam" {
  project  = var.project_id
  service  = "iam.googleapis.com"
}

resource "google_project_service" "enable_artifactregistry" {
  project  = var.project_id
  service  = "artifactregistry.googleapis.com"
}

resource "google_project_service" "enable_apigateway" {
  project  = var.project_id
  service  = "apigateway.googleapis.com"
}

# Création du compte de service Gateway
resource "google_service_account" "api_gateway_service_account" {
  account_id   = var.gateway_sa
  display_name = "Gateway Service Account"
}

resource "google_project_iam_binding" "service_account_user" {
  project  = var.project_id
  role    = "roles/iam.serviceAccountUser"
  members = ["serviceAccount:${google_service_account.api_gateway_service_account.email}"]
}

resource "google_project_iam_binding" "run_developer" {
  project  = var.project_id
  role    = "roles/run.developer"
  members = ["serviceAccount:${google_service_account.api_gateway_service_account.email}"]
}

# Création du compte de service Github Actions
resource "google_service_account" "github_action_sa" {
  account_id   = var.github_action_sa
  display_name = "GitHub Actions Service Account"
  project      = var.project_id
}
# TO DO Add roles 

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
  members    = ["serviceAccount:${google_service_account.api_gateway_service_account.email}"]
}

# Création de l'API Gateway
resource "google_api_gateway_api" "api_gateway" {
  provider = google-beta
  api_id   = var.api_id
}

resource "google_api_gateway_api_config" "api_config" {
  provider = google-beta

  api        = google_api_gateway_api.api_gateway.id
  api_config_id = var.api_config_id
  openapi_documents {
    document {
      path = "spec.yaml"
      contents = filebase64("openapi.yaml")
    }
  }
}

resource "google_api_gateway_gateway" "api_gateway_gateway" {
  provider = google-beta
  gateway_id    = var.gateway_id
  api_config = google_api_gateway_api_config.api_config.id
}

resource "google_service_account_iam_binding" "api_gateway_service_account_iam_binding" {

  service_account_id = google_service_account.api_gateway_service_account.name
  role               = "roles/iam.serviceAccountUser"
  members            = ["user:${var.service_account_email}"]
}










# resource "google_project_service" "enable_compute_engine" {
#   provider = google-beta
#   project  = var.project_id
#   service  = "compute.googleapis.com"
# }

# resource "google_project_service" "enable_api_keys" {
#   provider = google-beta
#   project  = var.project_id
#   service  = "apikeys.googleapis.com"
# }

# Création de la clé du compte de service
# resource "google_service_account_key" "api_gateway_service_account_key" {
#   service_account_id = google_service_account.api_gateway_service_account.id
#   private_key_type   = "json"
# }


# resource "google_service_account" "api_gateway_service_account" {
#   project      = var.project_id
#   account_id   = var.service_account_id
#   display_name = "API Gateway Service Account"
# }

# Création du backend
# resource "google_api_gateway_api_config" "api_config" {
#   project   = var.project_id
#   api_id    = google_api_gateway_api.api_gateway.id
#   config_id = var.config_id
#   openapi_documents {
#     document = var.api_definition
#   }
#   backend_configs {
#     backend {
#       google_cloud_run {
#         service_name = var.cloud_run_id
#         region       = var.cloud_run_region
#         backend_auth {
#           jwt {
#             audience = google_api_gateway_api.api_gateway.default_jwt_audience
#             issuer   = google_api_gateway_api.api_gateway.default_jwt_issuer
#           }
#         }
#       }
#     }
#   }
#   default_jwt_audience    = google_api_gateway_api.api_gateway.default_jwt_audience
#   default_jwt_issuer      = google_api_gateway_api.api_gateway.default_jwt_issuer
#   gateway_service_account = google_service_account.api_gateway_service_account.email
# }


# Ressources nécessaires pour l'API Gateway
# resource "google_api_gateway_api" "api_gateway" {
#   project      = var.project_id
#   display_name = "API Gateway"
# }