use sqlx::Error;
use uuid::Uuid;

use crate::api::todo::{
    models::{CreateTodo, Todo},
    repository::TodoRepository,
};

#[derive(Clone)]
pub struct TodoService {
    repo: TodoRepository,
}

impl TodoService {
    pub fn new(repo: TodoRepository) -> Self {
        Self { repo }
    }

    pub async fn list_for_user(&self, user_id: &Uuid) -> Result<Vec<Todo>, sqlx::Error> {
        self.repo.list_for_user(user_id).await
    }

    pub async fn create(&self, user_id: &Uuid, title: String) -> Result<Todo, Error> {
        let payload = CreateTodo { title };
        self.repo.create(user_id, payload).await
    }

    pub async fn mark_done(&self, id: &Uuid) -> Result<Todo, Error> {
        self.repo.mark_done(id).await
    }

    pub async fn mark_undone(&self, id: &Uuid) -> Result<Todo, Error> {
        self.repo.mark_undone(id).await
    }

    pub async fn delete(&self, id: &Uuid) -> Result<u64, Error> {
        self.repo.delete(id).await
    }
}
