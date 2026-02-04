use std::fmt;

/// Errors that can occur during database initialization
#[derive(Debug)]
pub enum DbInitError {
    /// Underlying sqlx error
    Sqlx(sqlx::Error),
    /// Connected to wrong schema
    WrongSchema {
        expected: String,
        actual: String,
        search_path: String,
    },
    /// Schema doesn't exist
    SchemaMissing {
        expected: String,
        search_path: String,
    },
    /// Migration error
    Migration(String),
}

impl fmt::Display for DbInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbInitError::Sqlx(err) => write!(f, "Database error: {}", err),
            DbInitError::WrongSchema {
                expected,
                actual,
                search_path,
            } => write!(
                f,
                "Connected to unexpected schema. Expected '{}', got '{}' (search_path: {})",
                expected, actual, search_path
            ),
            DbInitError::SchemaMissing {
                expected,
                search_path,
            } => write!(
                f,
                "Schema '{}' does not exist (search_path: {})",
                expected, search_path
            ),
            DbInitError::Migration(msg) => write!(f, "Migration error: {}", msg),
        }
    }
}

impl std::error::Error for DbInitError {}

impl From<sqlx::Error> for DbInitError {
    fn from(err: sqlx::Error) -> Self {
        DbInitError::Sqlx(err)
    }
}

impl From<sqlx::migrate::MigrateError> for DbInitError {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        DbInitError::Migration(err.to_string())
    }
}
