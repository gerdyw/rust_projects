use image_stitch::db::{init_pool, run_migrations};
use image_stitch::domain::{AppState, Settings};
use image_stitch::web::create_router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Load .env file if present (for local development)
    dotenvy::dotenv().ok();

    let telemetry = telemetry::init("image_stitch")?;

    tracing::info!("Starting image-stitch service");

    // Load settings from environment
    let settings = Settings::from_env()?;
    tracing::info!("Configuration loaded successfully");

    // Initialize database connection pool
    tracing::info!("Connecting to database at {}", settings.database.host);
    let pool = init_pool(&settings.database).await?;
    tracing::info!("Database connection established");

    // Run database migrations
    tracing::info!("Running database migrations");
    run_migrations(&pool).await?;
    tracing::info!("Database migrations completed");

    // Create application state
    let app_state = AppState::new(pool);

    // Build router with middleware
    let app = create_router(app_state, settings.api_key);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.service_port));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    telemetry.shutdown()?;

    Ok(())
}
