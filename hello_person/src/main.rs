use axum::{Router, extract::Path, response::Html, routing::get};
use maud::html;
use tokio::net::TcpListener;

async fn greet(Path(name): Path<String>) -> Html<String> {
    let markup = html! {
        h1 { "Hello, " (name) "!" }
    };

    Html(markup.into())
}

#[tokio::main]
async fn main() {
    let app: Router<()> = Router::new().route("/hello/{name}", get(greet));
    println!("http://localhost:3000");
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
