use crate::{
    appstate::AppState,
    handlers::{create_todo, delete_todo, list_todos, mark_done},
};
use axum::{
    Router,
    routing::{delete, get, put},
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/{id}/done", put(mark_done))
        .route("/todos/{id}", delete(delete_todo))
        .with_state(state)
}
