# Infra Deployment (VM Sparse Checkout)

This directory is the deployment entrypoint for `stitch-images` and `todo_web`.

## Scope

- Managed services: `stitch-images`, `todo_web`
- Shared infra: PostgreSQL, Redis, local Docker registry, telemetry stack

## Prerequisites

1. Create a local secrets file:
   - `cp .env.example .env`
   - replace placeholder secrets in `.env`
   - for database credentials, only `DATABASE_PASSWORD` is required
2. Ensure service images are available in the VM local registry:
   - `localhost:5500/stitch-images:latest`
   - `localhost:5500/todo_web:latest`

## First Boot Behavior

On first database startup (empty `postgres_data/data`), PostgreSQL runs `postgres_data/init.sql` via `/docker-entrypoint-initdb.d`.

That bootstrap script creates:

- extensions: `uuid-ossp`, `pgcrypto`

Service schemas are created by each service at startup via `SchemaMode::CreateIfMissing` in the shared `db_init` crate.

## Schema Provisioning Policy

- Do not add `CREATE SCHEMA` statements to service migrations.
- Do not add service-specific schema creation to `postgres_data/init.sql`.
- New services should set `DATABASE_SCHEMA` and use `SchemaMode::CreateIfMissing`.

This keeps onboarding a new service to configuration only, instead of requiring infra bootstrap and migration changes.

## Start Stack

From this directory:

```bash
docker compose up -d
```

## Verify

```bash
# Services
docker compose ps

# Logs
docker compose logs -f db stitch-images todo_web

# DB checks
docker compose exec db psql -U postgres -d services_db -c "\\dn"
docker compose exec db psql -U postgres -d services_db -c "\\dx"
```

## Update Deploy

1. Push new service images to VM local registry.
2. Pull new tags:

```bash
docker compose pull stitch-images todo_web
```

3. Restart services:

```bash
docker compose up -d stitch-images todo_web
```

## Notes

- `postgres_data/init.sql` runs only when Postgres initializes a new data directory.
- If you change bootstrap SQL later, clear `postgres_data/data` only if you intentionally want a fresh DB.