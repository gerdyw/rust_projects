use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tracing::debug;

use crate::domain::settings::DatabaseSettings;
pub mod metadata;

pub async fn init_db(db_settings: &DatabaseSettings) -> Pool<Postgres> {
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_settings.username,
        db_settings.password,
        db_settings.host,
        db_settings.port,
        db_settings.db_name
    );
    debug!("Database URL: {}", db_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create PostgreSQL connection pool");

    sqlx::migrate!()
        .run(&pool)
        .await
        .inspect_err(|e| debug!("Failed to run migrations: {}", e))
        .ok();

    pool
}
