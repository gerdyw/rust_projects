use axum::serve;
use std::net::SocketAddr;
use todo_web::appstate::AppState;
use todo_web::db::init_db;
use todo_web::repository;
use todo_web::routes::create_router;
use tokio::net::TcpListener;

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
