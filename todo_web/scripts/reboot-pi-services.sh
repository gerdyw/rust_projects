COMPOSE_FILE="docker-compose.yml"
ENV_FILE="docker.env"
GIT_BRANCH="main"

cd ~/code/axum_learnings/todo_web
set -euo pipefail

# 1. Pull latest code
echo "--- Pulling latest code from Git ---"
git fetch origin "$GIT_BRANCH"
git reset --hard "origin/$GIT_BRANCH"

# 2. Restart services
echo "--- Restarting services ---"
docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" --profile prod up -d
