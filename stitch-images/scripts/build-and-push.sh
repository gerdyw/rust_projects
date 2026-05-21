#!/bin/bash

set -euo pipefail

SERVICE_NAME="stitch-images"
REMOTE_REGISTRY="host.docker.internal:5500"
LOCAL_REGISTRY="localhost:5500"
REGISTRY="${REMOTE_REGISTRY}"
BUILD_TIME=$(date +%Y%m%d_%H%M%S)

while [[ $# -gt 0 ]]; do
    case "$1" in
        --local)
            REGISTRY="${LOCAL_REGISTRY}"
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [--local]"
            echo "  --local   Push to local registry (${LOCAL_REGISTRY}) instead of tunnel (${REMOTE_REGISTRY})"
            exit 0
            ;;
        *)
            echo "Unknown argument: $1" >&2
            echo "Usage: $0 [--local]" >&2
            exit 1
            ;;
    esac
done

echo "=== Building and pushing ${SERVICE_NAME} to registry ==="

# 1. Build the ARM64 image locally
REPO_ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
echo "--- Building ARM64 image from ${REPO_ROOT}/stitch-images/Dockerfile ---"
docker build \
    --platform linux/arm64 \
    -f "${REPO_ROOT}/stitch-images/Dockerfile" \
    -t "${SERVICE_NAME}:latest" \
    "${REPO_ROOT}"

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
