use crate::domain::AppState;
use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};

/// Health check endpoint - verifies database connectivity
pub async fn health_check(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    // Test database connection with a simple query
    let result = sqlx::query("SELECT 1").fetch_one(&state.db).await;

    match result {
        Ok(_) => Ok(Json(json!({
            "status": "healthy",
            "database": "connected"
        }))),
        Err(e) => {
            tracing::error!("Database health check failed: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}
