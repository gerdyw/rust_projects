use rocket::{State, get, http::Status, response::content};
use uuid::Uuid;

use crate::{api::models::JobId, db::repo::JobRepo};

#[get("/<job_id>")]
pub async fn get_loading_page(repo: &State<JobRepo>, job_id: JobId) ->  {
    let JobId(job_id) = job_id;
    let job = repo.get_job(job_id).await;
}