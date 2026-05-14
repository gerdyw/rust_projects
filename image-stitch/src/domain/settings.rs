use db_init::{DbConfig, SchemaMode};
use std::env;

/// API key authentication mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiKeyMode {
    /// Full - API key required for all endpoints
    Full(String),
    /// SubmitOnly - API key required only for POST endpoints (/stitch and /jobs)
    SubmitOnly(String),
    /// None - No API key required
    None,
}

/// Application settings loaded from environment variables
#[derive(Debug, Clone)]
pub struct Settings {
    pub database: DbConfig,
    pub service_port: u16,
    pub api_key: ApiKeyMode,
}

impl Settings {
    /// Load settings from environment variables
    pub fn from_env() -> Result<Self, String> {
        let database = DbConfig::from_env(SchemaMode::MustExist)?;

        let service_port = env::var("SERVICE_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|_| "SERVICE_PORT must be a valid number".to_string())?;

        // API Key configuration
        let api_key_mode_str = env::var("API_KEY_MODE").unwrap_or_else(|_| "full".to_string());

        let api_key = match api_key_mode_str.to_lowercase().as_str() {
            "full" => {
                let key = env::var("API_KEY")
                    .map_err(|_| "API_KEY must be set when API_KEY_MODE is 'full'.\nTo disable API key authentication, set API_KEY_MODE to 'none'.".to_string())?;
                ApiKeyMode::Full(key)
            }
            "submit_only" | "submitonly" => {
                let key = env::var("API_KEY").map_err(|_| {
                    "API_KEY must be set when API_KEY_MODE is 'submit_only'".to_string()
                })?;
                ApiKeyMode::SubmitOnly(key)
            }
            _ => ApiKeyMode::None,
        };

        Ok(Self {
            database,
            service_port,
            api_key,
        })
    }
}
