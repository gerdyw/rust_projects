use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use tower_sessions::Session;
use tracing::info;
use uuid::Uuid;

const PUBLIC_PATHS: [&str; 2] = ["/users/signin", "/users/signup"];
const USER_KEY: &str = "user";

pub async fn require_auth(
    session: Session,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    let path = req.uri().path();

    // Allow public paths to bypass auth
    if PUBLIC_PATHS.iter().any(|p| path.starts_with(p)) {
        return Ok(next.run(req).await);
    }

    // Check session
    let user_id = session.get::<Uuid>(USER_KEY).await.map_err(|e| {
        tracing::error!("Session error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    match user_id {
        Some(_) => Ok(next.run(req).await),
        None => {
            tracing::debug!("No user session found, redirecting to signin");
            Err(Redirect::to("/users").into_response())
        }
    }
}

/// Logs all headers on incoming requests.
pub async fn log_headers(req: Request<Body>, next: Next) -> Response {
    info!("{} {}", req.method(), req.uri());

    // for (name, value) in req.headers() {
    //     info!("Header: {} = {:?}", name, value);
    // }

    info!("{:?}", req.headers());

    next.run(req).await
}
