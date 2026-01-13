use std::fmt;

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
