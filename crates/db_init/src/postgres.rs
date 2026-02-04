//! PostgreSQL-specific database initialization

use crate::error::DbInitError;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use sqlx::{Connection, Executor};
use sqlx::migrate::Migrator;
use std::time::Duration;

/// Schema creation behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaMode {
    /// Expect schema to exist (default)
    MustExist,
    /// Create schema if it doesn't exist
    CreateIfMissing,
}

impl Default for SchemaMode {
    fn default() -> Self {
        SchemaMode::MustExist
    }
}

/// PostgreSQL database configuration
#[derive(Debug, Clone)]
pub struct DbConfig {
    /// Database host
    pub host: String,
    /// Database port
    pub port: u16,
    /// Database name
    pub database: String,
    /// Database username
    pub username: String,
    /// Database password
    pub password: String,
    /// Schema name (defaults to "public")
    pub schema: String,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Timeout for acquiring a connection from the pool (in seconds)
    pub acquire_timeout_secs: u64,
    /// Schema creation mode
    pub schema_mode: SchemaMode,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            database: "postgres".to_string(),
            username: "postgres".to_string(),
            password: String::new(),
            schema: "public".to_string(),
            max_connections: 5,
            acquire_timeout_secs: 3,
            schema_mode: SchemaMode::MustExist,
        }
    }
}

/// Initialize a PostgreSQL connection pool with the given configuration
///
/// # Arguments
///
/// * `config` - Database configuration
///
/// # Returns
///
/// Returns a configured `PgPool` ready for use
///
/// # Errors
///
/// Returns `DbInitError` if connection or schema verification fails
///
/// # Example
///
/// ```rust,no_run
/// use db_init::{DbConfig, init_pool};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = DbConfig {
///     host: "localhost".to_string(),
///     database: "mydb".to_string(),
///     username: "user".to_string(),
///     password: "pass".to_string(),
///     ..Default::default()
/// };
///
/// let pool = init_pool(&config).await?;
/// # Ok(())
/// # }
/// ```
pub async fn init_pool(config: &DbConfig) -> Result<PgPool, DbInitError> {
    // Handle schema creation if needed
    if config.schema_mode == SchemaMode::CreateIfMissing && config.schema != "public" {
        create_schema_if_missing(config).await?;
    }

    // Build connection options
    let mut options = PgConnectOptions::new();
    options = options
        .host(&config.host)
        .port(config.port)
        .database(&config.database)
        .username(&config.username)
        .password(&config.password);

    // Set up search_path for the schema
    let schema = config.schema.clone();
    let schema_identifier = escape_identifier(&schema);

    // Create pool with configured options
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(Duration::from_secs(config.acquire_timeout_secs))
        .after_connect(move |conn, _meta| {
            let schema_identifier = schema_identifier.clone();
            Box::pin(async move {
                let set_schema_sql = format!("set search_path = {}", schema_identifier);
                sqlx::query(&set_schema_sql).execute(conn).await?;
                Ok(())
            })
        })
        .connect_with(options)
        .await?;

    Ok(pool)
}

/// Create schema if it doesn't exist (using a bootstrap connection)
async fn create_schema_if_missing(config: &DbConfig) -> Result<(), DbInitError> {
    let mut options = PgConnectOptions::new();
    options = options
        .host(&config.host)
        .port(config.port)
        .database(&config.database)
        .username(&config.username)
        .password(&config.password);

    let mut conn = sqlx::postgres::PgConnection::connect_with(&options).await?;
    
    let schema_identifier = escape_identifier(&config.schema);
    let create_schema_sql = format!("create schema if not exists {}", schema_identifier);
    
    conn.execute(sqlx::query(&create_schema_sql)).await?;
    
    Ok(())
}

/// Run database migrations using the provided migrator
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `migrator` - Reference to the embedded migrator
///
/// # Returns
///
/// Returns `Ok(())` on success
///
/// # Errors
///
/// Returns `DbInitError` if migrations fail
///
/// # Example
///
/// ```rust,no_run
/// use db_init::{DbConfig, init_pool, run_migrations};
/// use sqlx::migrate::Migrator;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let pool = init_pool(&DbConfig::default()).await?;
/// 
/// // From caller's crate with embedded migrations
/// static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
/// run_migrations(&pool, &MIGRATOR).await?;
/// # Ok(())
/// # }
/// ```
pub async fn run_migrations(pool: &PgPool, migrator: &Migrator) -> Result<(), DbInitError> {
    migrator.run(pool).await?;
    Ok(())
}

/// Verify that the connected schema matches the expected schema
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `expected_schema` - Expected schema name
///
/// # Returns
///
/// Returns `Ok(())` if schema matches
///
/// # Errors
///
/// Returns `DbInitError::WrongSchema` if connected to wrong schema
/// Returns `DbInitError::SchemaMissing` if expected schema doesn't exist
pub async fn verify_schema(pool: &PgPool, expected_schema: &str) -> Result<(), DbInitError> {
    let search_path: String = sqlx::query_scalar("show search_path")
        .fetch_one(pool)
        .await?;
    
    let current_schema: Option<String> = sqlx::query_scalar("select current_schema()")
        .fetch_one(pool)
        .await?;

    let active_schema = current_schema.unwrap_or_else(|| "public".to_string());

    if active_schema != expected_schema {
        // Check if the expected schema exists
        let schema_exists: bool =
            sqlx::query_scalar("select exists(select 1 from pg_namespace where nspname = $1)")
                .bind(expected_schema)
                .fetch_one(pool)
                .await?;

        if !schema_exists {
            return Err(DbInitError::SchemaMissing {
                expected: expected_schema.to_string(),
                search_path,
            });
        }

        return Err(DbInitError::WrongSchema {
            expected: expected_schema.to_string(),
            actual: active_schema,
            search_path,
        });
    }

    Ok(())
}

/// Escape a PostgreSQL identifier (schema, table, column name)
fn escape_identifier(identifier: &str) -> String {
    let escaped = identifier.replace('"', "\"\"");
    format!("\"{}\"", escaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_identifier() {
        assert_eq!(escape_identifier("public"), "\"public\"");
        assert_eq!(escape_identifier("my_schema"), "\"my_schema\"");
        assert_eq!(escape_identifier("schema\"with\"quotes"), "\"schema\"\"with\"\"quotes\"");
    }

    #[test]
    fn test_default_config() {
        let config = DbConfig::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5432);
        assert_eq!(config.schema, "public");
        assert_eq!(config.max_connections, 5);
        assert_eq!(config.acquire_timeout_secs, 3);
        assert_eq!(config.schema_mode, SchemaMode::MustExist);
    }
}
