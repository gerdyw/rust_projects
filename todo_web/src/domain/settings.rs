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
    let database = DbConfig::from_env(SchemaMode::CreateIfMissing)
        .expect("Failed to load database configuration");
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
