use axum::{extract::Path, extract::State, http::StatusCode, response::Response, Json};
use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::Local;
use exif::{In, Tag};
use image::{
    codecs::png::{CompressionType, FilterType as PngFilterType, PngEncoder},
    DynamicImage, ExtendedColorType, GenericImage, ImageEncoder,
};
use rayon::prelude::*;
use std::fs;
use std::path::Path as FilePath;
use std::{cmp::min, io::Cursor};
use tokio::task;
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{
    data::image_processing::JobStatus,
    domain::AppState,
    web::{JobStatusResponse, JobSubmitResponse, StitchRequest},
};

const MAX_WIDTH: u32 = 2000;

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
        url: format!("/jobs/{}", job_id),
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

async fn process_stitch(image_data: Vec<String>) -> Result<String, String> {
    debug!("Spawning blocking task for image stitching");
    let result = task::spawn_blocking(move || process_stitch_blocking(image_data))
        .await
        .map_err(|e| format!("Task join error: {}", e))??;

    Ok(result)
}

async fn process_stitch_sync(image_data: Vec<String>) -> Result<Vec<u8>, String> {
    debug!("Spawning blocking task for image stitching");
    let result = task::spawn_blocking(move || process_stitch_blocking_bytes(image_data))
        .await
        .map_err(|e| format!("Task join error: {}", e))??;

    Ok(result)
}

