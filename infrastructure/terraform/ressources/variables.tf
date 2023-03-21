variable "credentials_file" {
  type = string
  description = "Path to the service account credentials JSON file for GCP authentication."
}

variable "project_id" {
  type = string
  description = "The ID of the GCP project where the resources will be created."
}

variable "service_account_email" {
  type = string
  description = "The email address of the service account used to deploy the Cloud Run service."
  default = "aimeric.sorin@gmail.com"
}

variable "artifact_repo_id" {
  type = string
  description = "The ID of the Artifact Registry repository used to store the container image."
  default = "artifact-erp"
}

variable "gcp_region" {
  type = string
  description = "The GCP region where the Cloud Run service will be deployed."
  default = "europe-west9"
}

variable "gateway_region" {
  type = string
  description = "The GCP region where the API Gateway will be deployed."
  default = "europe-west2"
}

variable "api_id" {
  type = string
  description = "The ID of the API Gateway service."
  default = "api-erp"
}

variable "api_config_id" {
  type = string
  description = "The ID of the API Gateway configuration."
  default = "config-gateway"
}

variable "gateway_sa" {
  type = string
  description = "The name of the service account used by the API Gateway."
  default = "gateway-sa"
}

variable "github_action_sa" {
  type = string
  description = "The name of the service account used by the GitHub Actions workflow."
  default = "github-sa"
}

variable "cloud_run_id" {
  type = string
  description = "The ID of the Cloud Run service."
  default = "cloud-run-erp"
}