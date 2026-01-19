# rust-boilerplate

A Rust web service template using Axum, PostgreSQL, and Docker.

## Features

- **Axum** - Modern async web framework
- **SQLx** - Compile-time checked SQL queries with PostgreSQL
- **Docker** - Multi-stage ARM64 build for Apple Silicon and Raspberry Pi
- **Layered Architecture** - Clean separation: web → domain → data → database
- **Repository Pattern** - Example CRUD implementation included
- **Justfile** - Convenient automation for common tasks

## Project Structure

```
image-stitch/
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library root
│   ├── db/               # Database infrastructure
│   │   ├── init.rs       # Pool initialization & migrations
│   │   ├── errors.rs     # Repository errors
│   │   └── types.rs      # Common DB types
│   ├── domain/           # Application core
│   │   ├── settings.rs   # Environment configuration
│   │   └── app_state.rs  # DI container
│   ├── web/              # HTTP layer
│   │   ├── router.rs     # Route definitions
│   │   └── handlers.rs   # Request handlers
│   └── data/             # Data access layer
│       └── example/      # Example entity (replace with your domain)
│           ├── models.rs
│           ├── repository.rs
│           └── services.rs
├── migrations/           # SQLx database migrations
├── Dockerfile            # ARM64 multi-stage build
├── docker-compose.yml    # Services orchestration
├── local.env             # Local development config
├── .env                  # Docker deployment config
└── Justfile              # Task automation
```

## Getting Started

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- Docker and Docker Compose
- Just task runner: `cargo install just`
- (Optional) sqlx-cli: `cargo install sqlx-cli`

### Local Development

1. **Copy the environment template:**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

2. **Start the database:**
   ```bash
   just db-up
   ```

3. **Run the application:**
   ```bash
   just run
   ```

4. **Test the health endpoint:**
   ```bash
   just health
   # or
   curl http://localhost:3000/health
   ```

### Docker Deployment

1. **Build and start all services:**
   ```bash
   just docker-up
   ```

2. **View logs:**
   ```bash
   just docker-logs
   ```

3. **Stop services:**
   ```bash
   just docker-down
   ```

## Configuration

Environment variables (see [.env.example](.env.example)):

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_HOST` | PostgreSQL host | `localhost` |
| `DATABASE_PORT` | PostgreSQL port | `5432` |
| `DATABASE_NAME` | Database name | `exampledb` |
| `DATABASE_USER` | Database user | `postgres` |
| `DATABASE_PASSWORD` | Database password | `postgres` |
| `SERVICE_PORT` | HTTP server port | `3000` (local), `8080` (docker) |
| `API_KEY` | API key for authentication | Required unless disabled |
| `API_KEY_DISABLED` | Disable API key auth (set to `true`) | `false` |
| `RUST_LOG` | Logging level | `project_name=debug` |

## Database Migrations

Migrations are automatically run on application startup. To manage migrations manually:

```bash
# Create a new migration
just migrate-create create_my_table

# Run pending migrations
just migrate-run

# Revert last migration
just migrate-revert
```

## Customizing the Template

### Replace Example Entity

1. Delete or rename [src/data/example/](src/data/example)
2. Create your domain module in `src/data/your_entity/`
3. Update migration in [migrations/](migrations)
4. Add routes in [src/web/router.rs](src/web/router.rs)
5. Update `.env` with your database name and credentials

### Add New Routes

In [src/web/router.rs](src/web/router.rs):

```rust
Router::new()
    .route("/health", get(super::health_check))
    .route("/api/your-endpoint", get(your_handler))
```

## Available Just Commands

```bash
just                 # List all commands
just run             # Run locally
just test            # Run tests
just check           # Check code
just fmt             # Format code
just lint            # Run clippy
just build           # Release build
just docker-up       # Start Docker services
just docker-down     # Stop Docker services
just docker-logs     # View logs
just db-up           # Start database only
just clean           # Clean everything
just health          # Test health endpoint
```

## Architecture

This template follows a layered architecture:

1. **Web Layer** (`src/web/`) - HTTP handlers, routing, middleware
2. **Domain Layer** (`src/domain/`) - Application state, configuration
3. **Data Layer** (`src/data/`) - Business logic, services, repositories
4. **Database Layer** (`src/db/`) - Connection pooling, migrations, errors

Each layer depends only on layers below it, ensuring clean separation of concerns.

## License

MIT
