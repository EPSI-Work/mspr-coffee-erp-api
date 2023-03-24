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

module "build_infra" {
  source = "../modules/ressources"

  project_id = var.project_id
  service_account_email = var.service_account_email

  artifact_repo_id = var.artifact_repo_id
  api_id = var.api_id
  api_config_id = var.api_config_id
  openapi_file = var.openapi_file
  cloud_run_id = var.cloud_run_id
}

