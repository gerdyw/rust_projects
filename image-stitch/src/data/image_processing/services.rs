use crate::data::image_processing::models::{
    CreateImageProcessingJob, ImageProcessingJob, JobStatus, UpdateImageProcessingJob,
};
use crate::data::image_processing::repository::ImageProcessingRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct ImageProcessingService {
    repository: ImageProcessingRepository,
}

impl ImageProcessingService {
    pub fn new(repository: ImageProcessingRepository) -> Self {
        Self { repository }
    }

    pub async fn create_job(&self, image_count: i32) -> Result<ImageProcessingJob, sqlx::Error> {
        let job = CreateImageProcessingJob { image_count };
        self.repository.create(job).await
    }

    pub async fn get_job(&self, id: Uuid) -> Result<Option<ImageProcessingJob>, sqlx::Error> {
        self.repository.get_by_id(id).await
    }

    pub async fn update_job_status(
        &self,
        id: Uuid,
        status: JobStatus,
        result_path: Option<String>,
        error_message: Option<String>,
    ) -> Result<ImageProcessingJob, sqlx::Error> {
        let update = UpdateImageProcessingJob {
            status,
            result_path,
            error_message,
        };
        self.repository.update(id, update).await
    }

    pub async fn mark_processing(&self, id: Uuid) -> Result<ImageProcessingJob, sqlx::Error> {
        self.update_job_status(id, JobStatus::Processing, None, None)
            .await
    }

    pub async fn mark_completed(
        &self,
        id: Uuid,
        result_path: String,
    ) -> Result<ImageProcessingJob, sqlx::Error> {
        self.update_job_status(id, JobStatus::Completed, Some(result_path), None)
            .await
    }

    pub async fn mark_failed(
        &self,
        id: Uuid,
        error_message: String,
    ) -> Result<ImageProcessingJob, sqlx::Error> {
        self.update_job_status(id, JobStatus::Failed, None, Some(error_message))
            .await
    }
}
