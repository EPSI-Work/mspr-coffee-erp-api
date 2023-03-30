variable "gateway_region" {
  type = string
  description = "The GCP region where the API Gateway will be deployed."
  default = "europe-west2"
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


