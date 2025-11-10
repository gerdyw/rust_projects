use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use tower_sessions::session::Error as SessionError;

#[repr(u16)]
pub enum HttpError {
    InternalServerError(String) = 500,
    NotFound(String) = 404,
    BadRequest(String) = 400,
    Unauthorized = 401,
    Forbidden = 403,
}

impl From<sqlx::Error> for HttpError {
    fn from(error: sqlx::Error) -> Self {
        HttpError::InternalServerError(error.to_string())
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match self {
            HttpError::InternalServerError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
            HttpError::NotFound(message) => (StatusCode::NOT_FOUND, message).into_response(),
            HttpError::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            HttpError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            HttpError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden").into_response(),
        }
    }
}

pub enum ServiceError {
    DatabaseError(sqlx::Error),
    NotFound,
    Unauthorized,
    InternalError(String),
}

impl From<sqlx::Error> for ServiceError {
    fn from(error: sqlx::Error) -> Self {
        ServiceError::DatabaseError(error)
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(error: serde_json::Error) -> Self {
        ServiceError::InternalError(error.to_string())
    }
}

impl From<SessionError> for ServiceError {
    fn from(error: SessionError) -> Self {
        ServiceError::InternalError(error.to_string())
    }
}

impl From<ServiceError> for HttpError {
    fn from(val: ServiceError) -> Self {
        match val {
            ServiceError::DatabaseError(e) => HttpError::InternalServerError(e.to_string()),
            ServiceError::NotFound => HttpError::NotFound("Resource not found".to_string()),
            ServiceError::Unauthorized => HttpError::Unauthorized,
            ServiceError::InternalError(message) => HttpError::InternalServerError(message),
        }
    }
}
