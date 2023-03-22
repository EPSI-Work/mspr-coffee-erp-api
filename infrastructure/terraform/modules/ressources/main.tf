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
      path = var.openapi_file
      contents = filebase64(var.openapi_file)
    }
  }
}

resource "google_api_gateway_gateway" "api_gateway_gateway" {
  provider = google-beta
  gateway_id = google_api_gateway_api.api_gateway.api_id
  api_config = google_api_gateway_api_config.api_config.id
  region = var.gateway_region
}
