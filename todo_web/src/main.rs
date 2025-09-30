use axum::serve;
use std::net::SocketAddr;
use todo_web::{
    api::{routes::create_router, todo::repository::TodoRepository},
    db::init_db,
    domain::{appstate::AppState, settings::load_settings},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // logging initialization
    let settings = load_settings();
    // init DB
    let pool = init_db(&settings.database).await;
    let todo_repo: TodoRepository = TodoRepository::new(pool.clone());
    let app_state = AppState::new(todo_repo);
    // app state + routes
    let app = create_router(app_state, &settings.assets_location);

    // serve
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.port));
    println!("Listening on http://{addr}");
    serve(TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
