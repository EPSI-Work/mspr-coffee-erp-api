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

resource "null_resource" "enable_service_usage_api" {
  provisioner "local-exec" {
    command = "gcloud services enable serviceusage.googleapis.com cloudresourcemanager.googleapis.com --project ${var.project_id}"
  }

}

# Wait for the new configuration to propagate
# (might be redundant)
resource "time_sleep" "wait_project_init" {
  create_duration = "60s"

  depends_on = [null_resource.enable_service_usage_api]
}

variable "gcp_service_list" {
  description = "The list of apis necessary for the project"
  type        = list(string)
  default = [
    "apigateway.googleapis.com",
    "servicemanagement.googleapis.com",
    "iam.googleapis.com",
    "artifactregistry.googleapis.com",
    "apigateway.googleapis.com",
    "servicecontrol.googleapis.com",
    "cloudapis.googleapis.com",
    "apikeys.googleapis.com"
  ]
}

resource "google_project_service" "gcp_services" {
  for_each = toset(var.gcp_service_list)
  project  = var.project_id
  service  = each.key
}

#  "serviceusage.googleapis.com",