use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::db::models::{JobStatus, ProcessingJob};

pub struct JobRepo {
    pool: Pool<Postgres>
}

impl JobRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_job(&self, image_count: u32) -> Result<Uuid, sqlx::Error> {
        let image_count = image_count as i32;

        sqlx::query_scalar!(r#"
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
}