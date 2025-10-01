use maud::{Markup, Render, html};

use crate::{api::user::models::User, domain::components::Page};

// pub struct LoginForm {}

// impl Render for LoginForm {
//     fn render(&self) -> Markup {
//         html! {}
//     }
// }

pub struct SignupForm {}

impl Render for SignupForm {
    fn render(&self) -> Markup {
        html! {
            form #signup-form hx-post="./users/signup" hx-swap="outerHTML" {
                label for="email" { "Email" }
                input type="email" name="email" {}
                // label for="password" { "Password" }
                // input type="password" name="password" {}
                button type="submit" { "Sign Up" }
            }
        }
    }
}

#[derive(Debug)]
pub struct UserComponent(pub Result<User, String>);

impl Render for UserComponent {
    fn render(&self) -> Markup {
        html! {
            @match &self.0 {
                Ok(user) => div { "Welcome, " (user.email) "!" }
                Err(err) => div { "Error: " (err) }
            }
        }
    }
}

pub struct UserPage {}

impl Render for UserPage {
    fn render(&self) -> Markup {
        Page("Signup", SignupForm {}.render()).render()
    }
}
