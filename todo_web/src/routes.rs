use crate::{
    appstate::AppState,
    handlers::{create_todo, delete_todo, get_index, list_todos, mark_done, mark_undone},
};
use axum::{
    Router,
    routing::{delete, get, put},
};

use tower_http::services::ServeDir;

pub fn create_router(state: AppState, assets_location: &String) -> Router {
    let static_router = get_servedir(assets_location);
    Router::new()
        .route("/", get(get_index))
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/{id}/done", put(mark_done))
        .route("/todos/{id}/undone", put(mark_undone))
        .route("/todos/{id}", delete(delete_todo))
        .with_state(state)
        .nest_service("/assets", static_router)
}

fn get_servedir(assets_location: &String) -> ServeDir {
    // embed_assets!("assets", compress = true);
    // static_router()

    ServeDir::new(assets_location)
}
