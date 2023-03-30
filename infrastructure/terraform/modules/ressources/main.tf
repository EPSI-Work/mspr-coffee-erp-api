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
resource "google_project_iam_member" "github_service_account_user" {
  project = var.project_id
  role    = "roles/iam.serviceAccountUser"
  member  = "serviceAccount:${google_service_account.github_action_service_account.email}"
 
}
resource "google_project_iam_member" "gateway_service_account_user" {
  project = var.project_id
  role    = "roles/iam.serviceAccountUser"
  member =  "serviceAccount:${google_service_account.api_gateway_service_account.email}"
}

resource "google_project_iam_member" "github_run" {
  project = var.project_id
  role    = "roles/run.developer"
  member  = "serviceAccount:${google_service_account.github_action_service_account.email}"
 
}
resource "google_project_iam_member" "gateway_run" {
  project = var.project_id
  role    = "roles/run.developer"
  member =  "serviceAccount:${google_service_account.api_gateway_service_account.email}"
}

resource "time_rotating" "key_rotation_sa" {
  rotation_days = 30
}

resource "google_service_account_key" "github_service_account_key" {
  service_account_id   = google_service_account.github_action_service_account.name

  keepers = {
    rotation_time = time_rotating.key_rotation_sa.rotation_rfc3339
  }
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

