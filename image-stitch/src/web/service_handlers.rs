use crate::data::example::{CreateExampleEntity, ExampleEntity, UpdateExampleEntity};
use crate::db::RepositoryError;
use crate::domain::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use uuid::Uuid;

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

// Example entity handlers

/// List all example entities
pub async fn list_examples(
    State(state): State<AppState>,
) -> Result<Json<Vec<ExampleEntity>>, StatusCode> {
    state.example_service.list().await.map(Json).map_err(|e| {
        tracing::error!("Failed to list examples: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

/// Get a specific example entity by ID
pub async fn get_example(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ExampleEntity>, StatusCode> {
    state
        .example_service
        .get(id)
        .await
        .map(Json)
        .map_err(|e| match e {
            RepositoryError::NotFound => StatusCode::NOT_FOUND,
            _ => {
                tracing::error!("Failed to get example {}: {}", id, e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })
}

/// Create a new example entity
pub async fn create_example(
    State(state): State<AppState>,
    Json(data): Json<CreateExampleEntity>,
) -> Result<(StatusCode, Json<ExampleEntity>), StatusCode> {
    state
        .example_service
        .create(data)
        .await
        .map(|entity| (StatusCode::CREATED, Json(entity)))
        .map_err(|e| match e {
            RepositoryError::ValidationError(_) => StatusCode::BAD_REQUEST,
            _ => {
                tracing::error!("Failed to create example: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })
}

/// Update an existing example entity
pub async fn update_example(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(data): Json<UpdateExampleEntity>,
) -> Result<Json<ExampleEntity>, StatusCode> {
    state
        .example_service
        .update(id, data)
        .await
        .map(Json)
        .map_err(|e| match e {
            RepositoryError::NotFound => StatusCode::NOT_FOUND,
            RepositoryError::ValidationError(_) => StatusCode::BAD_REQUEST,
            _ => {
                tracing::error!("Failed to update example {}: {}", id, e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })
}

/// Delete an example entity
pub async fn delete_example(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    state
        .example_service
        .delete(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| match e {
            RepositoryError::NotFound => StatusCode::NOT_FOUND,
            _ => {
                tracing::error!("Failed to delete example {}: {}", id, e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })
}
