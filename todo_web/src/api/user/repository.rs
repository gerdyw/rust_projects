use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn find_user_by_email(
        &self,
        email: &str,
    ) -> sqlx::Result<Option<super::models::User>> {
        sqlx::query_as::<_, super::models::User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn find_user_by_id(&self, id: &Uuid) -> sqlx::Result<Option<super::models::User>> {
        sqlx::query_as::<_, super::models::User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn create_user(&self, email: &str) -> sqlx::Result<super::models::User> {
        sqlx::query_as::<_, super::models::User>(
            r#"
            INSERT INTO users (email)
            VALUES ($1)
            RETURNING *
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_user(&self, email: &str) -> sqlx::Result<u64> {
        let result = sqlx::query("DELETE FROM users WHERE email = $1")
            .bind(email)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
