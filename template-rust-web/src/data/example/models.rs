use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Example entity model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExampleEntity {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: Option<String>,
}

/// Data transfer object for creating new entities
#[derive(Debug, Deserialize)]
pub struct CreateExampleEntity {
    pub name: String,
    pub description: Option<String>,
}

/// Data transfer object for updating entities
#[derive(Debug, Deserialize)]
pub struct UpdateExampleEntity {
    pub name: Option<String>,
    pub description: Option<String>,
}
