#!/bin/bash
set -e

# Change these
IMAGE_NAME=ghcr.io/gerdyw/rust-todo-app:latest
DATA_DIR=$(pwd)/todo_data

# 1. Create and use a Buildx builder
docker buildx create --use --name rustbuilder || true

# 2. Build and push ARM64 image to GHCR
docker buildx build \
    --platform linux/arm64 \
    -t $IMAGE_NAME \
    # --push \
    .

# 3. Optional: test locally by pulling and running
docker run -it --rm \
    -p 8080:8080 \
    -e DATABASE_URL=sqlite:///app/data/todo.db \
    -v $DATA_DIR:/app/data \
    $IMAGE_NAME
