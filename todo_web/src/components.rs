use crate::models::Todo;
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

// -----------------
// Components
// -----------------

pub struct MarkDoneButton(pub i64, pub bool);

impl Render for MarkDoneButton {
    fn render(&self) -> Markup {
        html! {
            @if self.1 {
                a .button hx-put=(format!("/todos/{}/undone", self.0)) hx-swap="outerHTML" hx-target="#todo-list"
                    hx-on:mouseenter="this.textContent = ' ⬜ '"
                    hx-on:mouseleave="this.textContent = ' ✅ '"
                    { " ✅ " }
            } @else {
                a .button hx-put=(format!("/todos/{}/done", self.0)) hx-swap="outerHTML" hx-target="#todo-list"
                  hx-on:mouseenter="this.textContent = ' ✅ '"
                  hx-on:mouseleave="this.textContent = ' ⬜ '"
                  { " ⬜ " }
            }
        }
    }
}

pub struct TodoList(pub Vec<Todo>);

impl Render for TodoList {
    fn render(&self) -> Markup {
        html! {
            ul #todo-list {
                @for todo in &self.0 {
                    li {
                        @if todo.done {
                            (MarkDoneButton(todo.id, todo.done).render())
                            s { (todo.title) }
                            a .button hx-delete=(format!("/todos/{}", todo.id)) hx-swap="outerHTML" hx-target="#todo-list" { " ❌ " }
                        } @else {
                            (MarkDoneButton(todo.id, todo.done).render())
                            (todo.title)
                        }
                    }
                }
            }
        }
    }
}

pub struct CreateTodoForm {}

impl Render for CreateTodoForm {
    fn render(&self) -> Markup {
        html! {
            form hx-post="/todos" hx-target="#todo-list" hx-swap="outerHTML" hx-on::after-request="if(event.detail.successful) this.reset()"{
                input type="text" name="title" {}
                input type="submit" value="Create" {}
            }
        }
    }
}

pub struct Index {
    pub todos: HtmlComponent<TodoList>,
}

impl Render for Index {
    fn render(&self) -> Markup {
        html! {
            html {
                head {
                    script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.7/dist/htmx.min.js" {}
                    link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css" {}
                    style { ".button { cursor:  pointer; text-decoration: none; }" }
                }
                body {
                    #todo-app style="max-width: 800px; margin: 0 auto" {
                        h1 { "Todo App" }
                        (self.todos)
                        (CreateTodoForm { })
                    }
                }
            }
        }
    }
}

// struct IncrementerButton {
//     href: String,
// }

// impl Render for IncrementerButton {
//     fn render(&self) -> Markup {
//         html! {
//             a hx-post=(self.href) hx-target="#count" hx-swap="innerHTML" {
//                 "Increment"
//             }
//         }
//     }
// }

// struct Counter {
//     count: usize,
// }

// impl Render for Counter {
//     fn render(&self) -> Markup {
//         html! {
//             #count { (self.count) }
//         }
//     }
// }

// // -----------------
// // Handlers
// // -----------------
// async fn index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
//     let count = state.count.lock().unwrap().clone();

//     html! {
//         html {
//             head {
//                 script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.7/dist/htmx.min.js" {}
//             }
//             body {
//                 (Counter { count })
//                 (IncrementerButton { href: "/increment".into() })
//             }
//         }
//     }
//     .into_html_component()
// }
