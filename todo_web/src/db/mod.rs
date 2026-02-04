use sqlx::{Pool, Postgres};
use tracing::debug;

use crate::domain::settings::DatabaseSettings;

// Re-export error types from shared crate for backward compatibility
pub use db_init::DbInitError;

pub mod errors;
pub mod metadata;

/// Initialize database with connection pool, schema creation, and migrations
///
/// This is a wrapper around the shared db_init crate that maintains backward
/// compatibility with todo_web's initialization behavior.
pub async fn init_db(db_settings: &DatabaseSettings) -> Pool<Postgres> {
    debug!(
        "Connecting to database at {}:{}/{} (schema: {})",
        db_settings.host, db_settings.port, db_settings.db_name, db_settings.schema
    );

    // Configure pool with schema auto-creation
    let config = db_init::DbConfig {
        host: db_settings.host.clone(),
        port: db_settings.port,
        database: db_settings.db_name.clone(),
        username: db_settings.username.clone(),
        password: db_settings.password.clone(),
        schema: db_settings.schema.clone(),
        max_connections: 5,
        acquire_timeout_secs: 3,
        schema_mode: db_init::SchemaMode::CreateIfMissing,
    };

    let pool = db_init::init_pool(&config)
        .await
        .expect("Failed to create PostgreSQL connection pool");

    // Verify schema
    db_init::verify_schema(&pool, &db_settings.schema)
        .await
        .expect("Connected to unexpected schema");

    // Run migrations
    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();
    debug!("Found {} migrations", MIGRATOR.migrations.len());

    MIGRATOR
        .iter()
        .for_each(|m| debug!("Migration available: {}", m.description));

    db_init::run_migrations(&pool, &MIGRATOR)
        .await
        .expect("Failed to run migrations");

    debug!("Database migrations completed");

    pool
}
