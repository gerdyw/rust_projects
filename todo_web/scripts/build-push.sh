#!/bin/bash

set -euo pipefail

COMPOSE_FILE="docker-compose.yml"
PI_ENV_FILE="docker.env"
LOCAL_ENV_FILE="local.env"
SERVICE_NAME="todo_web"
REMOTE_REGISTRY="raspberrypi.local:5500"
LOCAL_REGISTRY="localhost:5500"
REGISTRY="${REMOTE_REGISTRY}"
BUILD_TIME=$(date +%Y%m%d_%H%M%S)
IMAGE_TAG="${BUILD_TIME}"

while [[ $# -gt 0 ]]; do
	case "$1" in
		--local)
			REGISTRY="${LOCAL_REGISTRY}"
			shift
			;;
		-h|--help)
			echo "Usage: $0 [--local]"
			echo "  --local   Push to ${LOCAL_REGISTRY} instead of ${REMOTE_REGISTRY}"
			exit 0
			;;
		*)
			echo "Unknown argument: $1" >&2
			echo "Usage: $0 [--local]" >&2
			exit 1
			;;
	esac
done

echo "=== Building and deploying $SERVICE_NAME ==="

# 1. Build the image locally
echo "--- Building image locally ---"
REPO_ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
docker build -f "${REPO_ROOT}/todo_web/Dockerfile" -t "${SERVICE_NAME}:latest" "${REPO_ROOT}"

# 2. Tag and push to registry
echo "--- Pushing to registry ---"
docker tag "${SERVICE_NAME}:latest" "${REGISTRY}/${SERVICE_NAME}:latest"
docker push "${REGISTRY}/${SERVICE_NAME}:latest"