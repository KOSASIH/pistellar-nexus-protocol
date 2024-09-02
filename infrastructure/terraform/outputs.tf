output "deployment_name" {
  value = kubernetes_deployment.pistellar_nexus_core.metadata[0].name
}

output "service_name" {
  value = kubernetes_service.pistellar_nexus_core.metadata[0].name
}

output "ingress_host" {
  value = kubernetes_ingress.pistellar_nexus_core.spec[0].rule[0].host
}
