use db_init::{DbConfig, DbInitError};
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPool;

/// Initialize a PostgreSQL connection pool
pub async fn init_pool(config: &DbConfig) -> Result<PgPool, sqlx::Error> {
    db_init::init_pool(config).await.map_err(|e| match e {
        DbInitError::Sqlx(err) => err,
        other => sqlx::Error::Protocol(format!("Database init error: {}", other)),
    })
}

/// Run database migrations
/// Migrations are embedded in the binary at compile time
pub async fn run_migrations(pool: &PgPool) -> Result<(), DbInitError> {
    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    db_init::run_migrations(pool, &MIGRATOR).await
}

/// Ensure the active schema matches the expected value so migrations run in the right place
pub async fn verify_schema(pool: &PgPool, expected_schema: &str) -> Result<(), DbInitError> {
    db_init::verify_schema(pool, expected_schema).await
}