fn process_stitch_blocking(image_data: Vec<String>) -> Result<String, String> {
    debug!(
        "Starting image stitching process with {} images",
        image_data.len()
    );

    let images = decode_images(image_data)?;

    if images.is_empty() {
        return Err("No images provided".to_string());
    }

    // Find narrowest width
    let narrowest_width = images
        .iter()
        .map(|img| img.width())
        .min()
        .ok_or("Failed to determine narrowest width")?;

    debug!("Narrowest width: {}", narrowest_width);

    let desired_width = min(narrowest_width, MAX_WIDTH);
    let needs_scaling = images.iter().any(|img| img.width() != desired_width);

    // Scale all images in parallel across available cores
    let images: Vec<_> = if needs_scaling {
        images
            .into_par_iter()
            .enumerate()
            .map(|(idx, img)| {
                if img.width() == desired_width {
                    debug!("Image {} already at desired width {}", idx, desired_width);
                    return img;
                }

                let scale_factor = desired_width as f32 / img.width() as f32;
                let new_height = (img.height() as f32 * scale_factor) as u32;
                debug!("Scaling image {} to {}x{}", idx, desired_width, new_height);
                img.resize_exact(
                    desired_width,
                    new_height,
                    image::imageops::FilterType::Lanczos3,
                )
            })
            .collect()
    } else {
        debug!("All images already at the same width {}", narrowest_width);
        images
    };

    let total_height: u32 = images.iter().map(|img| img.height()).sum();
    debug!(
        "Total stitched image size: {}x{}",
        desired_width, total_height
    );

    // Create and stitch
    let mut stitched_image = DynamicImage::new_rgba8(desired_width, total_height).to_rgba8();
    let mut current_y = 0;

    for (idx, img) in images.iter().enumerate() {
        debug!("Copying image {} at y={}", idx, current_y);
        stitched_image
            .copy_from(&img.to_rgba8(), 0, current_y)
            .map_err(|e| format!("Failed to copy image {}: {}", idx, e))?;
        current_y += img.height();
    }

    // Encode to PNG (fast compression, no extra filtering)
    debug!("Encoding stitched image to PNG (fast mode)");
    let mut png_data = Vec::new();
    let (w, h) = stitched_image.dimensions();
    {
        let encoder = PngEncoder::new_with_quality(
            Cursor::new(&mut png_data),
            CompressionType::Fast,
            PngFilterType::NoFilter,
        );
        encoder
            .write_image(stitched_image.as_raw(), w, h, ExtendedColorType::Rgba8)
            .map_err(|e| format!("Failed to encode image to PNG: {}", e))?;
    }

    // Save the same PNG bytes to disk (avoid double encoding)
    debug!("Saving stitched image to disk");
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let output_dir = FilePath::new("./stitched_images");

    fs::create_dir_all(output_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    let file_path = output_dir.join(format!("stitched_{}.png", timestamp));
    fs::write(&file_path, &png_data).map_err(|e| format!("Failed to save image to file: {}", e))?;

    info!("Image saved to {:?}", file_path);

    Ok(file_path.to_string_lossy().to_string())
}

fn process_stitch_blocking_bytes(image_data: Vec<String>) -> Result<Vec<u8>, String> {
    debug!(
        "Starting image stitching process with {} images",
        image_data.len()
    );

    let images = decode_images(image_data)?;

    if images.is_empty() {
        return Err("No images provided".to_string());
    }

    // Find narrowest width
    let narrowest_width = images
        .iter()
        .map(|img| img.width())
        .min()
        .ok_or("Failed to determine narrowest width")?;

    debug!("Narrowest width: {}", narrowest_width);

    let desired_width = min(narrowest_width, MAX_WIDTH);
    let needs_scaling = images.iter().any(|img| img.width() != desired_width);

    // Scale all images in parallel across available cores
    let images: Vec<_> = if needs_scaling {
        images
            .into_par_iter()
            .enumerate()
            .map(|(idx, img)| {
                if img.width() == desired_width {
                    debug!("Image {} already at desired width {}", idx, desired_width);
                    return img;
                }

                let scale_factor = desired_width as f32 / img.width() as f32;
                let new_height = (img.height() as f32 * scale_factor) as u32;
                debug!("Scaling image {} to {}x{}", idx, desired_width, new_height);
                img.resize_exact(
                    desired_width,
                    new_height,
                    image::imageops::FilterType::Lanczos3,
                )
            })
            .collect()
    } else {
        debug!("All images already at the same width {}", narrowest_width);
        images
    };

    let total_height: u32 = images.iter().map(|img| img.height()).sum();
    debug!(
        "Total stitched image size: {}x{}",
        desired_width, total_height
    );

    // Create and stitch
    let mut stitched_image = DynamicImage::new_rgba8(desired_width, total_height).to_rgba8();
    let mut current_y = 0;

    for (idx, img) in images.iter().enumerate() {
        debug!("Copying image {} at y={}", idx, current_y);
        stitched_image
            .copy_from(&img.to_rgba8(), 0, current_y)
            .map_err(|e| format!("Failed to copy image {}: {}", idx, e))?;
        current_y += img.height();
    }

    // Encode to PNG (fast compression, no extra filtering)
    debug!("Encoding stitched image to PNG (fast mode)");
    let mut png_data = Vec::new();
    let (w, h) = stitched_image.dimensions();
    {
        let encoder = PngEncoder::new_with_quality(
            Cursor::new(&mut png_data),
            CompressionType::Fast,
            PngFilterType::NoFilter,
        );
        encoder
            .write_image(stitched_image.as_raw(), w, h, ExtendedColorType::Rgba8)
            .map_err(|e| format!("Failed to encode image to PNG: {}", e))?;
    }

    Ok(png_data)
}

fn decode_images(image_data: Vec<String>) -> Result<Vec<DynamicImage>, String> {
    let images = image_data
        .into_par_iter()
        .enumerate()
        .map(|(idx, img_data)| {
            debug!("Decoding image {}", idx);

            // Remove whitespace from base64 string
            let cleaned_data: String = img_data.chars().filter(|c| !c.is_whitespace()).collect();

            let decoded = BASE64_STANDARD
                .decode(&cleaned_data)
                .map_err(|e| format!("Failed to decode base64 for image {}: {}", idx, e))?;

            let img = image::load_from_memory(&decoded)
                .map_err(|e| format!("Failed to load image {} from memory: {}", idx, e))?;

            let img = apply_exif_orientation(&decoded, img);

            debug!(
                "Image {} loaded successfully: {}x{}",
                idx,
                img.width(),
                img.height()
            );
            Ok(img)
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(images)
}

fn apply_exif_orientation(bytes: &[u8], img: DynamicImage) -> DynamicImage {
    let mut cursor = Cursor::new(bytes);

    if let Ok(exif) = exif::Reader::new().read_from_container(&mut cursor) {
        if let Some(field) = exif.get_field(Tag::Orientation, In::PRIMARY) {
            if let Some(orientation) = field.value.get_uint(0) {
                return match orientation {
                    1 => img,
                    3 => img.rotate180(),
                    6 => img.rotate90(),
                    8 => img.rotate270(),
                    _ => img,
                };
            }
        }
    }

    img
}
