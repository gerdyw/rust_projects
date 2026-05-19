use rocket::{http::Status, request::FromParam};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateJob {
    pub image_count: u32,
}

#[derive(Serialize)]
pub struct CreateJobResponse {
    pub job_id: Uuid
}

pub struct JobId(pub Uuid);

impl<'a> FromParam<'a> for JobId {
    type Error = Status;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Uuid::parse_str(param)
            .map(JobId)
            .map_err(|_| Status::BadRequest)
    }
}