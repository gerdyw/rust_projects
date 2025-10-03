use uuid::Uuid;

use crate::{
    api::todo::{
        models::{CreateTodo, Todo},
        repository::TodoRepository,
    },
    domain::errors::InternalServerError,
};

#[derive(Clone)]
pub struct TodoService {
    repo: TodoRepository,
}

impl TodoService {
    pub fn new(repo: TodoRepository) -> Self {
        Self { repo }
    }

    pub async fn list_for_user(&self, user_id: Uuid) -> Result<Vec<Todo>, InternalServerError> {
        self.repo.list_for_user(user_id).await.map_err(|e| {
            tracing::error!("Failed to list todos for user {}: {}", user_id, e);
            InternalServerError::from(e)
        })
    }

    pub async fn create(&self, user_id: Uuid, title: String) -> Result<Todo, InternalServerError> {
        let payload = CreateTodo { title };
        self.repo.create(user_id, payload).await.map_err(|e| {
            tracing::error!("Failed to create todo: {}", e);
            InternalServerError::from(e)
        })
    }

    pub async fn mark_done(&self, id: Uuid) -> Result<Todo, InternalServerError> {
        self.repo.mark_done(id).await.map_err(|e| {
            tracing::error!("Failed to mark todo {} as done: {}", id, e);
            InternalServerError::from(e)
        })
    }

    pub async fn mark_undone(&self, id: Uuid) -> Result<u64, InternalServerError> {
        self.repo.mark_undone(id).await.map_err(|e| {
            tracing::error!("Failed to mark todo {} as undone: {}", id, e);
            InternalServerError::from(e)
        })
    }

    pub async fn delete(&self, id: Uuid) -> Result<u64, InternalServerError> {
        self.repo.delete(id).await.map_err(|e| {
            tracing::error!("Failed to delete todo {}: {}", id, e);
            InternalServerError::from(e)
        })
    }
}
