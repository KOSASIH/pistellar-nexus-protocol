#!/bin/bash

# Delete ingress resource
kubectl delete -f infrastructure/terraform/kubernetes_ingress.yaml -n pistellar-nexus-core

# Destroy Terraform resources
terraform destroy -auto-approve

# Delete Kubernetes namespace
kubectl delete namespace pistellar-nexus-core

echo "PiStellar Nexus Core application teardown complete!"
