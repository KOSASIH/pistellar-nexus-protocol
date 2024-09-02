provider "kubernetes" {
  config_path = "~/.kube/config"
}

resource "kubernetes_deployment" "pistellar_nexus_core" {
  metadata {
    name = "pistellar-nexus-core"
  }

  spec {
    replicas = 3

    selector {
      match_labels = {
        app = "pistellar-nexus-core"
      }
    }

    template {
      metadata {
        labels = {
          app = "pistellar-nexus-core"
        }
      }

      spec {
        container {
          image = "pistellar/nexus-core:latest"
          name  = "pistellar-nexus-core"

          port {
            container_port = 8080
          }

          env {
            name  = "DATABASE_URL"
            value = var.database_url
          }

          env {
            name  = "STELLAR_NETWORK"
            value = var.stellar_network
          }

          env {
            name  = "PI_NETWORK"
            value = var.pi_network
          }
        }
      }
    }
  }
}

resource "kubernetes_service" "pistellar_nexus_core" {
  metadata {
    name = "pistellar-nexus-core"
  }

  spec {
    selector = {
      app = "pistellar-nexus-core"
    }

    port {
      name = "http"
      port = 80
      target_port = 8080
    }

    type = "LoadBalancer"
  }
}

resource "kubernetes_ingress" "pistellar_nexus_core" {
  metadata {
    name = "pistellar-nexus-core"
  }

  spec {
    rule {
      host = "pistellar-nexus-core.example.com"

      http {
        path {
          path = "/"
          backend {
            service_name = "pistellar-nexus-core"
            service_port = 80
          }
        }
      }
    }
  }
}
