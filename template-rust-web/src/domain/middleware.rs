use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

/// Middleware to verify API key from x-api-key header
pub async fn verify_api_key(request: Request, next: Next) -> Result<Response, StatusCode> {
    // Get the API key from request extensions (injected by the layer)
    let expected_key = request
        .extensions()
        .get::<String>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get the x-api-key header
    let api_key = request
        .headers()
        .get("x-api-key")
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify the key
    if api_key != expected_key {
        tracing::warn!("Invalid API key attempt");
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}
