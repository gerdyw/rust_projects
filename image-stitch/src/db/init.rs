use sqlx::migrate::{MigrateError, Migrator};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Initialize a PostgreSQL connection pool
pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}

/// Run database migrations
/// Migrations are embedded in the binary at compile time
pub async fn run_migrations(pool: &PgPool) -> Result<(), MigrateError> {
    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    MIGRATOR.run(pool).await
}
