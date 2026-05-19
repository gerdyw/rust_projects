use rocket::http::Status;
use rocket::{Route, State, get, post};
use rocket::serde::json::Json;
use crate::api::models::{CreateJobResponse, JobId};
use crate::db::models::ProcessingJob;
use crate::{api::models::CreateJob, db::repo::JobRepo};

#[post("/create", data = "<job_data>")]
pub async fn create_job(repo: &State<JobRepo>, job_data: Json<CreateJob>) -> Result<Json<CreateJobResponse>, String> {
     repo
        .create_job(job_data.image_count)
        .await
        .map(|job_id| Json(CreateJobResponse { job_id }))
        .map_err(|e| e.to_string())
}

#[get("/<job_id>")]
pub async fn get_job(repo: &State<JobRepo>, job_id: JobId) -> Result<Json<ProcessingJob>, Status> {
    let JobId(job_id) = job_id;
    repo.get_job(job_id).await
        .map(Json)
        .map_err(|err| match err {
        sqlx::Error::RowNotFound => Status::NotFound,
        _ => {
            eprintln!("{}", err);
            Status::InternalServerError
        }
    })
}

pub fn routes() -> Vec<Route> {
    rocket::routes![create_job, get_job]
}
