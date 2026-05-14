use db_init::DbConfig;
use sqlx::{Pool, Postgres};
use tracing::debug;

pub mod errors;
pub mod metadata;

/// Initialize database with connection pool, schema creation, and migrations
pub async fn init_db(config: &DbConfig) -> Pool<Postgres> {
    debug!(
        "Connecting to database at {}:{}/{} (schema: {})",
        config.host, config.port, config.database, config.schema
    );

    let pool = db_init::init_pool(config)
        .await
        .expect("Failed to create PostgreSQL connection pool");

    // Verify schema
    db_init::verify_schema(&pool, &config.schema)
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
