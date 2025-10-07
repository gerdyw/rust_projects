#[derive(Debug)]
pub enum RepoError {
    NotFound,
    InvalidInput,
    DuplicateKey,
    DbError(sqlx::Error),
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
