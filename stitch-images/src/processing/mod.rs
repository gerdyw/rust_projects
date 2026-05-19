
use image::{DynamicImage, ImageError, ImageReader};

use crate::db::models::ProcessingJob;

pub async fn load_images(job: &ProcessingJob) -> Result<Vec<DynamicImage>, ImageError> {
    let images: Result<Vec<DynamicImage>, ImageError> = (0..job.submitted_count)
        .map(|idx| format!("./temp_images/{}-{}", job.id, idx))
        .map(|path| load_image(path))
        .collect();
    images
}

fn load_image(path: String) -> Result<DynamicImage, ImageError> {
    ImageReader::open(path)?.with_guessed_format()?.decode()
}