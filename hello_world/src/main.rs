use axum::{Router, response::Html, routing::get};
use maud::html;
use tokio::net::TcpListener;

async fn random_body() -> Html<String> {
    let body = html! {
        p { "Random number: " (rand::random::<u8>()) }
    };

    Html(body.into())
}

#[tokio::main]
async fn main() {
    let app: Router<()> = Router::new().route("/", get(random_body));
    println!("http://localhost:3000");
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
