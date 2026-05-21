use rocket::{Route, State, get, response::Redirect, routes};

use crate::{api::models::JobId, db::{models::ProcessingJob, repo::JobRepo}, view::{models::JobResult, page::{ErrorPage, LoadingPage}}};

#[get("/<job_id>")]
pub async fn get_job_page(repo: &State<JobRepo>, job_id: JobId) -> JobResult {
    let JobId(job_id) = job_id;

    let job = match repo.get_job(job_id).await {
        Ok(job) => job,
        Err(sqlx::Error::RowNotFound) => {
            return JobResult::NotFound(ErrorPage("Job not found".to_string()).render())
        }
        Err(_) => {
            return JobResult::InternalError(
                ErrorPage("An unexpected error has occurred".to_string()).render()
            )
        }
    };

    match job {
        ProcessingJob::Completed { id } => {
            JobResult::Ready(Redirect::to(format!("/images/{id}.jpeg")))
        }
        _ => JobResult::Loading(LoadingPage(job_id).render()),
    }
}

pub fn view_handlers() -> Vec<Route> {
    routes![get_job_page]
}