use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, types::chrono::NaiveDateTime};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProcessingJob {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: JobStatus,
    pub image_count: i32,
    pub submitted_count: i32,
    pub error_message: Option<String>
}

impl ProcessingJob {
    pub fn should_accept(&self, &idx: &u32) -> bool {
        self.status == JobStatus::Created
            && self.submitted_count < self.image_count
            && (idx as i32) < self.image_count
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Type, Deserialize, Serialize)]
#[sqlx(type_name = "job_status", rename_all = "lowercase")]
pub enum JobStatus {
    Created,
    SubmissionComplete,
    Processing,
    Completed,
    Failed,
}
