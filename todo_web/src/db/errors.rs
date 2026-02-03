#[derive(Debug)]
pub enum RepoError {
    NotFound,
    InvalidInput,
    DuplicateKey,
    DbError(sqlx::Error),
}

#[derive(Debug)]
pub enum DbInitError {
    Sqlx(sqlx::Error),
    SchemaMissing {
        expected: String,
        search_path: String,
    },
    WrongSchema {
        expected: String,
        actual: String,
        search_path: String,
    },
}

impl std::fmt::Display for DbInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbInitError::Sqlx(err) => write!(f, "Database error: {}", err),
            DbInitError::SchemaMissing {
                expected,
                search_path,
            } => write!(
                f,
                "Schema '{}' does not exist (search_path: {})",
                expected, search_path
            ),
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

impl From<sqlx::Error> for RepoError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => RepoError::NotFound,
            sqlx::Error::ColumnNotFound(_) => RepoError::InvalidInput,
            sqlx::Error::Database(err) if err.is_unique_violation() => RepoError::DuplicateKey,
            _ => RepoError::DbError(err),
        }
    }
}
