use rocket::{Route, State, get, response::Redirect, routes};

use crate::{
    api::models::JobId,
    db::{models::ProcessingJob, repo::JobRepo},
    view::{
        models::{HxRedirectResponse, JobPollResult, JobResult},
        page::{ErrorPage, LoadingPage},
    },
};

#[get("/<job_id>")]
pub async fn get_job_page(repo: &State<JobRepo>, job_id: JobId) -> JobResult {
    let JobId(job_id) = job_id;

    let job = match repo.get_job(job_id).await {
        Ok(job) => job,
        Err(sqlx::Error::RowNotFound) => {
            return JobResult::NotFound(ErrorPage("Job not found".to_string()).render());
        }
        Err(err) => {
            match err.as_database_error() {
                Some(error) => eprintln!("{}", error.message()),
                None => eprintln!("unknown error"),
            };
            return JobResult::InternalError(
                ErrorPage("An unexpected error has occurred".to_string()).render(),
            );
        }
    };

    match job {
        ProcessingJob::Completed { id } => {
            JobResult::Ready(Redirect::to(format!("/images/{id}.jpeg")))
        }
        _ => JobResult::Loading(LoadingPage(job_id).render()),
    }
}

#[get("/poll/<job_id>")]
pub async fn poll_job(repo: &State<JobRepo>, job_id: JobId) -> JobPollResult {
    let JobId(job_id) = job_id;

    let job = match repo.get_job(job_id).await {
        Ok(job) => job,
        Err(sqlx::Error::RowNotFound) => return JobPollResult::NotFound(()),
        Err(_) => return JobPollResult::InternalError(()),
    };

    match job {
        ProcessingJob::Completed { id } => {
            JobPollResult::HxRedirect(HxRedirectResponse(format!("/images/{id}.jpeg")))
        }
        _ => JobPollResult::NoContent(()),
    }
}

pub fn view_handlers() -> Vec<Route> {
    routes![get_job_page, poll_job]
}
