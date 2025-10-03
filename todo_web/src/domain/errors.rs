use axum::response::{IntoResponse, Redirect};
use tower_sessions::session::Error;

#[derive(Debug)]
pub struct InternalServerError {
    pub message: String,
}

impl IntoResponse for InternalServerError {
    fn into_response(self) -> axum::response::Response {
        Redirect::to("/error").into_response()
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
