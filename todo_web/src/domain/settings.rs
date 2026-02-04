use db_init::{DbConfig, SchemaMode};
use tower_sessions_redis_store::fred::prelude::{Config, Server, ServerConfig};

#[derive(Debug)]
pub struct Settings {
    pub database: DbConfig,
    pub cache: Config,
    pub assets_location: String,
    pub port: u16,
}

pub fn load_settings() -> Settings {
    let database = load_db_settings();
    let cache = load_cache_settings();

    Settings {
        database,
        cache,
        assets_location: std::env::var("ASSETS_LOCATION").expect("ASSETS_LOCATION must be set"),
        port: std::env::var("SERVICE_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("SERVICE_PORT must be a valid port number"),
    }
}

fn load_db_settings() -> DbConfig {
    DbConfig {
        host: std::env::var("DATABASE_HOST").expect("DATABASE_HOST must be set"),
        port: std::env::var("DATABASE_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse()
            .expect("DATABASE_PORT must be a valid port number"),
        database: std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set"),
        username: std::env::var("DATABASE_USER").expect("DATABASE_USER must be set"),
        password: std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set"),
        schema: std::env::var("DATABASE_SCHEMA").expect("DATABASE_SCHEMA must be set"),
        max_connections: 5,
        acquire_timeout_secs: 3,
        schema_mode: SchemaMode::CreateIfMissing,
    }
}

fn load_cache_settings() -> Config {
    Config {
        server: ServerConfig::Centralized {
            server: Server {
                host: std::env::var("CACHE_HOST")
                    .expect("CACHE_HOST must be set")
                    .into(),
                port: std::env::var("CACHE_PORT")
                    .expect("CACHE_PORT must be set")
                    .parse()
                    .expect("CACHE_PORT must be a valid port number"),
            },
        },
        ..Default::default()
    }
}
