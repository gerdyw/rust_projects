use rocket::{Request, http::Status, request::{FromRequest, Outcome}};
use serde::{Deserialize, Serialize};
use sqlx::{Error, prelude::Type, types::chrono::NaiveDateTime};
use uuid::Uuid;

use crate::db::repo::JobRepo;

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

#[derive(Clone, Debug, PartialEq, PartialOrd, Type, Deserialize, Serialize)]
#[sqlx(type_name = "job_status", rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Pending => write!(f, "pending"),
            JobStatus::Processing => write!(f, "processing"),
            JobStatus::Completed => write!(f, "completed"),
            JobStatus::Failed => write!(f, "failed"),
        }
    }
}

impl std::str::FromStr for JobStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(JobStatus::Pending),
            "processing" => Ok(JobStatus::Processing),
            "completed" => Ok(JobStatus::Completed),
            "failed" => Ok(JobStatus::Failed),
            _ => Err(format!("Invalid job status: {}", s)),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ProcessingJob {
    type Error = Status;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let result: Result<ProcessingJob, Status> = async {
            let job_id_header = request.headers().get_one("x-job-id").ok_or(Status::BadRequest)?;
            let job_id = Uuid::parse_str(job_id_header).map_err(|_| Status::BadRequest)?;
            let repo = request.rocket().state::<JobRepo>().ok_or(Status::InternalServerError)?;

            repo.get_job(job_id).await.map_err(|err| match err {
                Error::RowNotFound => Status::NotFound,
                _ => {
                    eprintln!("sqlx error: {}", err);
                    Status::InternalServerError
                }
            })
        }
        .await;

        match result {
            Ok(job) => Outcome::Success(job),
            Err(status) => Outcome::Error((status, status)),
        }
    }
}