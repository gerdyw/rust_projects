use crate::domain::DatabaseSettings;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPool;

// Re-export error type from shared crate for backward compatibility
pub use db_init::DbInitError;

/// Initialize a PostgreSQL connection pool
/// 
/// This is a thin wrapper around the shared db_init crate that maps
/// our application's DatabaseSettings to the shared DbConfig.
pub async fn init_pool(database: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    let config = db_init::DbConfig {
        host: database.host.clone(),
        port: database.port,
        database: database.name.clone(),
        username: database.user.clone(),
        password: database.password.clone(),
        schema: database.schema.clone(),
        max_connections: 5,
        acquire_timeout_secs: 3,
        schema_mode: db_init::SchemaMode::MustExist,
    };

    db_init::init_pool(&config)
        .await
        .map_err(|e| match e {
            db_init::DbInitError::Sqlx(err) => err,
            other => sqlx::Error::Protocol(format!("Database init error: {}", other)),
        })
}

/// Run database migrations
/// Migrations are embedded in the binary at compile time
pub async fn run_migrations(pool: &PgPool) -> Result<(), db_init::DbInitError> {
    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    db_init::run_migrations(pool, &MIGRATOR).await
}

/// Ensure the active schema matches the expected value so migrations run in the right place
pub async fn verify_schema(pool: &PgPool, expected_schema: &str) -> Result<(), DbInitError> {
    db_init::verify_schema(pool, expected_schema).await
}
