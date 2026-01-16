use crate::data::image_processing::models::{
    CreateImageProcessingJob, ImageProcessingJob, JobStatus, UpdateImageProcessingJob,
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct ImageProcessingRepository {
    pool: PgPool,
}

impl ImageProcessingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, job: CreateImageProcessingJob) -> Result<ImageProcessingJob, sqlx::Error> {
        let record = sqlx::query_as!(
            ImageProcessingJob,
            r#"
            INSERT INTO image_processing_jobs (image_count, status)
            VALUES ($1, 'pending')
            RETURNING 
                id, 
                created_at, 
                updated_at, 
                status as "status: JobStatus", 
                image_count, 
                result_path, 
                error_message
            "#,
            job.image_count
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<ImageProcessingJob>, sqlx::Error> {
        let record = sqlx::query_as!(
            ImageProcessingJob,
            r#"
            SELECT 
                id, 
                created_at, 
                updated_at, 
                status as "status: JobStatus", 
                image_count, 
                result_path, 
                error_message
            FROM image_processing_jobs
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn update(&self, id: Uuid, update: UpdateImageProcessingJob) -> Result<ImageProcessingJob, sqlx::Error> {
        let status_str = update.status.to_string();
        let record = sqlx::query_as!(
            ImageProcessingJob,
            r#"
            UPDATE image_processing_jobs
            SET 
                status = $2,
                result_path = $3,
                error_message = $4,
                updated_at = NOW()
            WHERE id = $1
            RETURNING 
                id, 
                created_at, 
                updated_at, 
                status as "status: JobStatus", 
                image_count, 
                result_path, 
                error_message
            "#,
            id,
            status_str,
            update.result_path,
            update.error_message
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }
}
