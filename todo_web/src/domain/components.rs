use axum::response::IntoResponse;
use axum::response::{Html, Response};
use maud::{Markup, Render, html};

// -----------------
// Reusable wrapper
// -----------------
pub struct HtmlComponent<T: Render>(pub T);

impl<T: Render> IntoResponse for HtmlComponent<T> {
    fn into_response(self) -> Response {
        Html(self.0.render().into_string()).into_response()
    }
}

impl<T: Render> Render for HtmlComponent<T> {
    fn render(&self) -> Markup {
        self.0.render()
    }
}

// Extension trait for ergonomics
pub trait IntoHtmlComponent: Render + Sized {
    fn into_html_component(self) -> HtmlComponent<Self> {
        HtmlComponent(self)
    }
}

impl<T: Render> IntoHtmlComponent for T {}

pub struct ErrorComponent {
    pub message: String,
}

impl Render for ErrorComponent {
    fn render(&self) -> Markup {
        html! {
            div class="notification is-danger" { (self.message) }
        }
    }
}

pub struct Page(pub &'static str, pub Markup);

impl Render for Page {
    fn render(&self) -> Markup {
        html! {
            html {
                head {
                    meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1" {}
                    title { (self.0) }
                    script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.7/dist/htmx.min.js" {}
                    link rel="stylesheet" href="./assets/pico.min.css" {}
                    link rel="stylesheet" href="./assets/styles.css" {}
                }
                body {
                    #app {
                        (self.1)
                    }
                }
            }
        }
    }
}
