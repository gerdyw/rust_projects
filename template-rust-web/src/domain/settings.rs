use std::env;

/// Application settings loaded from environment variables
#[derive(Debug, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub service_port: u16,
}

#[derive(Debug, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
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

        Ok(Self {
            database,
            service_port,
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
