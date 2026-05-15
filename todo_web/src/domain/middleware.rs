use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use tower_sessions::Session;
use tracing::info;
use uuid::Uuid;

use crate::domain::{appstate::AppState, models::HttpError};

const USER_KEY: &str = "user";

pub async fn require_auth(
    mut session: Session,
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, HttpError> {
    // Check session
    let user_id = session.get::<Uuid>(USER_KEY).await.ok().flatten();

    if user_id.is_some() {
        return Ok(next.run(req).await);
    }

    // Check Cloudflare header
    if let Some(cf_user) = req.headers().get("cf-access-authenticated-user-email") {
        let email = cf_user.to_str().unwrap_or_default();
        // Look up or create user in DB
        let result = state
            .user_service
            .sign_up_or_sign_in(&mut session, &email.to_string())
            .await;

        // Proceed with request as authenticated user
        if let Ok(_) = result {
            Ok(next.run(req).await)
        } else {
            Err(HttpError::UnauthorizedRedirect)
        }
    } else {
        // No session, no Cloudflare header: redirect to login
        Err(HttpError::UnauthorizedRedirect)
    }
}

/// Logs all headers on incoming requests.
pub async fn log_headers(req: Request<Body>, next: Next) -> Response {
    info!("{} {}", req.method(), req.uri());

    next.run(req).await
}

pub async fn log_cookie_outbound(req: Request<Body>, next: Next) -> Response {
    let response = next.run(req).await;
    // Log response cookie header
    info!(
        "Response cookie header: {:?}",
        response.headers().get("Set-Cookie")
    );
    response
}
