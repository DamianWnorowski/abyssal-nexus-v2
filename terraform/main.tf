```hcl
terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
  }
}

provider "google" {
  project = var.project_id
  region  = "us-central1"
}

# Cloud SQL (Matrix X persistence)
resource "google_sql_database_instance" "matrix_x" {
  name             = "matrix-x"
  database_version = "MYSQL_8_0"
  region           = "us-central1"

  settings {
    tier = "db-f1-micro"
    ip_configuration {
      ipv4_enabled    = true
      authorized_networks {
        name  = "local-dev"
        value = "0.0.0.0/0" # Tighten for prod
      }
    }
  }
}

resource "google_sql_database" "matrix_x_db" {
  name     = "matrix_x"
  instance = google_sql_database_instance.matrix_x.name
}

# Artifact Registry
resource "google_artifact_registry_repository" "abyssal_nexus" {
  location      = "us-central1"
  repository_id = "abyssal-nexus"
  format        = "DOCKER"
}

# Cloud Run
resource "google_cloud_run_service" "abyssal_nexus" {
  name     = "abyssal-nexus"
  location = "us-central1"

  template {
    spec {
      containers {
        image = "${google_artifact_registry_repository.abyssal_nexus.repository_url}/abyssal-nexus:latest"
        env {
          name  = "DATABASE_URL"
          value = "mysql://${google_sql_database_instance.matrix_x.public_ip_address}:3306/matrix_x"
        }
        ports {
          container_port = 3000
        }
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }

  depends_on = [google_sql_database_instance.matrix_x]
}

# IAM
resource "google_cloud_run_service_iam_member" "public" {
  service  = google_cloud_run_service.abyssal_nexus.name
  location = google_cloud_run_service.abyssal_nexus.location
  role     = "roles/run.invoker"
  member   = "allUsers"
}

variable "project_id" {
  description = "GCP Project ID"
  type        = string
}
```

**Deploy:**
```bash
terraform init
terraform apply -var="project_id=your-project-id"
```

**Output:** `abyssal-nexus-XXXX.a.run.app` → `/api/ai-chat` live