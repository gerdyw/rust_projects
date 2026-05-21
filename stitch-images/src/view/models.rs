use rocket::http::Status;

use crate::view::page::Page;

pub enum JobResult {
    JobLoadingPage(Page),
    Redirect(Status::Re)
}