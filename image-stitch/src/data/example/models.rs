use crate::db::Timestamped;
use serde::{Deserialize, Serialize};

/// Example entity model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExampleEntity {
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub timestamped: Timestamped,
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
