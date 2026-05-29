use crate::api::models::{
    CreateJobResponse, CreateJobResult, GetJobResult, JobId, SubmitImageResult,
};
use crate::db::models::ProcessingJob;
use crate::persistence::ImageManager;
use crate::processing::processor::process_stitch;
use crate::telemetry::TelemetryMetrics;
use crate::{api::models::CreateJob, db::repo::JobRepo};
use rocket::fs::TempFile;
use rocket::response::status::Accepted;
use rocket::serde::json::Json;
use rocket::tokio::task;
use rocket::{Route, State, get, post};
use sqlx::Error::RowNotFound;
use std::time::Instant;
use tracing::{Instrument, error, info, instrument};

#[post("/create", data = "<job_data>")]
#[instrument(skip(repo, job_data), fields(otel.kind = "server", image_count = job_data.image_count))]
pub async fn create_job(repo: &State<JobRepo>, job_data: Json<CreateJob>) -> CreateJobResult {
    match repo.create_job(job_data.image_count).await {
        Ok(job_id) => {
            info!(%job_id, "created image stitching job");
            CreateJobResult::Created(Json(CreateJobResponse { job_id }))
        }
        Err(err) => {
            error!(error = %err, "failed to create image stitching job");
            CreateJobResult::InternalError(err.to_string())
        }
    }
}

#[get("/job/<job_id>")]
#[instrument(skip(repo), fields(otel.kind = "server", job_id = %job_id.0))]
pub async fn get_job(repo: &State<JobRepo>, job_id: JobId) -> GetJobResult {
    let JobId(job_id) = job_id;
    match repo.get_job(job_id).await {
        Ok(job) => {
            info!(status = tracing::field::debug(&job), "retrieved job status");
            GetJobResult::Found(Json(job))
        }
        Err(RowNotFound) => GetJobResult::NotFound(()),
        Err(err) => {
            error!(error = %err, "failed to retrieve job");
            GetJobResult::InternalError(err.to_string())
        }
    }
}

#[post("/submit-image/<job_id>/<idx>", data = "<image>")]
#[instrument(
    skip(repo, image_manager, image, telemetry),
    fields(otel.kind = "server", job_id = %job_id.0, image_index = idx)
)]
pub async fn submit_image(
    repo: &State<JobRepo>,
    image_manager: &State<ImageManager>,
    telemetry: &State<TelemetryMetrics>,
    job_id: JobId,
    idx: u32,
    mut image: TempFile<'_>,
) -> SubmitImageResult {
    let started_at = Instant::now();
    let JobId(job_id) = job_id;
    let job = match repo.get_job(job_id).await {
        Ok(job) => job,
        Err(RowNotFound) => return SubmitImageResult::NotFound(()),
        Err(err) => {
            error!(error = %err, "failed to load job before image submission");
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
            error!(error = %err, "failed to increment submission count");
            return SubmitImageResult::InternalError(err.to_string());
        }
    };

    if let Err(err) = image_manager
        .save_temp_image(job.id(), &mut image, idx)
        .await
    {
        error!(error = %err, "failed to persist submitted image");
        return SubmitImageResult::InternalError(err.to_string());
    }

    if let Err(err) = tx.commit().await {
        error!(error = %err, "failed to commit submission transaction");
        return SubmitImageResult::InternalError(err.to_string());
    }

    match job {
        ProcessingJob::SubmissionComplete {
            id,
            submitted_count,
        } => {
            info!(%id, submitted_count, "submission complete; starting image stitch");
            let repo = repo.inner().clone();
            let image_manager = image_manager.inner().clone();
            let telemetry = telemetry.inner().clone();
            task::spawn(
                async move {
                    process_stitch(repo, image_manager, telemetry, id, submitted_count).await;
                }
                .instrument(tracing::info_span!(
                    "job_processing_task",
                    job_id = %id,
                    image_count = submitted_count
                )),
            );
        }
        _ => (),
    }

    telemetry.record_submission_duration(started_at.elapsed(), "accepted");
    info!("accepted submitted image");
    SubmitImageResult::Submitted(Accepted(()))
}

pub fn api_routes() -> Vec<Route> {
    rocket::routes![create_job, get_job, submit_image]
}
