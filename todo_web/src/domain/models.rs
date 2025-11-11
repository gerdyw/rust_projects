use axum::response::{IntoResponse, Redirect, Response};
use maud::Render;

use crate::domain::components::HtmlComponent;
use crate::domain::errors::ServiceError;

// Add this struct to store session data
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct UserSession {
    pub user_id: uuid::Uuid,
}

pub enum AuthedResult<T> {
    Authed(T),
    NotAuthed,
}
impl<T> IntoResponse for AuthedResult<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            AuthedResult::Authed(data) => data.into_response(),
            AuthedResult::NotAuthed => Redirect::to("/users").into_response(),
        }
    }
}

pub enum HttpSuccess<T: Render> {
    Html(HtmlComponent<T>),
    HxRedirect(String),
    Redirect(String),
    NoContent,
}

impl<T: Render> IntoResponse for HttpSuccess<T> {
    fn into_response(self) -> Response {
        let result: Result<Response, axum::http::Error> = match self {
            HttpSuccess::Html(html) => Ok(html.into_response()),
            HttpSuccess::HxRedirect(url) => Response::builder()
                .header("HX-Redirect", &url)
                .status(200)
                .body(axum::body::Body::empty()),
            HttpSuccess::Redirect(url) => Response::builder()
                .header("HX-Redirect", &url)
                .header("Location", &url)
                .status(302)
                .body(axum::body::Body::empty()),
            HttpSuccess::NoContent => Response::builder()
                .status(204)
                .body(axum::body::Body::empty()),
        };

        match result {
            Ok(resp) => resp,
            Err(e) => {
                tracing::error!("Failed to build success response: {:?}", e);
                Response::builder()
                    .status(500)
                    .body(axum::body::Body::from("Internal Server Error"))
                    .unwrap()
            }
        }
    }
}

pub enum HttpError {
    BadRequest(String),
    Unauthorized(String),
    UnauthorizedRedirect,
    NotFound(String),
    Conflict(String),
    InternalServerError,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let result: Result<Response, axum::http::Error> = match self {
            HttpError::BadRequest(message) => Response::builder()
                .status(400)
                .body(axum::body::Body::from(message)),
            HttpError::Unauthorized(message) => Response::builder()
                .status(401)
                .body(axum::body::Body::from(message)),
            HttpError::UnauthorizedRedirect => Ok(Redirect::to("/users").into_response()),
            HttpError::NotFound(message) => Response::builder()
                .status(404)
                .body(axum::body::Body::from(message)),
            HttpError::Conflict(message) => Response::builder()
                .status(409)
                .body(axum::body::Body::from(message)),
            HttpError::InternalServerError => Response::builder()
                .status(500)
                .body(axum::body::Body::from("Internal Server Error")),
        };

        match result {
            Ok(resp) => resp,
            Err(e) => {
                tracing::error!("Failed to build error response: {:?}", e);
                Response::builder()
                    .status(500)
                    .body(axum::body::Body::from("Internal Server Error"))
                    .unwrap()
            }
        }
    }
}

// From implementations for converting errors to HttpError
impl From<ServiceError> for HttpError {
    fn from(error: ServiceError) -> Self {
        match error {
            ServiceError::NotFound => HttpError::NotFound("Resource not found".to_string()),
            ServiceError::Conflict => HttpError::Conflict("Conflict occurred".to_string()),
            ServiceError::BadRequest => HttpError::BadRequest("Bad request".to_string()),
            ServiceError::InternalError => HttpError::InternalServerError,
        }
    }
}

impl From<tower_sessions::session::Error> for HttpError {
    fn from(_error: tower_sessions::session::Error) -> Self {
        HttpError::InternalServerError
    }
}

pub struct Empty;

impl maud::Render for Empty {
    fn render_to(&self, _: &mut String) {}
}

pub struct Never;

impl maud::Render for Never {
    fn render_to(&self, _: &mut String) {
        panic!("Never should never be rendered");
    }
}
