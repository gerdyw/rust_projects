use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
};
use maud::html;
use serde::{Deserialize, Serialize};

use crate::domain::components::{IntoHtmlComponent, Page};

pub async fn get_index() -> impl IntoResponse {
    Redirect::to("/todos")
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
