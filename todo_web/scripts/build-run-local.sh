#!/bin/bash

set -euo pipefail

COMPOSE_FILE="docker-compose.yml"
PI_ENV_FILE="docker.env"
LOCAL_ENV_FILE="local.env"
SERVICE_NAME="todo_web"
REGISTRY="raspberrypi.local:5000"
BUILD_TIME=$(date +%Y%m%d_%H%M%S)
IMAGE_TAG="${BUILD_TIME}"

echo "=== Building and deploying $SERVICE_NAME ==="

# 1. Build the image locally
echo "--- Building image locally ---"
docker build -t "${SERVICE_NAME}:latest" .

# 2. Tag and push to registry
echo "--- Pushing to registry ---"
docker tag "${SERVICE_NAME}:latest" "${REGISTRY}/${SERVICE_NAME}:${IMAGE_TAG}"
docker tag "${SERVICE_NAME}:latest" "${REGISTRY}/${SERVICE_NAME}:latest"
docker push "${REGISTRY}/${SERVICE_NAME}:${IMAGE_TAG}"
docker push "${REGISTRY}/${SERVICE_NAME}:latest"

# 3. Deploy the service using docker compose
echo "--- Deploying service using docker compose ---"
docker compose --env-file ${LOCAL_ENV_FILE} --env-file ${PI_ENV_FILE} pull ${SERVICE_NAME}
docker compose --env-file ${LOCAL_ENV_FILE} --env-file ${PI_ENV_FILE} up -d --force-recreate ${SERVICE_NAME}
docker compose --env-file ${LOCAL_ENV_FILE} --env-file ${PI_ENV_FILE} logs -f