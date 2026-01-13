use crate::domain::AppState;
use axum::{
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;

/// Create the application router with all routes and middleware
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(super::health_check))
        // Add your routes here
        // .route("/api/example", get(your_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
