use axum::{
    extract::Path,
    extract::State,
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    Json,
};
use std::fs;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    data::image_processing::JobStatus,
    domain::AppState,
    processing::{process_stitch, process_stitch_sync},
    web::{templates, JobStatusResponse, JobSubmitResponse, StitchRequest},
};

/// Submit images for async processing
pub async fn submit_stitch_job(
    State(state): State<AppState>,
    Json(data): Json<StitchRequest>,
) -> Result<Json<JobSubmitResponse>, StatusCode> {
    let image_count = data.images.len() as i32;

    // Create a job in the database
    let job = state
        .image_processing_service
        .create_job(image_count)
        .await
        .map_err(|e| {
            error!("Failed to create job: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let job_id = job.id;
    info!("Created image processing job: {}", job_id);

    // Spawn background task for processing
    let service = state.image_processing_service.clone();
    tokio::spawn(async move {
        info!("Starting background processing for job: {}", job_id);

        // Mark as processing
        if let Err(e) = service.mark_processing(job_id).await {
            error!("Failed to mark job {} as processing: {}", job_id, e);
            return;
        }

        // Process the images
        match process_stitch(data.images).await {
            Ok(result_path) => {
                info!(
                    "Job {} completed successfully, result: {}",
                    job_id, result_path
                );
                if let Err(e) = service.mark_completed(job_id, result_path).await {
                    error!("Failed to mark job {} as completed: {}", job_id, e);
                }
            }
            Err(e) => {
                error!("Job {} failed: {}", job_id, e);
                if let Err(e) = service.mark_failed(job_id, e).await {
                    error!("Failed to mark job {} as failed: {}", job_id, e);
                }
            }
        }
    });

    Ok(Json(JobSubmitResponse {
        job_id,
        status: "pending".to_string(),
        url: format!("/wait/{}", job_id),
    }))
}

/// Get job status
pub async fn get_job_status(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<JobStatusResponse>, StatusCode> {
    let job = state
        .image_processing_service
        .get_job(job_id)
        .await
        .map_err(|e| {
            error!("Failed to get job {}: {}", job_id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            error!("Job {} not found", job_id);
            StatusCode::NOT_FOUND
        })?;

    Ok(Json(JobStatusResponse {
        job_id: job.id,
        status: job.status.to_string(),
        image_count: job.image_count,
        result_path: job.result_path,
        error_message: job.error_message,
    }))
}

/// Get the processed image
pub async fn get_job_result(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<Response, StatusCode> {
    let job = state
        .image_processing_service
        .get_job(job_id)
        .await
        .map_err(|e| {
            error!("Failed to get job {}: {}", job_id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            error!("Job {} not found", job_id);
            StatusCode::NOT_FOUND
        })?;

    match job.status {
        JobStatus::Completed => {
            let result_path = job.result_path.ok_or_else(|| {
                error!("Job {} completed but no result path", job_id);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            let image_data = fs::read(&result_path).map_err(|e| {
                error!("Failed to read result file {}: {}", result_path, e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "image/png")
                .body(image_data.into())
                .map_err(|e| {
                    error!("Failed to build response: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?)
        }
        JobStatus::Failed => {
            error!("Job {} failed", job_id);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        _ => {
            error!(
                "Job {} is not yet completed (status: {})",
                job_id, job.status
            );
            Err(StatusCode::CONFLICT)
        }
    }
}

/// Legacy synchronous endpoint - now deprecated
pub async fn stitch_images(
    State(_state): State<AppState>,
    Json(data): Json<StitchRequest>,
) -> Result<Response, StatusCode> {
    match process_stitch_sync(data.images).await {
        Ok(png_data) => {
            info!("Image stitching completed successfully");
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "image/png")
                .body(png_data.into())
                .map_err(|e| {
                    error!("Failed to build response: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?)
        }
        Err(e) => {
            error!("Image stitching failed: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

/// Serve waiting page with loading spinner
pub async fn serve_waiting_page(Path(job_id): Path<Uuid>) -> Html<String> {
    Html(templates::waiting_page(job_id))
}

/// Get status fragment for HTMX polling
pub async fn get_status_fragment(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Response {
    let job = match state.image_processing_service.get_job(job_id).await {
        Ok(Some(job)) => job,
        Ok(None) => {
            return Html(templates::status_not_found()).into_response();
        }
        Err(e) => {
            error!("Failed to get job {}: {}", job_id, e);
            return Html(templates::status_error()).into_response();
        }
    };

    match job.status {
        JobStatus::Pending => Html(templates::status_pending()).into_response(),
        JobStatus::Processing => {
            Html(templates::status_processing(job.image_count)).into_response()
        }
        JobStatus::Completed | JobStatus::Failed => {
            // Redirect to image page (handles both success and error)
            Response::builder()
                .status(StatusCode::OK)
                .header("HX-Redirect", format!("/images/{}", job_id))
                .body(String::new().into())
                .unwrap()
        }
    }
}

/// Serve image or error page
pub async fn serve_image_page(State(state): State<AppState>, Path(job_id): Path<Uuid>) -> Response {
    let job = match state.image_processing_service.get_job(job_id).await {
        Ok(Some(job)) => job,
        Ok(None) => {
            return Html(templates::job_not_found(job_id)).into_response();
        }
        Err(e) => {
            error!("Failed to get job {}: {}", job_id, e);
            return Html(templates::error_page(
                "Error",
                "Failed to retrieve job information",
                None,
            ))
            .into_response();
        }
    };

    match job.status {
        JobStatus::Completed => {
            let result_path = match job.result_path {
                Some(path) => path,
                None => {
                    error!("Job {} completed but no result path", job_id);
                    return Html(templates::error_page(
                        "Error",
                        "Image processing completed but result file is missing",
                        None,
                    ))
                    .into_response();
                }
            };

            let image_data = match fs::read(&result_path) {
                Ok(data) => data,
                Err(e) => {
                    error!("Failed to read result file {}: {}", result_path, e);
                    return Html(templates::error_page(
                        "Error",
                        "Failed to read image file",
                        Some(&e.to_string()),
                    ))
                    .into_response();
                }
            };

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "image/png")
                .header(header::CACHE_CONTROL, "public, max-age=31536000")
                .body(image_data.into())
                .unwrap()
        }
        JobStatus::Failed => {
            let error_message = job
                .error_message
                .unwrap_or_else(|| "Unknown error".to_string());
            Html(templates::processing_failed(&error_message)).into_response()
        }
        _ => {
            // Still processing, redirect back to waiting page
            Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header(header::LOCATION, format!("/wait/{}", job_id))
                .body(String::new().into())
                .unwrap()
        }
    }
}
