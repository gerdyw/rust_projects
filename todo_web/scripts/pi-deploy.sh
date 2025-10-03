SERVICE_NAME="todo_web"
PI_ENV_FILE="docker.env"
GIT_BRANCH="main"

cd ~/code/axum_learnings/todo_web
set -euo pipefail

echo "=== Deploying $SERVICE_NAME on Raspberry Pi ==="

# 1. Pull latest code
echo "--- Pulling latest code from Git ---"
git fetch origin "$GIT_BRANCH"
git reset --hard "origin/$GIT_BRANCH"

docker compose --env-file ${PI_ENV_FILE} pull ${SERVICE_NAME} &&
docker compose --env-file ${PI_ENV_FILE} --profile prod up -d --force-recreate ${SERVICE_NAME}