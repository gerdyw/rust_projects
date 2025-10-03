use axum::serve;
use std::net::SocketAddr;
use todo_web::{
    api::routes::create_router,
    db::init_db,
    domain::{appstate::AppState, settings::load_settings},
};
use tokio::net::TcpListener;
use tower_sessions::{Expiry, MemoryStore, cookie::time::Duration};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // logging initialization
    let settings = load_settings();
    // init DB
    let pool = init_db(&settings.database).await;
    let session_store = MemoryStore::default();
    let session_service = tower_sessions::SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(30)));

    let app_state = AppState::init(&pool);

    // app state + routes
    let app = create_router(app_state, session_service, &settings.assets_location);

    // serve
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.port));
    println!("Listening on http://{addr}");
    serve(TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
