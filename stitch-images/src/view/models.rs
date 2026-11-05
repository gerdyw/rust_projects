use rocket::{
    Request, Responder,
    http::{Header, Status},
    response::{Redirect, Response, content::RawHtml},
};

#[derive(Responder)]
pub enum JobResult {
    #[response(status = 404)]
    NotFound(RawHtml<String>),

    #[response(status = 500)]
    InternalError(RawHtml<String>),

    #[response(status = 200)]
    Loading(RawHtml<String>),

    Ready(Redirect),
}

#[derive(Responder)]
pub enum JobPollResult {
    #[response(status = 404)]
    NotFound(()),

    #[response(status = 500)]
    InternalError(()),

    #[response(status = 204)]
    NoContent(()),

    HxRedirect(HxRedirectResponse),
}

pub struct HxRedirectResponse(pub String);

impl<'r> rocket::response::Responder<'r, 'static> for HxRedirectResponse {
    fn respond_to(self, _req: &'r Request<'_>) -> rocket::response::Result<'static> {
        Response::build()
            .status(Status::Ok)
            .header(Header::new("HX-Redirect", self.0))
            .ok()
    }
}
