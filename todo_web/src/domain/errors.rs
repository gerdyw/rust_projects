use axum::{http::Response, response::IntoResponse};
use tower_sessions::session::Error;
use tracing::{debug, warn};

use crate::db::errors::RepoError;

#[derive(Debug)]
pub struct InternalServerError {
    pub message: String,
}

impl IntoResponse for InternalServerError {
    fn into_response(self) -> axum::response::Response {
        warn!("Internal Server Error: {}", self.message);
        Response::builder()
            .status(500)
            .header("HX-Redirect", format!("/error?message={}", self.message))
            .body(format!("Internal Server Error: {}", self.message).into())
            .unwrap()
    }
}

impl From<sqlx::Error> for InternalServerError {
    fn from(error: sqlx::Error) -> Self {
        InternalServerError {
            message: error.to_string(),
        }
    }
}

impl From<Error> for InternalServerError {
    fn from(error: Error) -> Self {
        InternalServerError {
            message: error.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ServiceError {
    NotFound,
    Conflict,
    BadRequest,
    InternalError,
}

impl From<RepoError> for ServiceError {
    fn from(error: RepoError) -> Self {
        match error {
            RepoError::NotFound => ServiceError::NotFound,
            RepoError::DuplicateKey => ServiceError::Conflict,
            RepoError::InvalidInput => ServiceError::BadRequest,
            RepoError::DbError(_) => ServiceError::InternalError,
        }
    }
}

impl From<Error> for ServiceError {
    fn from(error: Error) -> Self {
        debug!("Error setting or getting session: {}", error);
        ServiceError::InternalError
    }
}
