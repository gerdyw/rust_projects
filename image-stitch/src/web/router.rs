use crate::domain::settings::ApiKeyMode;
use crate::domain::AppState;
use axum::extract::DefaultBodyLimit;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

/// Create the application router with all routes and middleware
pub fn create_router(state: AppState, api_key_mode: ApiKeyMode) -> Router {
    let app = Router::new()
        .route("/health", get(super::health_check))
        // Legacy sync endpoint
        .route("/stitch", post(super::stitch_images))
        // New async endpoints
        .route("/jobs", post(super::submit_stitch_job))
        .route("/jobs/{job_id}", get(super::get_job_status))
        .route("/jobs/{job_id}/result", get(super::get_job_result))
        // HTMX frontend endpoints
        .route("/wait/{job_id}", get(super::serve_waiting_page))
        .route("/status/{job_id}", get(super::get_status_fragment))
        .route("/images/{job_id}", get(super::serve_image_page))
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024))
        .layer(middleware::from_fn(telemetry::http_telemetry_middleware)); // 100MB limit

    let app = add_auth_layer(api_key_mode, app);

    app.layer(TraceLayer::new_for_http()).with_state(state)
}

fn add_auth_layer(api_key_mode: ApiKeyMode, app: Router<AppState>) -> Router<AppState> {
    match api_key_mode {
        ApiKeyMode::None => {
            tracing::info!("API key authentication is DISABLED");
            app
        }
        ApiKeyMode::Full(api_key) => {
            tracing::info!("API key authentication enabled for all routes");
            app.layer(middleware::from_fn(
                move |req: Request<Body>, next: Next| {
                    let api_key = api_key.clone();
                    async move {
                        verify_api_key(&api_key, &req)?;
                        Ok::<_, StatusCode>(next.run(req).await)
                    }
                },
            ))
        }
        ApiKeyMode::SubmitOnly(api_key) => {
            tracing::info!("API key authentication enabled for POST endpoints only");
            app.layer(middleware::from_fn(
                move |req: Request<Body>, next: Next| {
                    let api_key = api_key.clone();
                    async move {
                        // Only require auth for POST requests to /stitch and /jobs
                        let path = req.uri().path();
                        let is_stitch_endpoint = path == "/stitch" || path.starts_with("/jobs");
                        let is_post = req.method() == axum::http::Method::POST;

                        if is_stitch_endpoint && is_post {
                            verify_api_key(&api_key, &req)?;
                        }
                        Ok::<_, StatusCode>(next.run(req).await)
                    }
                },
            ))
        }
    }
}

fn verify_api_key(api_key: &str, req: &Request<Body>) -> Result<(), StatusCode> {
    let header_key = req.headers().get("x-api-key").and_then(|v| v.to_str().ok());

    match header_key {
        Some(key) if key == api_key => Ok(()),
        Some(_) => {
            tracing::warn!("Invalid API key attempt");
            Err(StatusCode::UNAUTHORIZED)
        }
        None => {
            tracing::warn!("Missing API key");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
