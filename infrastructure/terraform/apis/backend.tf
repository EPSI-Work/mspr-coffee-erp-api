terraform {
  backend "gcs" {
    bucket = "terraform_state_a218357f-f46a-4636-b7ab-7db30f28b628"
    prefix = "terraform/state/dev/apis"
  }
}

# export GOOGLE_APPLICATION_CREDENTIALS=key.json