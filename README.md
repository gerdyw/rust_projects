# Rust Projects

A collection of Rust projects ranging from web services to systems programming.

## Projects

### [stitch-images](./stitch-images)
An image-stitching web service. Accepts multiple images via multipart upload, stitches them into a single composite, and serves the result through a web interface.

**Stack:** Rocket, SQLx, PostgreSQL, Maud (server-side HTML), Docker (ARM64)

---

### [todo_web](./todo_web)
A full-stack todo application with user authentication, session management, and Redis-backed sessions. Deployed to a Raspberry Pi via a self-hosted Docker registry.

**Stack:** Axum, SQLx, PostgreSQL, Redis, Maud, Docker, OpenTelemetry

---

### [lc3](./lc3)
An LC-3 virtual machine implemented from scratch in Rust. Executes assembled `.obj` files and implements the full LC-3 instruction set including trap vectors and condition codes.

**Stack:** Pure Rust, no dependencies

---

### [infra](./infra)
Docker Compose stack for self-hosting on a Raspberry Pi. Bundles a shared database, cache, private Docker registry, and a full observability suite behind a Cloudflare tunnel.

**Stack:** PostgreSQL, Redis, Docker Registry, Grafana LGTM (Prometheus, Loki, Tempo), Cloudflare Tunnel

---

### [template-rust-web](./template-rust-web)
A reusable Axum web service template with layered architecture (web → domain → data → database), compile-time SQL, and a multi-stage ARM64 Dockerfile.

**Stack:** Axum, SQLx, PostgreSQL, Docker

---

### [crates/db_init](./crates/db_init)
A shared library crate for PostgreSQL connection pool initialisation and migration running, used across services in this repo.

---

## Smaller projects

| Project | Description |
|---|---|
| [image-stitch](./image-stitch) | Earlier Axum-based iteration of the image stitching service |
| [use_database](./use_database) | Axum + SQLx database integration example |
| [state_counter](./state_counter) | Shared state example with Axum |
| [hello_world](./hello_world) / [hello_person](./hello_person) | Initial Rust exercises |
