#!/bin/bash

cd ~/code/axum_learnings/todo_web
set -euo pipefail

COMPOSE_FILE="docker-compose.yaml"
GIT_BRANCH="main"
SERVICE_NAME="todo_web"

echo "=== Deploying $SERVICE_NAME via docker compose ==="

# 1. Pull latest code
echo "--- Pulling latest code from Git ---"
git fetch origin "$GIT_BRANCH"
git reset --hard "origin/$GIT_BRANCH"

# 2. Build the image and bring up containers
echo "--- Building images and updating containers ---"
docker compose -f "$COMPOSE_FILE" build "$SERVICE_NAME"
docker compose -f "$COMPOSE_FILE" up -d --no-deps "$SERVICE_NAME"

# Optional: remove old dangling images
echo "--- Cleaning up old images ---"
docker image prune -f

echo "=== Deployment finished successfully ==="
