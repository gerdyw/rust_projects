use crate::data::image_processing::{ImageProcessingRepository, ImageProcessingService};
use sqlx::PgPool;

/// Application state shared across all request handlers
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub image_processing_service: ImageProcessingService,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        let repository = ImageProcessingRepository::new(db.clone());
        let image_processing_service = ImageProcessingService::new(repository);

        Self {
            db,
            image_processing_service,
        }
    }
}
