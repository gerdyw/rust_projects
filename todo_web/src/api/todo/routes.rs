use super::handlers::{create_todo, delete_todo, list_todos, mark_undone};
use crate::domain::appstate::AppState;

use axum::{
    Router,
    routing::{delete, get, put},
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(list_todos).post(create_todo))
        .route("/{id}/done", put(mark_undone))
        .route("/{id}/undone", put(mark_undone))
        .route("/{id}", delete(delete_todo))
        .with_state(state)
}
