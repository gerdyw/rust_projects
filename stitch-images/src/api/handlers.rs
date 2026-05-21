use crate::api::models::{CreateJobResponse, JobId};
use crate::db::models::ProcessingJob;
use crate::persistence::ImageManager;
use crate::processing::processor::process_stitch;
use crate::{api::models::CreateJob, db::repo::JobRepo};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::response::status::Accepted;
use rocket::serde::json::Json;
use rocket::tokio::task;
use rocket::{Route, State, get, post};
use sqlx::Error::RowNotFound;

#[post("/create", data = "<job_data>")]
pub async fn create_job(
    repo: &State<JobRepo>,
    job_data: Json<CreateJob>,
) -> Result<Json<CreateJobResponse>, String> {
    repo.create_job(job_data.image_count)
        .await
        .map(|job_id| Json(CreateJobResponse { job_id }))
        .map_err(|e| e.to_string())
}

#[get("/<job_id>")]
pub async fn get_job(repo: &State<JobRepo>, job_id: JobId) -> Result<Json<ProcessingJob>, Status> {
    let JobId(job_id) = job_id;
    repo.get_job(job_id)
        .await
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
pub async fn submit_image(
    repo: &State<JobRepo>,
    image_manager: &State<ImageManager>,
    job_id: JobId,
    idx: u32,
    mut image: TempFile<'_>,
) -> Result<Accepted<()>, Status> {
    let JobId(job_id) = job_id;
    let job = repo.get_job(job_id).await.map_err(|err| match err {
        RowNotFound => Status::NotFound,
        _ => {
            eprintln!("{}", err);
            Status::InternalServerError
        }
    })?;

    match job {
        ProcessingJob::Created {
            id: _,
            image_count,
            submitted_count: _,
        } if idx < image_count => (),
        _ => return Err(Status::BadRequest),
    }

    let (job, tx) = repo
        .increment_submitted_count(job_id)
        .await
        .map_err(|err| {
            eprintln!("{}", err);
            Status::InternalServerError
        })?;

    image_manager
        .save_temp_image(job.id(), &mut image, idx)
        .await
        .map_err(|err| {
            eprintln!("{}", err);
            Status::InternalServerError
        })?;

    tx.commit().await.map_err(|err| {
        eprintln!("{}", err);
        Status::InternalServerError
    })?;

    match job {
        ProcessingJob::SubmissionComplete {
            id,
            submitted_count,
        } => {
            println!("Submission complete, starting stitch");
            let repo = repo.inner().clone();
            let image_manager = image_manager.inner().clone();
            task::spawn(async move {
                process_stitch(repo, image_manager, id, submitted_count).await;
            });
        }
        _ => (),
    }

    Ok(Accepted(()))
}

pub fn api_routes() -> Vec<Route> {
    rocket::routes![create_job, get_job, submit_image]
}
