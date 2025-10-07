use axum::response::{IntoResponse, Redirect, Response};
use maud::Render;

use crate::domain::components::HtmlComponent;

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

pub enum HttpResponse<T: Render> {
    Html(HtmlComponent<T>),
    HxRedirect(String),
    Redirect(String),
    BadRequestHtml(HtmlComponent<T>),
    BadRequest(String),
    Unauthorized(String),
    UnauthorizedRedirect,
    NotFound(String),
    Conflict(String),
    NoContent,
    InternalServerError,
}

impl<T: Render> IntoResponse for HttpResponse<T> {
    fn into_response(self) -> Response {
        let result: Result<Response, axum::http::Error> = match self {
            HttpResponse::Html(html) => Ok(html.into_response()),
            HttpResponse::HxRedirect(url) => Response::builder()
                .header("HX-Redirect", &url)
                .status(200)
                .body(axum::body::Body::empty()),
            HttpResponse::Redirect(url) => Response::builder()
                .header("HX-Redirect", &url)
                .header("Location", &url)
                .status(302)
                .body(axum::body::Body::empty()),
            HttpResponse::BadRequestHtml(html) => Response::builder()
                .status(400)
                .body(html.into_response().into_body()),
            HttpResponse::BadRequest(message) => Response::builder()
                .status(400)
                .body(axum::body::Body::from(message)),
            HttpResponse::Unauthorized(message) => Response::builder()
                .status(401)
                .body(axum::body::Body::from(message)),
            HttpResponse::UnauthorizedRedirect => Ok(Redirect::to("/users").into_response()),
            HttpResponse::NotFound(message) => Response::builder()
                .status(404)
                .body(axum::body::Body::from(message)),
            HttpResponse::Conflict(message) => Response::builder()
                .status(409)
                .body(axum::body::Body::from(message)),
            HttpResponse::NoContent => Response::builder()
                .status(204)
                .body(axum::body::Body::empty()),
            HttpResponse::InternalServerError => Response::builder()
                .status(500)
                .body(axum::body::Body::from("Internal Server Error")),
        };

        match result {
            Ok(resp) => resp,
            Err(e) => {
                tracing::error!("Failed to build response: {:?}", e);
                Response::builder()
                    .status(500)
                    .body(axum::body::Body::from("Internal Server Error"))
                    .unwrap()
            }
        }
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
