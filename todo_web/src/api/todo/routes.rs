use super::handlers::{create_todo, delete_todo, get_index, mark_undone};
use crate::{
    api::todo::handlers::mark_done,
    domain::{appstate::AppState, middleware::require_auth},
};

use axum::{
    Router, middleware,
    routing::{delete, get, put},
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_index).post(create_todo))
        .route("/{id}/done", put(mark_done))
        .route("/{id}/undone", put(mark_undone))
        .route("/{id}", delete(delete_todo))
        .with_state(state)
        .layer(middleware::from_fn(require_auth))
}
