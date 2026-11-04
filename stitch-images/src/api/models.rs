use rocket::{http::Status, request::FromParam};
use rocket::response::{Responder, status::Accepted};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::models::ProcessingJob;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateJob {
    pub image_count: u32,
}

#[derive(Serialize)]
pub struct CreateJobResponse {
    pub job_id: Uuid
}

#[derive(Responder)]
pub enum CreateJobResult {
    #[response(status = 201)]
    Created(Json<CreateJobResponse>),

    #[response(status = 500)]
    InternalError(String),
}

#[derive(Responder)]
pub enum GetJobResult {
    #[response(status = 200)]
    Found(Json<ProcessingJob>),

    #[response(status = 404)]
    NotFound(()),

    #[response(status = 500)]
    InternalError(String),
}

#[derive(Responder)]
pub enum SubmitImageResult {
    #[response(status = 202)]
    Submitted(Accepted<()>),

    #[response(status = 400)]
    BadRequest(()),

    #[response(status = 404)]
    NotFound(()),

    #[response(status = 500)]
    InternalError(String),
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