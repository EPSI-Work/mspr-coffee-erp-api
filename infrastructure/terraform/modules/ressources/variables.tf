variable "project_id" {
  type = string
  description = "The ID of the GCP project where the resources will be created."
}

variable "gateway_sa" {
  type = string
  description = "The name of the service account used by the API Gateway."
  default = "gateway-sa"
}

variable "service_account_email" {
  type = string
  description = "The email address of the service account used to deploy the Cloud Run service."
}

variable "github_action_sa" {
  type = string
  description = "The name of the service account used by the GitHub Actions workflow."
  default = "github-sa"
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

variable "artifact_repo_id" {
  type = string
  description = "The ID of the Artifact Registry repository used to store the container image."
}

variable "cloud_run_id" {
  type = string
  description = "The ID of the Cloud Run service."
}

variable "api_id" {
  type = string
  description = "The ID of the API Gateway service."
}

variable "api_config_id" {
  type = string
  description = "The ID of the API Gateway configuration."
}

variable "openapi_file" {
  type = string
  description = "The path of the openapi file used by the API Gateway."
}


