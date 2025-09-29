use axum::response::IntoResponse;
use axum::response::{Html, Response};
use maud::{Markup, Render};

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
