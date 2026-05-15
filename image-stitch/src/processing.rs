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
use std::time::Instant;
use tokio::task;
use tracing::{debug, instrument};

const MAX_WIDTH: u32 = 2000;

/// Process images asynchronously and save to disk
#[instrument(skip(image_data), fields(image_count = image_data.len(), operation = "async_job"))]
pub async fn process_stitch(image_data: Vec<String>) -> Result<String, String> {
    debug!("Spawning blocking task for image stitching");
    let result = task::spawn_blocking(move || process_stitch_blocking(image_data))
        .await
        .map_err(|e| format!("Task join error: {}", e))??;

    Ok(result)
}

/// Process images asynchronously and return bytes
#[instrument(skip(image_data), fields(image_count = image_data.len(), operation = "sync_request"))]
pub async fn process_stitch_sync(image_data: Vec<String>) -> Result<Vec<u8>, String> {
    debug!("Spawning blocking task for image stitching");
    let result = task::spawn_blocking(move || process_stitch_blocking_bytes(image_data))
        .await
        .map_err(|e| format!("Task join error: {}", e))??;

    Ok(result)
}

#[instrument(skip(image_data), fields(image_count = image_data.len(), operation = "async_job"))]
fn process_stitch_blocking(image_data: Vec<String>) -> Result<String, String> {
    let started_at = Instant::now();
    let image_count = image_data.len();

    let result = (|| -> Result<(String, usize, u32, u32), String> {
        debug!(
            "Starting image stitching process with {} images",
            image_count
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
        fs::write(&file_path, &png_data)
            .map_err(|e| format!("Failed to save image to file: {}", e))?;

        tracing::info!("Image saved to {:?}", file_path);

        Ok((file_path.to_string_lossy().to_string(), png_data.len(), w, h))
    })();

    match &result {
        Ok((_, output_bytes, output_width, output_height)) => telemetry::record_image_stitch_success(
            "async_job",
            image_count,
            started_at.elapsed(),
            *output_bytes,
            *output_width,
            *output_height,
        ),
        Err(_) => telemetry::record_image_stitch_failure("async_job", image_count, started_at.elapsed()),
    }

    result.map(|(path, _, _, _)| path)
}

#[instrument(skip(image_data), fields(image_count = image_data.len(), operation = "sync_request"))]
fn process_stitch_blocking_bytes(image_data: Vec<String>) -> Result<Vec<u8>, String> {
    let started_at = Instant::now();
    let image_count = image_data.len();

    let result = (|| -> Result<(Vec<u8>, u32, u32), String> {
        debug!(
            "Starting image stitching process with {} images",
            image_count
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

        Ok((png_data, w, h))
    })();

    match &result {
        Ok((png_data, output_width, output_height)) => telemetry::record_image_stitch_success(
            "sync_request",
            image_count,
            started_at.elapsed(),
            png_data.len(),
            *output_width,
            *output_height,
        ),
        Err(_) => telemetry::record_image_stitch_failure("sync_request", image_count, started_at.elapsed()),
    }

    result.map(|(png_data, _, _)| png_data)
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
