use crate::db::errors::DbInitError;
use crate::domain::DatabaseSettings;
use sqlx::migrate::{MigrateError, Migrator};
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use std::time::Duration;

/// Initialize a PostgreSQL connection pool
pub async fn init_pool(database: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    let mut options = PgConnectOptions::new();
    options = options
        .host(&database.host)
        .port(database.port)
        .database(&database.name)
        .username(&database.user)
        .password(&database.password);

    let schema = database.schema.clone();
    let schema_identifier = escape_identifier(&schema);

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .after_connect(move |conn, _meta| {
            let schema_identifier = schema_identifier.clone();
            Box::pin(async move {
                let set_schema_sql = format!("set search_path = {}", schema_identifier);
                sqlx::query(&set_schema_sql).execute(conn).await?;
                Ok(())
            })
        })
        .connect_with(options)
        .await
}

/// Run database migrations
/// Migrations are embedded in the binary at compile time
pub async fn run_migrations(pool: &PgPool) -> Result<(), MigrateError> {
    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    MIGRATOR.run(pool).await
}

/// Ensure the active schema matches the expected value so migrations run in the right place
pub async fn verify_schema(pool: &PgPool, expected_schema: &str) -> Result<(), DbInitError> {
    let search_path: String = sqlx::query_scalar("show search_path")
        .fetch_one(pool)
        .await?;
    let current_schema: Option<String> = sqlx::query_scalar("select current_schema()")
        .fetch_one(pool)
        .await?;

    let active_schema = current_schema.unwrap_or_else(|| "public".to_string());

    if active_schema != expected_schema {
        return Err(DbInitError::WrongSchema {
            expected: expected_schema.to_string(),
            actual: active_schema,
            search_path,
        });
    }

    Ok(())
}

fn escape_identifier(identifier: &str) -> String {
    let escaped = identifier.replace('"', "\"\"");
    format!("\"{}\"", escaped)
}
