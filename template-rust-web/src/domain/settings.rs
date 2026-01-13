use std::env;

/// Application settings loaded from environment variables
#[derive(Debug, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub service_port: u16,
    pub api_key: ApiKeySettings,
}

#[derive(Debug, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct ApiKeySettings {
    pub enabled: bool,
    pub key: Option<String>,
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
        };

        let service_port = env::var("SERVICE_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|_| "SERVICE_PORT must be a valid number".to_string())?;

        // API Key configuration
        let api_key_disabled = env::var("API_KEY_DISABLED")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase();
        let api_key_enabled = api_key_disabled != "true";

        let api_key = if api_key_enabled {
            let key = env::var("API_KEY")
                .map_err(|_| "API_KEY must be set, or set API_KEY_DISABLED=true to disable authentication".to_string())?;
            Some(key)
        } else {
            None
        };

        let api_key_settings = ApiKeySettings {
            enabled: api_key_enabled,
            key: api_key,
        };

        Ok(Self {
            database,
            service_port,
            api_key: api_key_settings,
        })
    }

    /// Build database connection URL
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database.user,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.name
        )
    }
}
