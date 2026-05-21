use std::{fs, io::{self, Error}, path::PathBuf};

use image::{DynamicImage, ImageError, ImageReader};
use rocket::{fs::TempFile, futures::future::try_join_all};
use uuid::Uuid;

use crate::config::Config;

#[derive(Clone)]
pub struct ImageManager(Config);

impl ImageManager {
    pub fn new(config: Config) -> ImageManager {
        Self(config)
    }

    pub async fn save_temp_image(
        &self,
        job_id: Uuid,
        temp_image: &mut TempFile<'_>,
        idx: u32,
    ) -> Result<(), Error> {
        let path = self.temp_image_path(job_id, idx);
        temp_image.persist_to(path).await
    }

    pub async fn retrieve_temp_images_for_job(
        &self,
        job_id: Uuid,
        submitted_count: u32
    ) -> Result<Vec<DynamicImage>, ImageError> {
        let image_futures = (0..submitted_count)
            .into_iter()
            .map(|idx| self.retrieve_temp_image(job_id, idx));

        try_join_all(image_futures).await
    }

    pub fn save_stitched_image(&self, job_id: Uuid, image: DynamicImage) -> Result<(), ImageError> {
        let path = self.stitched_image_path(job_id);
        image.save(path)
    }

    pub fn remove_job_images(&self, job_id: Uuid, image_count: u32) -> io::Result<()> {
        self.temp_image_paths(job_id, image_count).iter().map(|path| fs::remove_file(path)).collect()
    }

    async fn retrieve_temp_image(&self, job_id: Uuid, idx: u32) -> Result<DynamicImage, ImageError> {
        let path = self.temp_image_path(job_id, idx);
        ImageReader::open(path)?.with_guessed_format()?.decode()
    }

    fn temp_image_path(&self, job_id: Uuid, idx: u32) -> PathBuf {
        let filename = format!("{}-{}", job_id, idx);
        PathBuf::from(&self.0.temp_images_dir).join(filename)
    }

    fn stitched_image_path(&self, job_id: Uuid) -> PathBuf {
        let filename = format!("{job_id}.jpeg");
        PathBuf::from(&self.0.dest_images_dir).join(filename)
    }

    fn temp_image_paths(&self, job_id: Uuid, image_count: u32) -> Vec<PathBuf> {
        (0..image_count).into_iter().map(|idx| self.temp_image_path(job_id, idx)).collect()
    }
}
