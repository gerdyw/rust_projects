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
pub fn create_router(state: AppState, api_key: Option<String>) -> Router {
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
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024)); // 100MB limit

    let app = add_auth_layer(api_key, app);

    app.layer(TraceLayer::new_for_http()).with_state(state)
}

fn add_auth_layer(api_key: Option<String>, app: Router<AppState>) -> Router<AppState> {
    // Apply API key middleware only if enabled
    let app = if let Some(api_key) = api_key {
        tracing::info!("API key authentication enabled");
        app.layer(middleware::from_fn(
            move |req: Request<Body>, next: Next| {
                let api_key = api_key.clone();
                async move {
                    let header_key = req.headers().get("x-api-key").and_then(|v| v.to_str().ok());

                    match header_key {
                        Some(key) if key == api_key => Ok(next.run(req).await),
                        Some(_) => {
                            tracing::warn!("Invalid API key attempt");
                            Err(StatusCode::UNAUTHORIZED)
                        }
                        None => Err(StatusCode::UNAUTHORIZED),
                    }
                }
            },
        ))
    } else {
        tracing::warn!("API key authentication is DISABLED");
        app
    };
    app
}
