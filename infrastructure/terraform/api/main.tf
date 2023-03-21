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

resource "google_project_service" "enable_servicecontrol" {
  project  = var.project_id
  service  = "servicecontrol.googleapis.com"
}

resource "google_project_service" "enable_serviceusage" {
  project  = var.project_id
  service  = "serviceusage.googleapis.com"
}

resource "google_project_service" "enable_cloudapis" {
  project  = var.project_id
  service  = "cloudapis.googleapis.com"
}

resource "google_project_service" "enable_api_keys" {
  provider = google-beta
  project  = var.project_id
  service  = "apikeys.googleapis.com"
}