use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Common database fields for entities with timestamps
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Timestamped {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Timestamped {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl Default for Timestamped {
    fn default() -> Self {
        Self::new()
    }
}
