use axum::{extract::State, response::IntoResponse};

use crate::{
    api::todo::{components::TodoPage, handlers::list_todos},
    domain::{appstate::AppState, components::IntoHtmlComponent},
};

pub async fn get_index(state: State<AppState>) -> impl IntoResponse {
    let todos = list_todos(state).await;
    TodoPage { todos }.into_html_component()
}
