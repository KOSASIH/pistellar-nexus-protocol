#!/bin/bash

# Set environment variables
export DATABASE_URL=$(terraform output -json | jq -r '.database_url.value')
export STELLAR_NETWORK=$(terraform output -json | jq -r '.stellar_network.value')
export PI_NETWORK=$(terraform output -json | jq -r '.pi_network.value')

# Create Kubernetes namespace
kubectl create namespace pistellar-nexus-core || true

# Apply Terraform configuration
terraform apply -auto-approve

# Wait for deployment to be ready
kubectl rollout status deployment/pistellar-nexus-core -n pistellar-nexus-core --timeout=5m

# Create ingress resource
kubectl apply -f infrastructure/terraform/kubernetes_ingress.yaml -n pistellar-nexus-core

echo "PiStellar Nexus Core application setup complete!"
