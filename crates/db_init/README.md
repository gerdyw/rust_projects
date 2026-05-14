# db_init

A reusable database initialization library for Rust projects using sqlx.

## Features

- **PostgreSQL Support**: Initialize PostgreSQL connection pools with schema support
- **Schema Management**: Verify schemas or create them automatically
- **Embedded Migrations**: Run sqlx migrations from your application
- **Configurable Pooling**: Control max connections, timeouts, and other pool settings
- **Environment Loading**: Standardized loading of database configuration from environment variables
- **Type-Safe**: Leverages sqlx's compile-time query checking

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
db_init = { path = "../crates/db_init", features = ["postgres"] }
sqlx = { version = "0.8.6", features = ["migrate", "postgres", ...] }
```

### Loading from Environment Variables

The simplest way to configure the database connection is to load from environment variables:

```rust
use db_init::{DbConfig, SchemaMode, init_pool, run_migrations};
use sqlx::migrate::Migrator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let config = DbConfig::from_env(SchemaMode::CreateIfMissing)?;

    // Initialize connection pool
    let pool = init_pool(&config).await?;

    // Run migrations (from your application's migrations directory)
    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    run_migrations(&pool, &MIGRATOR).await?;

    Ok(())
}
```

#### Required Environment Variables

- `DATABASE_HOST`: Database host (required)
- `DATABASE_NAME`: Database name (required)
- `DATABASE_USER`: Database username (required)
- `DATABASE_PASSWORD`: Database password (required)

#### Optional Environment Variables

- `DATABASE_PORT`: Database port (default: 5432)
- `DATABASE_SCHEMA`: Schema name (default: "public")
- `DATABASE_MAX_CONNECTIONS`: Maximum pool connections (default: 5)
- `DATABASE_ACQUIRE_TIMEOUT_SECS`: Connection acquire timeout in seconds (default: 3)

### Manual Configuration Example

```rust
use db_init::{DbConfig, SchemaMode, init_pool, run_migrations};
use sqlx::migrate::Migrator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure database connection
    let config = DbConfig {
        host: "localhost".to_string(),
        port: 5432,
        database: "mydb".to_string(),
        username: "user".to_string(),
        password: "pass".to_string(),
        schema: "my_schema".to_string(),
        max_connections: 5,
        acquire_timeout_secs: 3,
        schema_mode: SchemaMode::CreateIfMissing,
    };

    // Initialize connection pool
    let pool = init_pool(&config).await?;

    // Run migrations (from your application's migrations directory)
    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    run_migrations(&pool, &MIGRATOR).await?;

    Ok(())
}
```

```rust
use db_init::{DbConfig, SchemaMode, init_pool, run_migrations};
use sqlx::migrate::Migrator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Manually configure database connection
    let config = DbConfig {
        host: "localhost".to_string(),
        port: 5432,
        database: "mydb".to_string(),
        username: "user".to_string(),
        password: "pass".to_string(),
        schema: "my_schema".to_string(),
        max_connections: 5,
        acquire_timeout_secs: 3,
        schema_mode: SchemaMode::CreateIfMissing,
    };

    // Initialize connection pool
    let pool = init_pool(&config).await?;

    // Run migrations (from your application's migrations directory)
    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    run_migrations(&pool, &MIGRATOR).await?;

    Ok(())
}
```

### Schema Modes

The library supports two schema modes:

- `SchemaMode::MustExist` (default): Expects the schema to already exist
- `SchemaMode::CreateIfMissing`: Creates the schema if it doesn't exist

### Environment-Based Configuration (Legacy)

You can also build configuration from environment variables manually:

```rust
use db_init::DbConfig;

fn config_from_env() -> DbConfig {
    DbConfig {
        host: std::env::var("DATABASE_HOST").unwrap(),
        port: std::env::var("DATABASE_PORT").unwrap().parse().unwrap(),
        database: std::env::var("DATABASE_NAME").unwrap(),
        username: std::env::var("DATABASE_USER").unwrap(),
        password: std::env::var("DATABASE_PASSWORD").unwrap(),
        schema: std::env::var("DATABASE_SCHEMA").unwrap_or_else(|_| "public".to_string()),
        ..Default::default()
    }
}
```

## API Overview

### Types

- `DbConfig`: Configuration struct for database connection
- `SchemaMode`: Enum controlling schema creation behavior
- `DbInitError`: Error type for initialization failures

### Functions

- `init_pool(config: &DbConfig) -> Result<PgPool, DbInitError>`: Initialize a connection pool
- `run_migrations(pool: &PgPool, migrator: &Migrator) -> Result<(), DbInitError>`: Run migrations
- `verify_schema(pool: &PgPool, expected_schema: &str) -> Result<(), DbInitError>`: Verify connected schema

## Feature Flags

- `postgres`: Enable PostgreSQL support (currently the only supported backend)
- `mysql`: (Reserved for future MySQL support)
- `sqlite`: (Reserved for future SQLite support)

## Migration Guide

### From Individual Implementations

If you're migrating from a custom database initialization implementation:

1. Add `db_init` to your dependencies with the `postgres` feature
2. Replace your config struct with `DbConfig` or map your existing config to it
3. Replace your pool initialization with `init_pool()`
4. Keep your existing `sqlx::migrate!()` but pass the migrator to `run_migrations()`

Example migration:

```rust
// Before
let pool = init_pool(&settings.database).await?;
run_migrations(&pool).await?;

// After  
use db_init::{DbConfig, SchemaMode};

let config = DbConfig {
    host: settings.database.host,
    port: settings.database.port,
    database: settings.database.name,
    username: settings.database.user,
    password: settings.database.password,
    schema: settings.database.schema,
    schema_mode: SchemaMode::MustExist,
    ..Default::default()
};

let pool = db_init::init_pool(&config).await?;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
db_init::run_migrations(&pool, &MIGRATOR).await?;
```

## License

Same as parent project.
