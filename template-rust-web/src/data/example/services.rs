use crate::db::RepositoryError;
use uuid::Uuid;
use super::models::{ExampleEntity, CreateExampleEntity, UpdateExampleEntity};
use super::repository::ExampleRepository;

/// Service layer for business logic on ExampleEntity
pub struct ExampleService {
    repository: ExampleRepository,
}

impl ExampleService {
    pub fn new(repository: ExampleRepository) -> Self {
        Self { repository }
    }

    /// Get an entity by ID
    pub async fn get(&self, id: Uuid) -> Result<ExampleEntity, RepositoryError> {
        self.repository.find_by_id(id).await
    }

    /// List all entities
    pub async fn list(&self) -> Result<Vec<ExampleEntity>, RepositoryError> {
        self.repository.find_all().await
    }

    /// Create a new entity with validation
    pub async fn create(&self, data: CreateExampleEntity) -> Result<ExampleEntity, RepositoryError> {
        // Add business logic validation here
        if data.name.trim().is_empty() {
            return Err(RepositoryError::ValidationError(
                "Name cannot be empty".to_string()
            ));
        }

        self.repository.create(data).await
    }

    /// Update an entity with validation
    pub async fn update(&self, id: Uuid, data: UpdateExampleEntity) -> Result<ExampleEntity, RepositoryError> {
        // Add business logic validation here
        if let Some(ref name) = data.name {
            if name.trim().is_empty() {
                return Err(RepositoryError::ValidationError(
                    "Name cannot be empty".to_string()
                ));
            }
        }

        self.repository.update(id, data).await
    }

    /// Delete an entity
    pub async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        self.repository.delete(id).await
    }
}
