use rocket::{Responder, response::{Redirect, content::RawHtml}};


#[derive(Responder)]
pub enum JobResult {
    #[response(status = 404)]
    NotFound(RawHtml<String>),

    #[response(status = 500)]
    InternalError(RawHtml<String>),
    
    #[response(status = 200)]
    Loading(RawHtml<String>),

    Ready(Redirect)
}