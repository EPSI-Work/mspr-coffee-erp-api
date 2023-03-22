output "gateway_ip" {
  description = "The hostname of the gateway"
  value = module.build_infra.gateway_ip
}

output "artifact_id" {
  description = "The id of the artifact registery"
  value = module.build_infra.artifact_id
}