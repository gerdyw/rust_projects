use db_init::{DbConfig, SchemaMode, init_pool, run_migrations};
use sqlx::{Pool, Postgres, migrate::Migrator};

pub mod repo;
pub mod models;

pub async fn init() -> Result<Pool<Postgres>, String> {
    let config = DbConfig::from_env(SchemaMode::CreateIfMissing)?;
    let pool = init_pool(&config).await.map_err(|err| err.to_string())?;
    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    run_migrations(&pool, &MIGRATOR).await.map_err(|err| err.to_string())?;
    Ok(pool)
}
