use std::fmt;

/// Errors that can occur while preparing database connectivity
#[derive(Debug)]
pub enum DbInitError {
    Sqlx(sqlx::Error),
    WrongSchema {
        expected: String,
        actual: String,
        search_path: String,
    },
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
        }
    }
}

impl std::error::Error for DbInitError {}

impl From<sqlx::Error> for DbInitError {
    fn from(err: sqlx::Error) -> Self {
        DbInitError::Sqlx(err)
    }
}

/// Repository-level errors
#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    DatabaseError(sqlx::Error),
    ValidationError(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::NotFound => write!(f, "Resource not found"),
            RepositoryError::DatabaseError(e) => write!(f, "Database error: {}", e),
            RepositoryError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

impl From<sqlx::Error> for RepositoryError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => RepositoryError::NotFound,
            e => RepositoryError::DatabaseError(e),
        }
    }
}
