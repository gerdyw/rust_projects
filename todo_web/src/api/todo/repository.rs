use sqlx::{Pool, Postgres};
use tracing::debug;
use uuid::Uuid;

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
        sqlx::query_as::<_, Todo>("SELECT * FROM todo_items ORDER BY done DESC, created_at ASC")
            .fetch_all(&self.pool)
            .await
    }

    /// Create a new todo
    pub async fn create(&self, payload: CreateTodo) -> sqlx::Result<Todo> {
        sqlx::query_as::<_, Todo>(
            r#"
            INSERT INTO todo_items (title)
            VALUES ($1)
            RETURNING *
            "#,
        )
        .bind(payload.title)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn mark_done(&self, id: Uuid) -> sqlx::Result<Todo> {
        debug!("Marking todo as done with id: {}", id);
        sqlx::query_as::<_, Todo>("UPDATE todo_items SET done = true WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    /// Mark a todo as not done
    pub async fn mark_undone(&self, id: Uuid) -> sqlx::Result<u64> {
        let result = sqlx::query("UPDATE todo_items SET done = false WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        debug!("mark_undone rows affected: {}", result.rows_affected());

        Ok(result.rows_affected())
    }

    /// Delete a todo
    pub async fn delete(&self, id: Uuid) -> sqlx::Result<u64> {
        let result = sqlx::query("DELETE FROM todo_items WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        debug!("delete rows affected: {}", result.rows_affected());

        Ok(result.rows_affected())
    }
}
