variable "project_id" {
  type = string
  description = "The ID of the GCP project where the resources will be created."
}

variable "gcp_service_list" {
  type        = list(string)
  description = "The GCP region where the Cloud Run service will be deployed."
}