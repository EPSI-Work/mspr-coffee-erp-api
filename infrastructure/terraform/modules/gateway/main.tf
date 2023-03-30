resource "google_api_gateway_api" "api_gateway" {
  provider = google-beta
  api_id   = var.api_id
}

resource "google_api_gateway_api_config" "api_config" {
  provider = google-beta
  api        = google_api_gateway_api.api_gateway.api_id
  api_config_id = var.api_config_id

  openapi_documents {
    document {
      path = var.openapi_file
      contents = filebase64(var.openapi_file)
    }
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "google_api_gateway_gateway" "api_gateway_gateway" {
  provider = google-beta
  gateway_id = google_api_gateway_api.api_gateway.api_id
  api_config = google_api_gateway_api_config.api_config.id
  region = var.gateway_region
}
