use worker::*;

#[event(fetch)]
async fn fetch(_req: HttpRequest, _env: Env, _ctx: Context) -> Result<HttpResponse> {
    let response = Response::ok("hello world")?;
    Ok(response.try_into()?)
}
