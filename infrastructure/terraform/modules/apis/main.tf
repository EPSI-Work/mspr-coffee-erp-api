# resource "null_resource" "enable_service_usage_api" {
#   provisioner "local-exec" {
#     command = "gcloud services enable serviceusage.googleapis.com cloudresourcemanager.googleapis.com --project ${var.project_id}"
#   }

# }

# resource "time_sleep" "wait_project_init" {
#   create_duration = "60s"

#   depends_on = [null_resource.enable_service_usage_api]
# }

resource "google_project_service" "gcp_services" {
  for_each = toset(var.gcp_service_list)
  project  = var.project_id
  service  = each.key
}

#  "serviceusage.googleapis.com",