output "gateway_ip" {
  description = "The hostname of the gateway"
  value = google_api_gateway_gateway.api_gateway_gateway.default_hostname
}

output "artifact_id" {
  description = "The id of the artifact registery"
  value = google_artifact_registry_repository.api_repository.repository_id
}
