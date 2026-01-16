use axum::{http::StatusCode, response::Response, Json};
use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::Local;
use image::{DynamicImage, GenericImage, ImageFormat};
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::{cmp::min, io::Cursor};
use tokio::task;
use tracing::{debug, error, info};

use crate::web::StitchRequest;

const MAX_WIDTH: u32 = 2000;

pub async fn stitch_images(Json(data): Json<StitchRequest>) -> Result<Response, StatusCode> {
    match process_stitch(data.images).await {
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

async fn process_stitch(image_data: Vec<String>) -> Result<Vec<u8>, String> {
    debug!("Spawing blocking task for image stitching");
    let result = task::spawn_blocking(move || process_stitch_blocking(image_data))
        .await
        .map_err(|e| format!("Task join error: {}", e))?;

    Ok(result.await?)
}

async fn process_stitch_blocking(image_data: Vec<String>) -> Result<Vec<u8>, String> {
    debug!(
        "Starting image stitching process with {} images",
        image_data.len()
    );

    // Decode all images
    let mut images = Vec::new();
    for (idx, img_data) in image_data.iter().enumerate() {
        debug!("Decoding image {}", idx);

        // Remove whitespace from base64 string
        let cleaned_data: String = img_data.chars().filter(|c| !c.is_whitespace()).collect();

        let decoded = BASE64_STANDARD
            .decode(&cleaned_data)
            .map_err(|e| format!("Failed to decode base64 for image {}: {}", idx, e))?;

        let img = image::load_from_memory(&decoded)
            .map_err(|e| format!("Failed to load image {} from memory: {}", idx, e))?;

        debug!(
            "Image {} loaded successfully: {}x{}",
            idx,
            img.width(),
            img.height()
        );
        images.push(img);
    }

    if images.is_empty() {
        return Err("No images provided".to_string());
    }

    // Find widest width
    let narrowest_width = images
        .iter()
        .map(|img| img.width())
        .min()
        .ok_or("Failed to determine widest width")?;

    let desired_width = min(narrowest_width, MAX_WIDTH);

    debug!("Narrowest width: {}", narrowest_width);

    // Scale all images in parallel across available cores
    let scaled_images: Vec<_> = images
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
        .collect();

    let total_height: u32 = scaled_images.iter().map(|img| img.height()).sum();
    debug!(
        "Total stitched image size: {}x{}",
        desired_width, total_height
    );

    // Create and stitch
    let mut stitched_image = DynamicImage::new_rgba8(desired_width, total_height).to_rgba8();
    let mut current_y = 0;

    for (idx, img) in scaled_images.iter().enumerate() {
        debug!("Copying image {} at y={}", idx, current_y);
        stitched_image
            .copy_from(&img.to_rgba8(), 0, current_y)
            .map_err(|e| format!("Failed to copy image {}: {}", idx, e))?;
        current_y += img.height();
    }

    // Encode to PNG
    debug!("Encoding stitched image to PNG");
    let mut png_data = Vec::new();
    DynamicImage::ImageRgba8(stitched_image.clone())
        .write_to(&mut Cursor::new(&mut png_data), ImageFormat::Png)
        .map_err(|e| format!("Failed to encode image to PNG: {}", e))?;

    // Save to file
    debug!("Saving stitched image to disk");
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let output_dir = Path::new("./stitched_images");

    fs::create_dir_all(output_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    let file_path = output_dir.join(format!("stitched_{}.png", timestamp));
    DynamicImage::ImageRgba8(stitched_image)
        .save(&file_path)
        .map_err(|e| format!("Failed to save image to file: {}", e))?;

    info!("Image saved to {:?}", file_path);

    Ok(png_data)
}
