variable "credentials_file" {}

variable "project_id" {}

variable "service_account_email" {
  default = "aimeric.sorin@gmail.com"
}
variable "artifact_repo_id" {
  default = "artifact-erp"
}
variable "gcp_region" {
  default = "europe-west9"
}
variable "api_id" {
  default = "api-erp"
}
variable "api_config_id" {
  default = "config-gateway"
}
variable "gateway_id" {
  default = "gateway-erp"
}
variable "gateway_sa" {
  default = "gateway-sa"
}
variable "github_action_sa" {
  default = "github-sa"
}
variable "cloud_run_id" {
  default = "cloud-run-erp"
}
