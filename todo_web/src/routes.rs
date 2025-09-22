use crate::{
    appstate::AppState,
    handlers::{create_todo, delete_todo, get_index, list_todos, mark_done, mark_undone},
};
use axum::{
    Router,
    routing::{delete, get, put},
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_index))
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/{id}/done", put(mark_done))
        .route("/todos/{id}/undone", put(mark_undone))
        .route("/todos/{id}", delete(delete_todo))
        .with_state(state)
}
