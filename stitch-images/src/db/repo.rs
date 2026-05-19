use sqlx::{Pool, Postgres, Transaction};
use uuid::Uuid;

use crate::db::models::{JobStatus, ProcessingJob};

pub struct JobRepo {
    pool: Pool<Postgres>,
}

impl JobRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_job(&self, image_count: u32) -> Result<Uuid, sqlx::Error> {
        let image_count = image_count as i32;

        sqlx::query_scalar!(
            r#"
                INSERT INTO image_processing_jobs (image_count)
                VALUES ($1)
                RETURNING id
            "#,
            image_count
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_job(&self, job_id: Uuid) -> Result<ProcessingJob, sqlx::Error> {
        sqlx::query_as!(
            ProcessingJob,
            r#"
                SELECT
                    id,
                    created_at,
                    updated_at,
                    status as "status: JobStatus",
                    image_count,
                    submitted_count,
                    error_message
                FROM image_processing_jobs
                WHERE id = $1
            "#,
            job_id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn increment_submitted_count(
        &self,
        job_id: Uuid,
    ) -> Result<(bool, Transaction<'_, Postgres>), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        let is_complete = sqlx::query_scalar!(
            r#"
                UPDATE image_processing_jobs
                SET submitted_count = submitted_count + 1
                WHERE id = $1
                RETURNING (submitted_count = image_count) as "is_complete!"
            "#,
            job_id
        )
        .fetch_one(&mut *transaction)
        .await?;

        if is_complete {
            sqlx::query!(
                r#"
                    UPDATE image_processing_jobs
                    SET status = 'submission_complete'
                    WHERE id = $1
                "#,
                job_id
            ).fetch_one(&mut *transaction).await?;
        }

        Ok((is_complete, transaction))
    }

    pub async fn set_failed(&self, job_id: Uuid, error_message: String) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"
            UPDATE image_processing_jobs
            SET status = 'failed', error_message = $1
            WHERE id = $2
        "#, error_message, job_id)
        .fetch_one(&self.pool).await.map(|_| ())
    }
}
