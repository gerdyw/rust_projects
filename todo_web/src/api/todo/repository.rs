use sqlx::{Pool, Postgres};
use tracing::{debug, error};
use uuid::Uuid;

use crate::db::errors::RepoError;

use super::models::{CreateTodo, Todo};

#[derive(Clone)]
pub struct TodoRepository {
    pool: Pool<Postgres>,
}

impl TodoRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    /// List all todos
    pub async fn list(&self) -> sqlx::Result<Vec<Todo>> {
        debug!("Listing all todos");
        sqlx::query_as::<_, Todo>("SELECT * FROM todo_items ORDER BY done DESC, created_at ASC")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn list_for_user(&self, user_id: Uuid) -> Result<Vec<Todo>, RepoError> {
        debug!("Listing todos for user {}", user_id);
        sqlx::query_as::<_, Todo>(
            "SELECT * FROM todo_items WHERE user_id = $1 ORDER BY done DESC, created_at ASC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.into())
        .inspect_err(|err| match err {
            RepoError::DbError(e) => error!("Database error: {}", e),
            _ => (),
        })
    }

    /// Create a new todo
    pub async fn create(&self, user_id: Uuid, payload: CreateTodo) -> Result<Todo, RepoError> {
        debug!("Creating new todo for user {}", user_id);
        sqlx::query_as::<_, Todo>(
            r#"
            INSERT INTO todo_items (user_id, title)
            VALUES ($1, $2)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(payload.title)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.into())
        .inspect_err(|err| match err {
            RepoError::DbError(e) => error!("Database error: {}", e),
            _ => (),
        })
    }

    pub async fn mark_done(&self, id: Uuid) -> Result<(), RepoError> {
        debug!("Marking todo as done with id: {}", id);
        let result = sqlx::query("UPDATE todo_items SET done = true WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            Err(RepoError::NotFound)
        } else {
            Ok(())
        }
    }

    /// Mark a todo as not done
    pub async fn mark_undone(&self, id: Uuid) -> Result<(), RepoError> {
        debug!("Marking todo as not done with id: {}", id);
        let result = sqlx::query("UPDATE todo_items SET done = false WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            Err(RepoError::NotFound)
        } else {
            Ok(())
        }
    }

    /// Delete a todo
    pub async fn delete(&self, id: Uuid) -> Result<(), RepoError> {
        debug!("Deleting todo with id: {}", id);
        let result = sqlx::query("DELETE FROM todo_items WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            Err(RepoError::NotFound)
        } else {
            Ok(())
        }
    }
}
