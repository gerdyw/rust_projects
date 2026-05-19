use rocket::fs::TempFile;
use rocket::http::{ContentType, Status};
use rocket::response::status::Accepted;
use rocket::{Route, State, get, post};
use rocket::serde::json::Json;
use sqlx::Error::RowNotFound;
use crate::api::models::{CreateJobResponse, JobId};
use crate::db::models::{JobStatus, ProcessingJob};
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
        RowNotFound => Status::NotFound,
        _ => {
            eprintln!("{}", err);
            Status::InternalServerError
        }
    })
}

#[post("/submit-image/<job_id>/<idx>", data = "<image>")]
pub async fn submit_image(repo: &State<JobRepo>, job_id: JobId, idx: u32, mut image: TempFile<'_>) -> Result<Accepted<()>, Status> {
    let JobId(job_id) = job_id;
    let job = repo.get_job(job_id).await.map_err(|err| match err {
        RowNotFound => Status::NotFound,
        _ => {
            eprintln!("{}", err);
            Status::InternalServerError
        }
    })?;

    if !job.should_accept(&idx) {
        return Err(Status::BadRequest);
    };

    let (is_complete, transaction) = repo.increment_submitted_count(job_id).await.map_err(|err| {
        eprintln!("{}", err);
        Status::InternalServerError
    })?;

    let file_type = image.content_type().unwrap_or(&ContentType::JPEG).to_owned();
    // let extension = if file_type.is_jpeg() {
    //     ".jpeg"
    // } else if file_type.is_png() {
    //     ".png"
    // } else {
    //     return Err(Status::BadRequest)
    // };

    let extension = "";
    let path = format!("./temp_images/{}-{}{}", job_id, idx, extension);

    image.persist_to(path).await.map_err(|err| {
        eprintln!("{}", err);
        Status::InternalServerError
    })?;

    transaction.commit().await.map_err(|err| {
        eprintln!("{}", err);
        Status::InternalServerError
    })?;

    Ok(Accepted(()))
}

pub fn routes() -> Vec<Route> {
    rocket::routes![create_job, get_job, submit_image]
}
