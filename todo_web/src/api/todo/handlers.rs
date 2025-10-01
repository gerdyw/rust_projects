use crate::api::todo::components::TodoList;
use crate::api::todo::models::CreateTodo;
use crate::domain::appstate::AppState;
use crate::domain::components::{HtmlComponent, IntoHtmlComponent};
use axum::{
    Form,
    extract::{Path, State},
    response::IntoResponse,
};
use tracing::error;
use uuid::Uuid;

pub async fn list_todos(State(state): State<AppState>) -> HtmlComponent<TodoList> {
    let todos = state.todo_repo.list().await.unwrap();
    TodoList::from(todos).into_html_component()
}

pub async fn create_todo(
    state: State<AppState>,
    Form(payload): Form<CreateTodo>,
) -> impl IntoResponse {
    let _ = state
        .todo_repo
        .create(payload)
        .await
        .inspect_err(|e| error!("{}", e));

    list_todos(state).await
}

pub async fn mark_done(state: State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let _ = state
        .todo_repo
        .mark_done(id)
        .await
        .inspect_err(|e| error!("{:#?}", e));

    list_todos(state).await
}

pub async fn mark_undone(state: State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let _ = state
        .todo_repo
        .mark_undone(id)
        .await
        .inspect_err(|e| error!("{:#?}", e));

    list_todos(state).await
}

pub async fn delete_todo(state: State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let _ = state
        .todo_repo
        .delete(id)
        .await
        .inspect_err(|e| error!("{}", e));
    list_todos(state).await
}
