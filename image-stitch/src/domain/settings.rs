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
    pub database: DatabaseSettings,
    pub service_port: u16,
    pub api_key: ApiKeyMode,
}

#[derive(Debug, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
    pub schema: String,
}

impl Settings {
    /// Load settings from environment variables
    pub fn from_env() -> Result<Self, String> {
        let database = DatabaseSettings {
            host: env::var("DATABASE_HOST").map_err(|_| "DATABASE_HOST not set".to_string())?,
            port: env::var("DATABASE_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .map_err(|_| "DATABASE_PORT must be a valid number".to_string())?,
            name: env::var("DATABASE_NAME").map_err(|_| "DATABASE_NAME not set".to_string())?,
            user: env::var("DATABASE_USER").map_err(|_| "DATABASE_USER not set".to_string())?,
            password: env::var("DATABASE_PASSWORD")
                .map_err(|_| "DATABASE_PASSWORD not set".to_string())?,
            schema: env::var("DATABASE_SCHEMA").unwrap_or_else(|_| "public".to_string()),
        };

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

    /// Build database connection URL with schema
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?search_path={}",
            self.database.user,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.name,
            self.database.schema
        )
    }
}
