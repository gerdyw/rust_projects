use project_name::data::example::{ExampleRepository, ExampleService};
use project_name::db::{init_pool, run_migrations};
use project_name::domain::{AppState, Settings};
use project_name::web::create_router;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "project_name=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting project-name service");

    // Load settings from environment
    let settings = Settings::from_env()?;
    tracing::info!("Configuration loaded successfully");

    // Initialize database connection pool
    let database_url = settings.database_url();
    tracing::info!("Connecting to database at {}", settings.database.host);
    let pool = init_pool(&database_url).await?;
    tracing::info!("Database connection established");

    // Run database migrations
    tracing::info!("Running database migrations");
    run_migrations(&pool).await?;
    tracing::info!("Database migrations completed");

    // Create application state
    let app_state = AppState::new(pool);

    // Build router with middleware
    let app = create_router(app_state, settings.api_key.key);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.service_port));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
