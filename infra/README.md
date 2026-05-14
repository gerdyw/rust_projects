# Infra deployment bundle

This directory is self-contained for sparse checkout deployments.

## Sparse checkout only infra

```bash
git clone --filter=blob:none --sparse https://github.com/gerdyw/rust_projects.git
cd rust_projects
git sparse-checkout set infra
```

## Deploy

```bash
cd infra
cp .env.example .env
docker compose up -d
```

Both Rust services run their own startup migrations. `todo_web` creates its schema if missing, and `image-stitch` is configured to do the same.
