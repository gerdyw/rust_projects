use sqlx::{
    Connection, Pool, Postgres,
    postgres::{PgConnectOptions, PgConnection, PgPoolOptions},
};
use tracing::{debug, error};

use crate::db::errors::DbInitError;
use crate::domain::settings::DatabaseSettings;
pub mod errors;
pub mod metadata;

pub async fn init_db(db_settings: &DatabaseSettings) -> Pool<Postgres> {
    debug!(
        "Connecting to database at {}:{}/{} (schema: {})",
        db_settings.host, db_settings.port, db_settings.db_name, db_settings.schema
    );

    let mut options = PgConnectOptions::new();
    options = options
        .host(&db_settings.host)
        .port(db_settings.port)
        .database(&db_settings.db_name)
        .username(&db_settings.username)
        .password(&db_settings.password);

    let schema = db_settings.schema.clone();
    let schema_identifier = escape_identifier(&schema);

    let mut bootstrap_conn = PgConnection::connect_with(&options)
        .await
        .expect("Failed to connect for schema bootstrap");
    let create_schema_sql = format!("create schema if not exists {}", schema_identifier);
    sqlx::query(&create_schema_sql)
        .execute(&mut bootstrap_conn)
        .await
        .expect("Failed to create schema");

    let pool = PgPoolOptions::new()
        .max_connections(5)
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
        .expect("Failed to create PostgreSQL connection pool");

    verify_schema(&pool, &db_settings.schema)
        .await
        .expect("Connected to unexpected schema");
    let migrator = sqlx::migrate!();
    debug!("Found {} migrations", migrator.migrations.len());

    migrator
        .run(&pool)
        .await
        .inspect_err(|e| error!("Failed to run migrations: {}", e))
        .unwrap();

    migrator.iter().for_each(|m| {
        debug!("Applied migration: {:#?}", m);
    });
    pool
}

async fn verify_schema(pool: &Pool<Postgres>, expected_schema: &str) -> Result<(), DbInitError> {
    let mut conn = pool.acquire().await?;
    let search_path: String = sqlx::query_scalar("show search_path")
        .fetch_one(&mut *conn)
        .await?;
    let current_schema: Option<String> = sqlx::query_scalar("select current_schema()")
        .fetch_one(&mut *conn)
        .await?;

    let active_schema = current_schema.unwrap_or_else(|| "public".to_string());

    if active_schema != expected_schema {
        let schema_exists: bool =
            sqlx::query_scalar("select exists(select 1 from pg_namespace where nspname = $1)")
                .bind(expected_schema)
                .fetch_one(&mut *conn)
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

fn escape_identifier(identifier: &str) -> String {
    let escaped = identifier.replace('"', "\"\"");
    format!("\"{}\"", escaped)
}
