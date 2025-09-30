use maud::{Markup, Render, html};

use crate::domain::components::Page;

pub struct LoginForm {}

impl Render for LoginForm {
    fn render(&self) -> Markup {
        html! {}
    }
}

pub struct SignupForm {}

impl Render for SignupForm {
    fn render(&self) -> Markup {
        html! {
            form #signup-form hx-post="./user/signup" hx-swap="outerHTML" hx-target="#signup-form" {
                label for="email" { "Email" }
                input type="email" name="email" {}
                // label for="password" { "Password" }
                // input type="password" name="password" {}
            }
        }
    }
}

pub struct UserPage {
    logged_in: bool,
}

impl Render for UserPage {
    fn render(&self) -> Markup {
        Page(
            "User",
            html! {
                #user-app {
                    @if self.logged_in {
                        (LoginForm { })
                    } @else {
                        (SignupForm { })
                    }
                }
            },
        )
        .render()
    }
}
