pub enum ProcessingError {
    DbError(sqlx::Error),
    ImageError(image::ImageError),
    CalculationError(String)
}

impl ProcessingError {
    pub fn message(&self) -> String {
        match self {
            ProcessingError::DbError(_) => "database error".to_string(),
            ProcessingError::ImageError(image_error) => image_error.to_string(),
            ProcessingError::CalculationError(err) => err.to_owned(),
        }
    }
}