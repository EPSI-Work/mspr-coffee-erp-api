output "artifact_id" {
  description = "The id of the artifact registery"
  value = google_artifact_registry_repository.api_repository.repository_id
}
