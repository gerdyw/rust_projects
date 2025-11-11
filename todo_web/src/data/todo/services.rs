use uuid::Uuid;

use crate::{
    data::todo::{
        models::{CreateTodo, Todo},
        repository::TodoRepository,
    },
    domain::errors::ServiceError,
};

#[derive(Clone)]
pub struct TodoService {
    repo: TodoRepository,
}

impl TodoService {
    pub fn new(repo: TodoRepository) -> Self {
        Self { repo }
    }

    pub async fn list_for_user(&self, user_id: Uuid) -> Result<Vec<Todo>, ServiceError> {
        Ok(self.repo.list_for_user(user_id).await?)
    }

    pub async fn create(&self, user_id: Uuid, title: String) -> Result<Vec<Todo>, ServiceError> {
        let payload = CreateTodo { title };
        let _ = self.repo.create(user_id, payload).await?;

        let todos = self.repo.list_for_user(user_id).await?;
        Ok(todos)
    }

    pub async fn mark_done(&self, user_id: Uuid, id: Uuid) -> Result<Vec<Todo>, ServiceError> {
        self.repo.mark_done(id).await?;

        let todos = self.repo.list_for_user(user_id).await?;
        Ok(todos)
    }

    pub async fn mark_undone(&self, user_id: Uuid, id: Uuid) -> Result<Vec<Todo>, ServiceError> {
        self.repo.mark_undone(id).await?;

        let todos = self.repo.list_for_user(user_id).await?;
        Ok(todos)
    }

    pub async fn delete(&self, user_id: Uuid, id: Uuid) -> Result<Vec<Todo>, ServiceError> {
        self.repo.delete(id).await?;

        let todos = self.repo.list_for_user(user_id).await?;
        Ok(todos)
    }
}
