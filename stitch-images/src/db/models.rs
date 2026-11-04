use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, types::chrono::NaiveDateTime};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProcessingJobEntity {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: JobStatus,
    pub image_count: i32,
    pub submitted_count: i32,
    pub error_message: Option<String>,
}

impl ProcessingJobEntity {
    pub fn should_accept(&self, &idx: &u32) -> bool {
        self.status == JobStatus::Created
            && self.submitted_count < self.image_count
            && (idx as i32) < self.image_count
    }
}

impl Into<ProcessingJob> for ProcessingJobEntity {
    fn into(self) -> ProcessingJob {
        match self.status {
            JobStatus::Created => ProcessingJob::Created {
                id: self.id,
                image_count: self.image_count as u32,
                submitted_count: self.submitted_count as u32,
            },
            JobStatus::SubmissionComplete => ProcessingJob::SubmissionComplete {
                id: self.id,
                submitted_count: self.submitted_count as u32,
            },
            JobStatus::Processing => ProcessingJob::Processing {
                id: self.id,
            },
            JobStatus::Completed => ProcessingJob::Completed {
                id: self.id,
            },
            JobStatus::Failed => ProcessingJob::Failed {
                id: self.id,
                error_message: self.error_message.unwrap_or_else(|| "Unknown error".to_string()),
            },
        }
    }
}


#[derive(Clone, Debug, PartialEq, PartialOrd, Type, Deserialize, Serialize)]
#[sqlx(type_name = "job_status", rename_all = "snake_case")]
pub enum JobStatus {
    Created,
    SubmissionComplete,
    Processing,
    Completed,
    Failed,
}

#[derive(Serialize)]
pub enum ProcessingJob {
    Created {
        id: Uuid,
        image_count: u32,
        submitted_count: u32,
    },
    SubmissionComplete {
        id: Uuid,
        submitted_count: u32,
    },
    Processing {
        id: Uuid,
    },
    Completed {
        id: Uuid,
    },
    Failed {
        id: Uuid,
        error_message: String,
    },
}

impl ProcessingJob {
    pub fn id(&self) -> Uuid {
        match self {
            Self::Created {
                id,
                image_count: _,
                submitted_count: _,
            } => *id,
            Self::SubmissionComplete {
                id,
                submitted_count: _,
            } => *id,
            Self::Processing { id } => *id,
            Self::Completed { id } => *id,
            Self::Failed {
                id,
                error_message: _,
            } => *id,
        }
    }

    pub fn should_accept(&self) -> bool {
        matches!(self, ProcessingJob::Created { id: _, image_count: _, submitted_count: _ })
    }
}
