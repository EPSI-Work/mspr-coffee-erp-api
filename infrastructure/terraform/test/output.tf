output "gateway_ip" {
  value = google_api_gateway_gateway.api_gateway_gateway.default_hostname
}
