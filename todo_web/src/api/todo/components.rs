use crate::domain::components::{HtmlComponent, IntoHtmlComponent};
use crate::{api::todo::models::Todo, domain::components::Page};
use maud::{Markup, Render, html};
use uuid::Uuid;

pub struct MarkDoneButton(pub Uuid, pub bool);

impl Render for MarkDoneButton {
    fn render(&self) -> Markup {
        let checked = "' ✅ '";
        let unchecked = "' ⬜ '";
        html! {
            @if self.1 {
                a .button hx-put=(format!("./todos/{}/undone", self.0)) hx-swap="outerHTML" hx-target="#todo-list"
                    hx-on:mouseenter={"this.textContent=" (unchecked)}
                    hx-on:mouseleave={"this.textContent=" (checked)}
                    { " ✅ " }
            } @else {
                a .button hx-put=(format!("./todos/{}/done", self.0)) hx-swap="outerHTML" hx-target="#todo-list"
                  hx-on:mouseenter={"this.textContent=" (checked)}
                  hx-on:mouseleave={"this.textContent=" (unchecked)}
                  { " ⬜ " }
            }
        }
    }
}

struct DeleteButton(pub Uuid);

impl Render for DeleteButton {
    fn render(&self) -> Markup {
        html! {
            a .button hx-delete=(format!("./todos/{}", self.0)) hx-swap="outerHTML" hx-target="#todo-list" { " ❌ " }
        }
    }
}

impl Render for Todo {
    fn render(&self) -> Markup {
        html! {
          li {
            (MarkDoneButton(self.meta.id, self.done))
              @if self.done {
                s { (self.title) }
              } @else {
                (self.title)
              }
              (DeleteButton(self.meta.id))
          }
        }
    }
}

pub struct TodoList(pub Vec<Todo>);

impl From<Vec<Todo>> for TodoList {
    fn from(todos: Vec<Todo>) -> Self {
        TodoList(todos)
    }
}

impl Render for TodoList {
    fn render(&self) -> Markup {
        html! {
            ul #todo-list {
                @for todo in &self.0 {
                    (todo)
                }
            }
        }
    }
}

pub struct CreateTodoForm {}

impl Render for CreateTodoForm {
    fn render(&self) -> Markup {
        html! {
            form hx-post="./todos" hx-target="#todo-list" hx-swap="outerHTML" hx-on::after-request="if(event.detail.successful) this.reset()"{
                input type="text" name="title" {}
                input type="submit" value="Create" {}
            }
        }
    }
}

pub struct TodoPage {
    pub todos: HtmlComponent<TodoList>,
    pub email: String,
    pub error: Option<String>,
}

impl TodoPage {
    pub fn new(todos: Vec<Todo>, email: String) -> Self {
        Self {
            todos: TodoList::from(todos).into_html_component(),
            email,
            error: None,
        }
    }
}

impl Render for TodoPage {
    fn render(&self) -> Markup {
        let markup = html! {
            div.page-container {
                header.app-header {
                    h1 { "Todo App" }
                    form.signout-form method="post" action="/users/signout" {
                        button.button type="submit" { "Sign Out" }
                    }
                }

                #app {
                        (self.todos)
                        (CreateTodoForm { })
                        @if let Some(err) = &self.error {
                            p .error { (err) }
                        }
                    }
                }
        };

        Page("Todos", markup).render()
    }
}
