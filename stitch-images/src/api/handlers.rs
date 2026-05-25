use crate::api::models::{CreateJobResponse, CreateJobResult, GetJobResult, JobId, SubmitImageResult};
use crate::db::models::ProcessingJob;
use crate::persistence::ImageManager;
use crate::processing::processor::process_stitch;
use crate::{api::models::CreateJob, db::repo::JobRepo};
use rocket::fs::TempFile;
use rocket::response::status::Accepted;
use rocket::serde::json::Json;
use rocket::tokio::task;
use rocket::{Route, State, get, post};
use sqlx::Error::RowNotFound;

#[post("/create", data = "<job_data>")]
pub async fn create_job(
    repo: &State<JobRepo>,
    job_data: Json<CreateJob>,
) -> CreateJobResult {
    match repo.create_job(job_data.image_count).await {
        Ok(job_id) => CreateJobResult::Created(Json(CreateJobResponse { job_id })),
        Err(err) => CreateJobResult::InternalError(err.to_string()),
    }
}

#[get("/job/<job_id>")]
pub async fn get_job(repo: &State<JobRepo>, job_id: JobId) -> GetJobResult {
    let JobId(job_id) = job_id;
    match repo.get_job(job_id).await {
        Ok(job) => GetJobResult::Found(Json(job)),
        Err(RowNotFound) => GetJobResult::NotFound(()),
        Err(err) => {
            eprintln!("{}", err);
            GetJobResult::InternalError(err.to_string())
        }
    }
}

#[post("/submit-image/<job_id>/<idx>", data = "<image>")]
pub async fn submit_image(
    repo: &State<JobRepo>,
    image_manager: &State<ImageManager>,
    job_id: JobId,
    idx: u32,
    mut image: TempFile<'_>,
) -> SubmitImageResult {
    let JobId(job_id) = job_id;
    let job = match repo.get_job(job_id).await {
        Ok(job) => job,
        Err(RowNotFound) => return SubmitImageResult::NotFound(()),
        Err(err) => {
            eprintln!("{}", err);
            return SubmitImageResult::InternalError(err.to_string());
        }
    };

    match job {
        ProcessingJob::Created {
            id: _,
            image_count,
            submitted_count: _,
        } if idx < image_count => (),
        _ => return SubmitImageResult::BadRequest(()),
    }

    let (job, tx) = match repo.increment_submitted_count(job_id).await {
        Ok(result) => result,
        Err(err) => {
            eprintln!("{}", err);
            return SubmitImageResult::InternalError(err.to_string());
        }
    };

    if let Err(err) = image_manager.save_temp_image(job.id(), &mut image, idx).await {
        eprintln!("{}", err);
        return SubmitImageResult::InternalError(err.to_string());
    }

    if let Err(err) = tx.commit().await {
        eprintln!("{}", err);
        return SubmitImageResult::InternalError(err.to_string());
    }

    match job {
        ProcessingJob::SubmissionComplete { id, submitted_count } => {
            println!("Submission complete, starting stitch");
            let repo = repo.inner().clone();
            let image_manager = image_manager.inner().clone();
            task::spawn(async move {
                process_stitch(repo, image_manager, id, submitted_count).await;
            });
        }
        _ => (),
    }

    SubmitImageResult::Submitted(Accepted(()))
}

pub fn api_routes() -> Vec<Route> {
    rocket::routes![create_job, get_job, submit_image]
}
