use std::cmp::min;
use std::time::Instant;

use crate::{
    db::repo::JobRepo, persistence::ImageManager, processing::models::ProcessingError,
    telemetry::TelemetryMetrics,
};
use image::{DynamicImage, GenericImage};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use tracing::{error, info, instrument};
use uuid::Uuid;

const MAX_WIDTH: u32 = 2000;

#[instrument(skip(job_repo, image_manager, telemetry), fields(job_id = %job_id, image_count))]
pub async fn process_stitch(
    job_repo: JobRepo,
    image_manager: ImageManager,
    telemetry: TelemetryMetrics,
    job_id: Uuid,
    image_count: u32,
) {
    let started_at = Instant::now();
    let result = image_manager
        .retrieve_temp_images_for_job(job_id, image_count)
        .await
        .map_err(|err| ProcessingError::ImageError(err))
        .map(|images| process_stitch_blocking(images))
        .flatten()
        .map(|image| {
            image_manager
                .save_stitched_image(job_id, image)
                .map_err(|err| ProcessingError::ImageError(err))
        })
        .flatten();

    match result {
        Ok(()) => {
            telemetry.record_processing_duration(started_at.elapsed(), "completed", image_count);
            info!("completed image stitching job");
            job_repo.set_completed(job_id).await.unwrap()
        }
        Err(err) => {
            telemetry.record_processing_duration(started_at.elapsed(), "failed", image_count);
            error!(error = %err.message(), "image stitching job failed");
            job_repo.set_failed(job_id, err.message()).await.unwrap()
        }
    };

    info!("finished processing job");

    image_manager
        .remove_job_images(job_id, image_count)
        .expect(&format!("Failed to remove images for job {}", job_id));

    info!("removed temporary images for completed job");
}

fn process_stitch_blocking(images: Vec<DynamicImage>) -> Result<DynamicImage, ProcessingError> {
    let narrowest_width =
        images
            .iter()
            .map(|img| img.width())
            .min()
            .ok_or(ProcessingError::CalculationError(
                "Failed to determine narrowest width".to_string(),
            ))?;

    let desired_width = min(narrowest_width, MAX_WIDTH);
    let needs_scaling = images.iter().any(|img| img.width() != desired_width);

    let images: Vec<_> = if needs_scaling {
        images
            .into_par_iter()
            .enumerate()
            .map(|(idx, img)| {
                if img.width() == desired_width {
                    info!(
                        image_index = idx,
                        desired_width, "image already at desired width"
                    );
                    return img;
                }

                let scale_factor = desired_width as f32 / img.width() as f32;
                let new_height = (img.height() as f32 * scale_factor) as u32;
                info!(
                    image_index = idx,
                    target_width = desired_width,
                    target_height = new_height,
                    "scaling image before stitch"
                );
                img.resize_exact(
                    desired_width,
                    new_height,
                    image::imageops::FilterType::Lanczos3,
                )
            })
            .collect()
    } else {
        info!(narrowest_width, "all images already have matching width");
        images
    };

    let total_height: u32 = images.iter().map(|img| img.height()).sum();
    info!(
        stitched_width = desired_width,
        stitched_height = total_height,
        "computed stitched image dimensions"
    );

    // Create and stitch
    let mut stitched_image = DynamicImage::new_rgba8(desired_width, total_height).to_rgba8();
    let mut current_y = 0;

    for (idx, img) in images.iter().enumerate() {
        info!(
            image_index = idx,
            y_offset = current_y,
            "copying image into stitched output"
        );
        stitched_image
            .copy_from(&img.to_rgba8(), 0, current_y)
            .map_err(|err| ProcessingError::ImageError(err))?;
        current_y += img.height();
    }

    let dynamic: DynamicImage = stitched_image.into();
    Ok(dynamic)
}
