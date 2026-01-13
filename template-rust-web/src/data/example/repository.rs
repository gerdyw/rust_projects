use super::models::{CreateExampleEntity, ExampleEntity, UpdateExampleEntity};
use crate::db::RepositoryError;
use sqlx::PgPool;
use uuid::Uuid;

/// Repository for database operations on ExampleEntity
#[derive(Clone)]
pub struct ExampleRepository {
    pool: PgPool,
}

impl ExampleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Find an entity by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<ExampleEntity, RepositoryError> {
        let entity = sqlx::query_as::<_, ExampleEntity>(
            "SELECT id, created_at, updated_at, name, description 
             FROM example_entities 
             WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(entity)
    }

    /// List all entities
    pub async fn find_all(&self) -> Result<Vec<ExampleEntity>, RepositoryError> {
        let entities = sqlx::query_as::<_, ExampleEntity>(
            "SELECT id, created_at, updated_at, name, description 
             FROM example_entities 
             ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(entities)
    }

    /// Create a new entity
    pub async fn create(
        &self,
        data: CreateExampleEntity,
    ) -> Result<ExampleEntity, RepositoryError> {
        let entity = sqlx::query_as::<_, ExampleEntity>(
            "INSERT INTO example_entities (id, created_at, updated_at, name, description)
             VALUES ($1, NOW(), NOW(), $2, $3)
             RETURNING id, created_at, updated_at, name, description",
        )
        .bind(Uuid::new_v4())
        .bind(&data.name)
        .bind(&data.description)
        .fetch_one(&self.pool)
        .await?;

        Ok(entity)
    }

    /// Update an existing entity
    pub async fn update(
        &self,
        id: Uuid,
        data: UpdateExampleEntity,
    ) -> Result<ExampleEntity, RepositoryError> {
        let entity = sqlx::query_as::<_, ExampleEntity>(
            "UPDATE example_entities 
             SET name = COALESCE($2, name),
                 description = COALESCE($3, description),
                 updated_at = NOW()
             WHERE id = $1
             RETURNING id, created_at, updated_at, name, description",
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.description)
        .fetch_one(&self.pool)
        .await?;

        Ok(entity)
    }

    /// Delete an entity
    pub async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let result = sqlx::query("DELETE FROM example_entities WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}
