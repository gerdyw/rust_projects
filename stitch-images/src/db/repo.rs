use sqlx::{Pool, Postgres, Transaction, postgres::PgQueryResult};
use uuid::Uuid;

use crate::db::models::{JobStatus, ProcessingJob, ProcessingJobEntity};

#[derive(Clone)]
pub struct JobRepo {
    pool: Pool<Postgres>,
}

impl JobRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_job(&self, image_count: u32) -> Result<Uuid, sqlx::Error> {
        let image_count = image_count as i32;
        let id = Uuid::now_v7();
        sqlx::query_scalar!(
            r#"
                INSERT INTO image_processing_jobs (id, image_count)
                VALUES ($1, $2)
                RETURNING id
            "#,
            id,
            image_count
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_job(&self, job_id: Uuid) -> Result<ProcessingJob, sqlx::Error> {
        let job = sqlx::query_as!(
            ProcessingJobEntity,
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
        .await?;
        Ok(job.into())
    }

    pub async fn increment_submitted_count(
        &self,
        job_id: Uuid,
    ) -> Result<(ProcessingJob, Transaction<'_, Postgres>), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let entity = sqlx::query_as!(
            ProcessingJobEntity,
            r#"
                UPDATE image_processing_jobs
                SET submitted_count = submitted_count + 1
                WHERE id = $1
                RETURNING id,
                    created_at,
                    updated_at,
                    status as "status: JobStatus",
                    image_count,
                    submitted_count,
                    error_message
            "#,
            job_id
        )
        .fetch_one(&mut *tx)
        .await?;

        let entity = if entity.submitted_count == entity.image_count {
            sqlx::query_as!(
                ProcessingJobEntity,
                r#"
                UPDATE image_processing_jobs
                SET status = 'submission_complete'
                WHERE id = $1
                RETURNING id,
                    created_at,
                    updated_at,
                    status as "status: JobStatus",
                    image_count,
                    submitted_count,
                    error_message "#,
                job_id
            )
            .fetch_one(&mut *tx)
            .await?
        } else {
            entity
        };

        Ok((entity.into(), tx))
    }

    pub async fn set_failed(
        &self,
        job_id: Uuid,
        error_message: String,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE image_processing_jobs
            SET status = 'failed', error_message = $1
            WHERE id = $2
        "#,
            error_message,
            job_id
        )
        .execute(&self.pool)
        .await
    }

    pub async fn set_completed(&self, job_id: Uuid) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            r#"
                UPDATE image_processing_jobs
                SET status = 'completed'
                WHERE id = $1
                AND submitted_count = image_count
            "#,
            job_id
        )
        .execute(&self.pool)
        .await
    }
}
