use super::models::{CreateTodo, Todo};
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct TodoRepository {
    pool: Pool<Sqlite>,
}

impl TodoRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> sqlx::Result<Vec<Todo>> {
        sqlx::query_as::<_, Todo>("SELECT id, title, done FROM todos ORDER BY done DESC, id ASC")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create(&self, payload: CreateTodo) -> sqlx::Result<Todo> {
        sqlx::query_as::<_, Todo>(
            "INSERT INTO todos (title, done) VALUES (?, false) RETURNING id, title, done",
        )
        .bind(payload.title)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn mark_done(&self, id: i64) -> sqlx::Result<Todo> {
        sqlx::query_as::<_, Todo>(
            "UPDATE todos SET done = true WHERE id = ? RETURNING id, title, done",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn mark_undone(&self, id: i64) -> sqlx::Result<Todo> {
        sqlx::query_as::<_, Todo>(
            "UPDATE todos SET done = false WHERE id = ? RETURNING id, title, done",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<u64> {
        let result = sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}
