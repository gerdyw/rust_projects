use axum::{extract::Query, response::IntoResponse};
use maud::html;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::domain::{
    components::{IntoHtmlComponent, Page},
    models::{HttpResponse, Never},
};

pub async fn get_index() -> HttpResponse<Never> {
    debug!("GET / redirecting to /todos");
    HttpResponse::Redirect("/todos".into())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorQuery {
    message: Option<String>,
}
// #[axum::debug_handler]
pub async fn get_error(Query(query): Query<ErrorQuery>) -> impl IntoResponse {
    Page(
        "error",
        html!(h3 { "An unexpected error has occurred"
                @if let Some(msg) = query.message {
                    p { (msg.clone()) }
                }
        }),
    )
    .into_html_component()
}
