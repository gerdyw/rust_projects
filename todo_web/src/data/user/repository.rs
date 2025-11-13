use sqlx::{Pool, Postgres};
use tracing::debug;
use uuid::Uuid;

use crate::{data::user::models::User, db::errors::RepoError};

#[derive(Clone)]
pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<User, RepoError> {
        let user = sqlx::query_as::<_, super::models::User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn find_user_by_id(&self, id: Uuid) -> Result<User, RepoError> {
        let user = sqlx::query_as::<_, super::models::User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn create_user(&self, email: &str) -> Result<User, RepoError> {
        let user = sqlx::query_as::<_, super::models::User>(
            r#"
            INSERT INTO users (email)
            VALUES ($1)
            RETURNING *
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

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
