#!/bin/bash

set -euo pipefail

SERVICE_NAME="image-stitch"
REGISTRY="raspberrypi.local:5000"
BUILD_TIME=$(date +%Y%m%d_%H%M%S)

echo "=== Building and pushing $SERVICE_NAME to registry ==="

# 1. Build the ARM64 image locally
echo "--- Building ARM64 image ---"
docker build --platform linux/arm64 -t "${SERVICE_NAME}:latest" .

# 2. Tag for registry
echo "--- Tagging image for registry ---"
docker tag "${SERVICE_NAME}:latest" "${REGISTRY}/${SERVICE_NAME}:latest"
docker tag "${SERVICE_NAME}:latest" "${REGISTRY}/${SERVICE_NAME}:${BUILD_TIME}"

# 3. Push to registry
echo "--- Pushing to ${REGISTRY} ---"
docker push "${REGISTRY}/${SERVICE_NAME}:latest"
docker push "${REGISTRY}/${SERVICE_NAME}:${BUILD_TIME}"

echo "=== Push completed successfully ==="
echo "Latest tag: ${REGISTRY}/${SERVICE_NAME}:latest"
echo "Timestamped tag: ${REGISTRY}/${SERVICE_NAME}:${BUILD_TIME}"
