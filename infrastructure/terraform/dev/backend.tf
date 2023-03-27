terraform {
  backend "gcs" {
    bucket      = "terraform-state-5f96fc31-b260-40fc-8244-d3f208c41722"
    prefix      = "terraform/state/dev"
    credentials = "key.json"
  }
}