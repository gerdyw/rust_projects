use crate::data::image_processing::models::{
    CreateImageProcessingJob, ImageProcessingJob, JobStatus, UpdateImageProcessingJob,
};
use sqlx::PgPool;
use uuid::Uuid;
use std::str::FromStr;

#[derive(Clone)]
pub struct ImageProcessingRepository {
    pool: PgPool,
}

fn parse_job_status(status_str: &str) -> Result<JobStatus, sqlx::Error> {
    JobStatus::from_str(status_str).map_err(|e| {
        sqlx::Error::Decode(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            e,
        )))
    })
}

impl ImageProcessingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, job: CreateImageProcessingJob) -> Result<ImageProcessingJob, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            INSERT INTO image_processing_jobs (image_count, status)
            VALUES ($1, 'pending')
            RETURNING 
                id, 
                created_at, 
                updated_at, 
                status, 
                image_count, 
                result_path, 
                error_message
            "#,
            job.image_count
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(ImageProcessingJob {
            id: record.id,
            created_at: record.created_at,
            updated_at: record.updated_at,
            status: parse_job_status(&record.status)?,
            image_count: record.image_count,
            result_path: record.result_path,
            error_message: record.error_message,
        })
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<ImageProcessingJob>, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            SELECT 
                id, 
                created_at, 
                updated_at, 
                status, 
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

        match record {
            Some(rec) => Ok(Some(ImageProcessingJob {
                id: rec.id,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                status: parse_job_status(&rec.status)?,
                image_count: rec.image_count,
                result_path: rec.result_path,
                error_message: rec.error_message,
            })),
            None => Ok(None),
        }
    }

    pub async fn update(&self, id: Uuid, update: UpdateImageProcessingJob) -> Result<ImageProcessingJob, sqlx::Error> {
        let status_str = update.status.to_string();
        let record = sqlx::query!(
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
                status, 
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

        Ok(ImageProcessingJob {
            id: record.id,
            created_at: record.created_at,
            updated_at: record.updated_at,
            status: parse_job_status(&record.status)?,
            image_count: record.image_count,
            result_path: record.result_path,
            error_message: record.error_message,
        })
    }
}
