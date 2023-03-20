resource "google_artifact_registry_repository" "gcf_artifacts" {
  description = "This repository is created and used by Cloud Functions for storing function docker images."
  format      = "DOCKER"
  labels = {
    goog-managed-by = "cloudfunctions"
  }
  location      = "europe-west1"
  project       = "mspr-epsi-coffee"
  repository_id = "gcf-artifacts"
}
# terraform import google_artifact_registry_repository.gcf_artifacts projects/mspr-epsi-coffee/locations/europe-west1/repositories/gcf-artifacts
resource "google_artifact_registry_repository" "erp_api" {
  description   = "Artifact Repository for ERP api"
  format        = "DOCKER"
  location      = "europe-west9"
  project       = "mspr-epsi-coffee"
  repository_id = "erp-api"
}
# terraform import google_artifact_registry_repository.erp_api projects/mspr-epsi-coffee/locations/europe-west9/repositories/erp-api
resource "google_compute_firewall" "default_allow_icmp" {
  allow {
    protocol = "icmp"
  }
  description   = "Allow ICMP from anywhere"
  direction     = "INGRESS"
  name          = "default-allow-icmp"
  network       = "https://www.googleapis.com/compute/v1/projects/mspr-epsi-coffee/global/networks/default"
  priority      = 65534
  project       = "mspr-epsi-coffee"
  source_ranges = ["0.0.0.0/0"]
}
# terraform import google_compute_firewall.default_allow_icmp projects/mspr-epsi-coffee/global/firewalls/default-allow-icmp
resource "google_compute_firewall" "default_allow_ssh" {
  allow {
    ports    = ["22"]
    protocol = "tcp"
  }
  description   = "Allow SSH from anywhere"
  direction     = "INGRESS"
  name          = "default-allow-ssh"
  network       = "https://www.googleapis.com/compute/v1/projects/mspr-epsi-coffee/global/networks/default"
  priority      = 65534
  project       = "mspr-epsi-coffee"
  source_ranges = ["0.0.0.0/0"]
}
# terraform import google_compute_firewall.default_allow_ssh projects/mspr-epsi-coffee/global/firewalls/default-allow-ssh
resource "google_compute_firewall" "default_allow_internal" {
  allow {
    ports    = ["0-65535"]
    protocol = "tcp"
  }
  allow {
    ports    = ["0-65535"]
    protocol = "udp"
  }
  allow {
    protocol = "icmp"
  }
  description   = "Allow internal traffic on the default network"
  direction     = "INGRESS"
  name          = "default-allow-internal"
  network       = "https://www.googleapis.com/compute/v1/projects/mspr-epsi-coffee/global/networks/default"
  priority      = 65534
  project       = "mspr-epsi-coffee"
  source_ranges = ["10.128.0.0/9"]
}
# terraform import google_compute_firewall.default_allow_internal projects/mspr-epsi-coffee/global/firewalls/default-allow-internal
resource "google_compute_firewall" "default_allow_rdp" {
  allow {
    ports    = ["3389"]
    protocol = "tcp"
  }
  description   = "Allow RDP from anywhere"
  direction     = "INGRESS"
  name          = "default-allow-rdp"
  network       = "https://www.googleapis.com/compute/v1/projects/mspr-epsi-coffee/global/networks/default"
  priority      = 65534
  project       = "mspr-epsi-coffee"
  source_ranges = ["0.0.0.0/0"]
}
# terraform import google_compute_firewall.default_allow_rdp projects/mspr-epsi-coffee/global/firewalls/default-allow-rdp
resource "google_project" "mspr_epsi_coffee" {
  auto_create_network = true
  billing_account     = "017415-41215F-BE81EC"
  labels = {
    firebase = "enabled"
  }
  name       = "mspr-epsi-coffee"
  project_id = "mspr-epsi-coffee"
}
# terraform import google_project.mspr_epsi_coffee projects/mspr-epsi-coffee
resource "google_service_account" "gateway_sa" {
  account_id   = "gateway-sa"
  display_name = "Gateway Service Account"
  project      = "mspr-epsi-coffee"
}
# terraform import google_service_account.gateway_sa projects/mspr-epsi-coffee/serviceAccounts/gateway-sa@mspr-epsi-coffee.iam.gserviceaccount.com
resource "google_service_account" "iosmatch" {
  account_id   = "iosmatch"
  description  = "Permet d'accéder et de sauvegarder les clé et les profiles de déploiement apple"
  display_name = "iosmatch"
  project      = "mspr-epsi-coffee"
}
# terraform import google_service_account.iosmatch projects/mspr-epsi-coffee/serviceAccounts/iosmatch@mspr-epsi-coffee.iam.gserviceaccount.com
resource "google_service_account" "mspr_epsi_coffee" {
  account_id   = "mspr-epsi-coffee"
  display_name = "App Engine default service account"
  project      = "mspr-epsi-coffee"
}
# terraform import google_service_account.mspr_epsi_coffee projects/mspr-epsi-coffee/serviceAccounts/mspr-epsi-coffee@mspr-epsi-coffee.iam.gserviceaccount.com
resource "google_service_account" "996044745806_compute" {
  account_id   = "996044745806-compute"
  display_name = "Default compute service account"
  project      = "mspr-epsi-coffee"
}
# terraform import google_service_account.996044745806_compute projects/mspr-epsi-coffee/serviceAccounts/996044745806-compute@mspr-epsi-coffee.iam.gserviceaccount.com
resource "google_logging_log_sink" "a_default" {
  destination            = "logging.googleapis.com/projects/mspr-epsi-coffee/locations/global/buckets/_Default"
  filter                 = "NOT LOG_ID(\"cloudaudit.googleapis.com/activity\") AND NOT LOG_ID(\"externalaudit.googleapis.com/activity\") AND NOT LOG_ID(\"cloudaudit.googleapis.com/system_event\") AND NOT LOG_ID(\"externalaudit.googleapis.com/system_event\") AND NOT LOG_ID(\"cloudaudit.googleapis.com/access_transparency\") AND NOT LOG_ID(\"externalaudit.googleapis.com/access_transparency\")"
  name                   = "_Default"
  project                = "996044745806"
  unique_writer_identity = true
}
# terraform import google_logging_log_sink.a_default 996044745806###_Default
resource "google_service_account" "gateway_test_sa" {
  account_id   = "gateway-test-sa"
  display_name = "Gateway Service Account Test"
  project      = "mspr-epsi-coffee"
}
# terraform import google_service_account.gateway_test_sa projects/mspr-epsi-coffee/serviceAccounts/gateway-test-sa@mspr-epsi-coffee.iam.gserviceaccount.com
resource "google_service_account" "firebase_adminsdk_qck8r" {
  account_id   = "firebase-adminsdk-qck8r"
  description  = "Firebase Admin SDK Service Agent"
  display_name = "firebase-adminsdk"
  project      = "mspr-epsi-coffee"
}
# terraform import google_service_account.firebase_adminsdk_qck8r projects/mspr-epsi-coffee/serviceAccounts/firebase-adminsdk-qck8r@mspr-epsi-coffee.iam.gserviceaccount.com
resource "google_project_service" "bigquery_googleapis_com" {
  project = "996044745806"
  service = "bigquery.googleapis.com"
}
# terraform import google_project_service.bigquery_googleapis_com 996044745806/bigquery.googleapis.com
resource "google_project_service" "apigateway_googleapis_com" {
  project = "996044745806"
  service = "apigateway.googleapis.com"
}
# terraform import google_project_service.apigateway_googleapis_com 996044745806/apigateway.googleapis.com
resource "google_service_account" "github_action_sa" {
  account_id   = "github-action-sa"
  display_name = "GitHub Actions Service Account"
  project      = "mspr-epsi-coffee"
}
# terraform import google_service_account.github_action_sa projects/mspr-epsi-coffee/serviceAccounts/github-action-sa@mspr-epsi-coffee.iam.gserviceaccount.com
resource "google_project_service" "containerregistry_googleapis_com" {
  project = "996044745806"
  service = "containerregistry.googleapis.com"
}
# terraform import google_project_service.containerregistry_googleapis_com 996044745806/containerregistry.googleapis.com
resource "google_project_service" "firebaseappdistribution_googleapis_com" {
  project = "996044745806"
  service = "firebaseappdistribution.googleapis.com"
}
# terraform import google_project_service.firebaseappdistribution_googleapis_com 996044745806/firebaseappdistribution.googleapis.com
resource "google_project_service" "clouddebugger_googleapis_com" {
  project = "996044745806"
  service = "clouddebugger.googleapis.com"
}
# terraform import google_project_service.clouddebugger_googleapis_com 996044745806/clouddebugger.googleapis.com
resource "google_project_service" "apikeys_googleapis_com" {
  project = "996044745806"
  service = "apikeys.googleapis.com"
}
# terraform import google_project_service.apikeys_googleapis_com 996044745806/apikeys.googleapis.com
resource "google_project_service" "artifactregistry_googleapis_com" {
  project = "996044745806"
  service = "artifactregistry.googleapis.com"
}
# terraform import google_project_service.artifactregistry_googleapis_com 996044745806/artifactregistry.googleapis.com
resource "google_project_service" "datastore_googleapis_com" {
  project = "996044745806"
  service = "datastore.googleapis.com"
}
# terraform import google_project_service.datastore_googleapis_com 996044745806/datastore.googleapis.com
resource "google_project_service" "firestore_googleapis_com" {
  project = "996044745806"
  service = "firestore.googleapis.com"
}
# terraform import google_project_service.firestore_googleapis_com 996044745806/firestore.googleapis.com
resource "google_project_service" "cloudresourcemanager_googleapis_com" {
  project = "996044745806"
  service = "cloudresourcemanager.googleapis.com"
}
# terraform import google_project_service.cloudresourcemanager_googleapis_com 996044745806/cloudresourcemanager.googleapis.com
resource "google_project_service" "fcmregistrations_googleapis_com" {
  project = "996044745806"
  service = "fcmregistrations.googleapis.com"
}
# terraform import google_project_service.fcmregistrations_googleapis_com 996044745806/fcmregistrations.googleapis.com
resource "google_project_service" "bigquerystorage_googleapis_com" {
  project = "996044745806"
  service = "bigquerystorage.googleapis.com"
}
# terraform import google_project_service.bigquerystorage_googleapis_com 996044745806/bigquerystorage.googleapis.com
resource "google_project_service" "pubsub_googleapis_com" {
  project = "996044745806"
  service = "pubsub.googleapis.com"
}
# terraform import google_project_service.pubsub_googleapis_com 996044745806/pubsub.googleapis.com
resource "google_project_service" "iamcredentials_googleapis_com" {
  project = "996044745806"
  service = "iamcredentials.googleapis.com"
}
# terraform import google_project_service.iamcredentials_googleapis_com 996044745806/iamcredentials.googleapis.com
resource "google_project_service" "firebasedynamiclinks_googleapis_com" {
  project = "996044745806"
  service = "firebasedynamiclinks.googleapis.com"
}
# terraform import google_project_service.firebasedynamiclinks_googleapis_com 996044745806/firebasedynamiclinks.googleapis.com
resource "google_project_service" "firebaserules_googleapis_com" {
  project = "996044745806"
  service = "firebaserules.googleapis.com"
}
# terraform import google_project_service.firebaserules_googleapis_com 996044745806/firebaserules.googleapis.com
resource "google_project_service" "firebase_googleapis_com" {
  project = "996044745806"
  service = "firebase.googleapis.com"
}
# terraform import google_project_service.firebase_googleapis_com 996044745806/firebase.googleapis.com
resource "google_logging_log_sink" "a_required" {
  destination            = "logging.googleapis.com/projects/mspr-epsi-coffee/locations/global/buckets/_Required"
  filter                 = "LOG_ID(\"cloudaudit.googleapis.com/activity\") OR LOG_ID(\"externalaudit.googleapis.com/activity\") OR LOG_ID(\"cloudaudit.googleapis.com/system_event\") OR LOG_ID(\"externalaudit.googleapis.com/system_event\") OR LOG_ID(\"cloudaudit.googleapis.com/access_transparency\") OR LOG_ID(\"externalaudit.googleapis.com/access_transparency\")"
  name                   = "_Required"
  project                = "996044745806"
  unique_writer_identity = true
}
# terraform import google_logging_log_sink.a_required 996044745806###_Required
resource "google_project_service" "monitoring_googleapis_com" {
  project = "996044745806"
  service = "monitoring.googleapis.com"
}
# terraform import google_project_service.monitoring_googleapis_com 996044745806/monitoring.googleapis.com
resource "google_project_service" "fcm_googleapis_com" {
  project = "996044745806"
  service = "fcm.googleapis.com"
}
# terraform import google_project_service.fcm_googleapis_com 996044745806/fcm.googleapis.com
resource "google_project_service" "run_googleapis_com" {
  project = "996044745806"
  service = "run.googleapis.com"
}
# terraform import google_project_service.run_googleapis_com 996044745806/run.googleapis.com
resource "google_project_service" "identitytoolkit_googleapis_com" {
  project = "996044745806"
  service = "identitytoolkit.googleapis.com"
}
# terraform import google_project_service.identitytoolkit_googleapis_com 996044745806/identitytoolkit.googleapis.com
resource "google_project_service" "logging_googleapis_com" {
  project = "996044745806"
  service = "logging.googleapis.com"
}
# terraform import google_project_service.logging_googleapis_com 996044745806/logging.googleapis.com
resource "google_project_service" "bigquerymigration_googleapis_com" {
  project = "996044745806"
  service = "bigquerymigration.googleapis.com"
}
# terraform import google_project_service.bigquerymigration_googleapis_com 996044745806/bigquerymigration.googleapis.com
resource "google_project_service" "cloudasset_googleapis_com" {
  project = "996044745806"
  service = "cloudasset.googleapis.com"
}
# terraform import google_project_service.cloudasset_googleapis_com 996044745806/cloudasset.googleapis.com
resource "google_project_service" "gateway_coffee_0bs2w9fb1lddt_apigateway_mspr_epsi_coffee_cloud_goog" {
  project = "996044745806"
  service = "gateway-coffee-0bs2w9fb1lddt.apigateway.mspr-epsi-coffee.cloud.goog"
}
# terraform import google_project_service.gateway_coffee_0bs2w9fb1lddt_apigateway_mspr_epsi_coffee_cloud_goog 996044745806/gateway-coffee-0bs2w9fb1lddt.apigateway.mspr-epsi-coffee.cloud.goog
resource "google_project_service" "appengine_googleapis_com" {
  project = "996044745806"
  service = "appengine.googleapis.com"
}
# terraform import google_project_service.appengine_googleapis_com 996044745806/appengine.googleapis.com
resource "google_project_service" "servicecontrol_googleapis_com" {
  project = "996044745806"
  service = "servicecontrol.googleapis.com"
}
# terraform import google_project_service.servicecontrol_googleapis_com 996044745806/servicecontrol.googleapis.com
resource "google_project_service" "firebaseremoteconfig_googleapis_com" {
  project = "996044745806"
  service = "firebaseremoteconfig.googleapis.com"
}
# terraform import google_project_service.firebaseremoteconfig_googleapis_com 996044745806/firebaseremoteconfig.googleapis.com
resource "google_project_service" "cloudapis_googleapis_com" {
  project = "996044745806"
  service = "cloudapis.googleapis.com"
}
# terraform import google_project_service.cloudapis_googleapis_com 996044745806/cloudapis.googleapis.com
resource "google_project_service" "testing_googleapis_com" {
  project = "996044745806"
  service = "testing.googleapis.com"
}
# terraform import google_project_service.testing_googleapis_com 996044745806/testing.googleapis.com
resource "google_storage_bucket" "mspr_epsi_coffee_firestore_export" {
  force_destroy               = false
  location                    = "EUROPE-WEST1"
  name                        = "mspr-epsi-coffee-firestore-export"
  project                     = "mspr-epsi-coffee"
  public_access_prevention    = "enforced"
  storage_class               = "COLDLINE"
  uniform_bucket_level_access = true
}
# terraform import google_storage_bucket.mspr_epsi_coffee_firestore_export mspr-epsi-coffee-firestore-export
resource "google_project_service" "cloudtrace_googleapis_com" {
  project = "996044745806"
  service = "cloudtrace.googleapis.com"
}
# terraform import google_project_service.cloudtrace_googleapis_com 996044745806/cloudtrace.googleapis.com
resource "google_project_service" "storage_api_googleapis_com" {
  project = "996044745806"
  service = "storage-api.googleapis.com"
}
# terraform import google_project_service.storage_api_googleapis_com 996044745806/storage-api.googleapis.com
resource "google_project_service" "runtimeconfig_googleapis_com" {
  project = "996044745806"
  service = "runtimeconfig.googleapis.com"
}
# terraform import google_project_service.runtimeconfig_googleapis_com 996044745806/runtimeconfig.googleapis.com
resource "google_storage_bucket" "iosdeploy" {
  force_destroy               = false
  location                    = "EUROPE-WEST1"
  name                        = "iosdeploy"
  project                     = "mspr-epsi-coffee"
  public_access_prevention    = "enforced"
  storage_class               = "STANDARD"
  uniform_bucket_level_access = true
}
# terraform import google_storage_bucket.iosdeploy iosdeploy
resource "google_storage_bucket" "gcf_sources_996044745806_europe_west1" {
  cors {
    method = ["GET"]
    origin = ["https://*.cloud.google.com", "https://*.corp.google.com", "https://*.corp.google.com:*"]
  }
  force_destroy               = false
  location                    = "EUROPE-WEST1"
  name                        = "gcf-sources-996044745806-europe-west1"
  project                     = "mspr-epsi-coffee"
  public_access_prevention    = "inherited"
  storage_class               = "STANDARD"
  uniform_bucket_level_access = true
}
# terraform import google_storage_bucket.gcf_sources_996044745806_europe_west1 gcf-sources-996044745806-europe-west1
resource "google_storage_bucket" "eu_artifacts_mspr_epsi_coffee_appspot_com" {
  force_destroy            = false
  location                 = "EU"
  name                     = "eu.artifacts.mspr-epsi-coffee.appspot.com"
  project                  = "mspr-epsi-coffee"
  public_access_prevention = "inherited"
  storage_class            = "STANDARD"
}
# terraform import google_storage_bucket.eu_artifacts_mspr_epsi_coffee_appspot_com eu.artifacts.mspr-epsi-coffee.appspot.com
resource "google_project_service" "servicemanagement_googleapis_com" {
  project = "996044745806"
  service = "servicemanagement.googleapis.com"
}
# terraform import google_project_service.servicemanagement_googleapis_com 996044745806/servicemanagement.googleapis.com
resource "google_project_service" "serviceusage_googleapis_com" {
  project = "996044745806"
  service = "serviceusage.googleapis.com"
}
# terraform import google_project_service.serviceusage_googleapis_com 996044745806/serviceusage.googleapis.com
resource "google_project_service" "storage_component_googleapis_com" {
  project = "996044745806"
  service = "storage-component.googleapis.com"
}
# terraform import google_project_service.storage_component_googleapis_com 996044745806/storage-component.googleapis.com
resource "google_project_service" "firebaseinstallations_googleapis_com" {
  project = "996044745806"
  service = "firebaseinstallations.googleapis.com"
}
# terraform import google_project_service.firebaseinstallations_googleapis_com 996044745806/firebaseinstallations.googleapis.com
resource "google_project_service" "storage_googleapis_com" {
  project = "996044745806"
  service = "storage.googleapis.com"
}
# terraform import google_project_service.storage_googleapis_com 996044745806/storage.googleapis.com
resource "google_project_service" "firebasehosting_googleapis_com" {
  project = "996044745806"
  service = "firebasehosting.googleapis.com"
}
# terraform import google_project_service.firebasehosting_googleapis_com 996044745806/firebasehosting.googleapis.com
resource "google_project_service" "cloudbuild_googleapis_com" {
  project = "996044745806"
  service = "cloudbuild.googleapis.com"
}
# terraform import google_project_service.cloudbuild_googleapis_com 996044745806/cloudbuild.googleapis.com
resource "google_project_service" "compute_googleapis_com" {
  project = "996044745806"
  service = "compute.googleapis.com"
}
# terraform import google_project_service.compute_googleapis_com 996044745806/compute.googleapis.com
resource "google_project_service" "cloudfunctions_googleapis_com" {
  project = "996044745806"
  service = "cloudfunctions.googleapis.com"
}
# terraform import google_project_service.cloudfunctions_googleapis_com 996044745806/cloudfunctions.googleapis.com
resource "google_project_service" "oslogin_googleapis_com" {
  project = "996044745806"
  service = "oslogin.googleapis.com"
}
# terraform import google_project_service.oslogin_googleapis_com 996044745806/oslogin.googleapis.com
resource "google_project_service" "sql_component_googleapis_com" {
  project = "996044745806"
  service = "sql-component.googleapis.com"
}
# terraform import google_project_service.sql_component_googleapis_com 996044745806/sql-component.googleapis.com
resource "google_project_service" "securetoken_googleapis_com" {
  project = "996044745806"
  service = "securetoken.googleapis.com"
}
# terraform import google_project_service.securetoken_googleapis_com 996044745806/securetoken.googleapis.com
resource "google_project_service" "mobilecrashreporting_googleapis_com" {
  project = "996044745806"
  service = "mobilecrashreporting.googleapis.com"
}
# terraform import google_project_service.mobilecrashreporting_googleapis_com 996044745806/mobilecrashreporting.googleapis.com
