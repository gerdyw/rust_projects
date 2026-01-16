#[derive(serde::Deserialize)]
pub struct StitchRequest {
    pub images: Vec<String>,
}
