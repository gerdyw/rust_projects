use crate::{appstate::AppState, models::CreateTodo};

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};

pub async fn list_todos(State(state): State<AppState>) -> impl IntoResponse {
    let todos = state.repo.list().await.unwrap();
    Json(todos)
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = state.repo.create(payload).await.unwrap();
    Json(todo)
}

pub async fn mark_done(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    let todo = state.repo.mark_done(id).await.unwrap();
    Json(todo)
}

pub async fn delete_todo(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    state.repo.delete(id).await.unwrap();
    "Deleted".into_response()
}
