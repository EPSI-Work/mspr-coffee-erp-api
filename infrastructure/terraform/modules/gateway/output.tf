output "gateway_ip" {
  description = "The URL of the gateway"
  value = google_api_gateway_gateway.api_gateway_gateway.default_hostname
}
