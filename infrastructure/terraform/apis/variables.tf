variable "credentials_file" {
  type = string
  description = "Path to the service account credentials JSON file for GCP authentication."
}

variable "project_id" {
  type = string
  description = "The ID of the GCP project where the resources will be created."
}

variable "gcp_region" {
  type = string
  description = "The GCP region where the Cloud Run service will be deployed."
  default = "europe-west9"
}