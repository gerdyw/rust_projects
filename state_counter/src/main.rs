use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use maud::{Markup, Render, html};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    count: Arc<Mutex<usize>>,
}

// -----------------
// Reusable wrapper
// -----------------
pub struct HtmlComponent<T: Render>(pub T);

impl<T: Render> IntoResponse for HtmlComponent<T> {
    fn into_response(self) -> Response {
        Html(self.0.render().into_string()).into_response()
    }
}

// Extension trait for ergonomics
trait IntoHtmlComponent: Render + Sized {
    fn into_html_component(self) -> HtmlComponent<Self> {
        HtmlComponent(self)
    }
}

impl<T: Render> IntoHtmlComponent for T {}

// -----------------
// Components
// -----------------
struct IncrementerButton {
    href: String,
}

impl Render for IncrementerButton {
    fn render(&self) -> Markup {
        html! {
            a hx-post=(self.href) hx-target="#count" hx-swap="innerHTML" {
                "Increment"
            }
        }
    }
}

struct Counter {
    count: usize,
}

impl Render for Counter {
    fn render(&self) -> Markup {
        html! {
            #count { (self.count) }
        }
    }
}

// -----------------
// Handlers
// -----------------
async fn index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let count = state.count.lock().unwrap().clone();

    html! {
        html {
            head {
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.7/dist/htmx.min.js" {}
            }
            body {
                (Counter { count })
                (IncrementerButton { href: "/increment".into() })
            }
        }
    }
    .into_html_component()
}

async fn increment(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut count = state.count.lock().unwrap();
    *count += 1;
    Counter { count: *count }.into_html_component()
}

// -----------------
// Main
// -----------------
#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        count: Arc::new(Mutex::new(0)),
    });

    let app = Router::new()
        .route("/", get(index))
        .route("/increment", post(increment))
        .with_state(state);

    println!("http://localhost:3000");
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
