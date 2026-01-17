use serde::Serialize;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct StitchRequest {
    pub images: Vec<String>,
}

#[derive(Serialize)]
pub struct JobSubmitResponse {
    pub job_id: Uuid,
    pub status: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct JobStatusResponse {
    pub job_id: Uuid,
    pub status: String,
    pub image_count: i32,
    pub result_path: Option<String>,
    pub error_message: Option<String>,
}
