use crate::data::example::ExampleService;
use sqlx::PgPool;

/// Application state shared across all request handlers
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub example_service: ExampleService,
}

impl AppState {
    pub fn new(db: PgPool, example_service: ExampleService) -> Self {
        Self {
            db,
            example_service,
        }
    }
}
