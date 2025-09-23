#!/bin/bash
set -e

# Configuration
IMAGE_NAME=todo_web
PORT=8080
DATA_DIR=$(pwd)/todo_data
STATIC_DIR=$(pwd)/assets

# Ensure directories exist
mkdir -p "$DATA_DIR"
mkdir -p "$STATIC_DIR"

echo "=== Building ARM64 binary natively ==="
# Add target if missing
rustup target add aarch64-unknown-linux-gnu || true

# Build the release binary for ARM64 Linux
docker run --rm -it --platform linux/arm64 \
  -v $(pwd):/usr/src/app -w /usr/src/app \
  rust:1.89 bash -c "rustup target add aarch64-unknown-linux-gnu && apt-get update && apt-get install -y build-essential && cargo build --release --target aarch64-unknown-linux-gnu"

echo "=== Building Docker image ==="
docker build -t $IMAGE_NAME .

echo "=== Running container locally ==="
docker run --rm -it \
    -p $PORT:$PORT \
    -e DATABASE_URL=sqlite:///app/data/todo.db \
    -e PORT=$PORT \
    -v "$DATA_DIR":/app/data \
    -v "$STATIC_DIR":/app/assets \
    $IMAGE_NAME
