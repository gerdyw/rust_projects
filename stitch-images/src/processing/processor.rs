use std::cmp::min;

use crate::{db::repo::JobRepo, persistence::ImageManager, processing::models::ProcessingError};
use image::{DynamicImage, GenericImage};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use uuid::Uuid;

const MAX_WIDTH: u32 = 2000;

pub async fn process_stitch(
    job_repo: JobRepo,
    image_manager: ImageManager,
    job_id: Uuid,
    image_count: u32,
) {
    let result = image_manager
        .retrieve_temp_images_for_job(job_id, image_count)
        .await
        .map_err(|err| ProcessingError::ImageError(err))
        .map(|images| process_stitch_blocking(images))
        .flatten()
        .map(|image| image_manager.save_stitched_image(job_id, image).map_err(|err| ProcessingError::ImageError(err)))
        .flatten();

    match result {
        Ok(()) => job_repo.set_completed(job_id).await.unwrap(),
        Err(err) => job_repo.set_failed(job_id, err.message()).await.unwrap()
    };

    println!("Finished processing {job_id}");

    image_manager.remove_job_images(job_id, image_count).expect(&format!("Failed to remove images for job {}", job_id));

    println!("Removed images for {job_id}");
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
                    println!("Image {} already at desired width {}", idx, desired_width);
                    return img;
                }

                let scale_factor = desired_width as f32 / img.width() as f32;
                let new_height = (img.height() as f32 * scale_factor) as u32;
                println!("Scaling image {} to {}x{}", idx, desired_width, new_height);
                img.resize_exact(
                    desired_width,
                    new_height,
                    image::imageops::FilterType::Lanczos3,
                )
            })
            .collect()
    } else {
        println!("All images already at the same width {}", narrowest_width);
        images
    };

    let total_height: u32 = images.iter().map(|img| img.height()).sum();
    println!(
        "Total stitched image size: {}x{}",
        desired_width, total_height
    );

    // Create and stitch
    let mut stitched_image = DynamicImage::new_rgba8(desired_width, total_height).to_rgba8();
    let mut current_y = 0;

    for (idx, img) in images.iter().enumerate() {
        println!("Copying image {} at y={}", idx, current_y);
        stitched_image
            .copy_from(&img.to_rgba8(), 0, current_y)
            .map_err(|err| ProcessingError::ImageError(err))?;
        current_y += img.height();
    }

    let dynamic: DynamicImage = stitched_image.into();
    Ok(dynamic)
}
