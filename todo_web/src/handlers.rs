use crate::{
    appstate::AppState,
    components::{HtmlComponent, Index, IntoHtmlComponent, TodoList},
    models::CreateTodo,
};

use axum::{
    Form,
    extract::{Path, State},
    response::IntoResponse,
};

pub async fn get_index(state: State<AppState>) -> impl IntoResponse {
    let todos = list_todos(state).await;
    Index { todos }.into_html_component()
}

pub async fn list_todos(State(state): State<AppState>) -> HtmlComponent<TodoList> {
    let todos = state.repo.list().await.unwrap();
    TodoList(todos).into_html_component()
}

pub async fn create_todo(
    state: State<AppState>,
    Form(payload): Form<CreateTodo>,
) -> impl IntoResponse {
    state.repo.create(payload).await.unwrap();
    list_todos(state).await
}

pub async fn mark_done(state: State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    state.repo.mark_done(id).await.unwrap();
    list_todos(state).await
}

pub async fn mark_undone(state: State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    state.repo.mark_undone(id).await.unwrap();
    list_todos(state).await
}

pub async fn delete_todo(state: State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    state.repo.delete(id).await.unwrap();
    list_todos(state).await
}
