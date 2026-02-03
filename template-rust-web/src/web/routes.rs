use crate::domain::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

/// Create routes for example entity CRUD operations
pub fn examples_router() -> Router<AppState> {
    Router::new()
        .route("/", get(super::list_examples))
        .route("/:id", get(super::get_example))
        .route("/", post(super::create_example))
        .route("/:id", put(super::update_example))
        .route("/:id", delete(super::delete_example))
}
