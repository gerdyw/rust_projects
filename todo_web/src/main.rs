use axum::serve;
use std::net::SocketAddr;
use todo_web::{
    web::app_routes::create_router,
    db::init_db,
    domain::{appstate::AppState, session_store::create_store, settings::load_settings},
};
use tokio::net::TcpListener;
use tracing::debug;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // logging initialization
    debug!("Starting up...");
    let settings = load_settings();
    // init DB
    let pool = init_db(&settings.database).await;
    let session_layer = create_store(settings.cache).await;

    let app_state = AppState::init(&pool);

    // app state + routes
    let app = create_router(app_state, session_layer, &settings.assets_location);

    // serve
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.port));
    println!("Listening on http://{addr}");
    serve(TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
