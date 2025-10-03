use axum::response::{IntoResponse, Redirect};
use maud::html;

use crate::domain::components::{IntoHtmlComponent, Page};

pub async fn get_index() -> impl IntoResponse {
    Redirect::to("/todos")
}

pub async fn get_error() -> impl IntoResponse {
    Page("error", html!(h3 { "An unexpected error has occurred" })).into_html_component()
}
