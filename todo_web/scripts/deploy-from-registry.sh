#!/bin/bash

set -euo pipefail

COMPOSE_FILE="docker-compose.yml"
PI_ENV_FILE="docker.env"
LOCAL_ENV_FILE="local.env"
SERVICE_NAME="todo_web"
REGISTRY="raspberrypi.local:5500"
BUILD_TIME=$(date +%Y%m%d_%H%M%S)
IMAGE_TAG="${BUILD_TIME}"

echo "=== Building and deploying $SERVICE_NAME ==="

# 1. Build the image locally
echo "--- Building image locally ---"
docker build -t "${SERVICE_NAME}:latest" .

# 2. Tag and push to registry
echo "--- Pushing to registry ---"
docker tag "${SERVICE_NAME}:latest" "${REGISTRY}/${SERVICE_NAME}:latest"
docker push "${REGISTRY}/${SERVICE_NAME}:latest"

# 3. Deploy using the registry image
echo "--- Deploying from registry ---"
ssh raspberrypi.local "bash -s" <  ./scripts/pi-deploy.sh

echo "=== Deployment finished successfully ==="
