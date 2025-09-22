use axum::serve;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use use_database::appstate::AppState;
use use_database::db::init_db;
use use_database::repository;
use use_database::routes::create_router;

#[tokio::main]
async fn main() {
    // init DB
    let pool = init_db().await;
    let repo = repository::TodoRepository::new(pool.clone());
    let app_state = AppState { repo };
    // app state + router
    let app = create_router(app_state);

    // serve
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    serve(TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
