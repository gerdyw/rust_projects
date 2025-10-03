use maud::{Markup, Render, html};

use crate::domain::components::Page;

pub struct SignupForm(pub Option<String>);

impl Render for SignupForm {
    fn render(&self) -> Markup {
        html! {
            form #signup-form method="post" action="/users/signup" {
                @if let Some(err) = &self.0 {
                    div.error-message { (err) }
                }

                div.form-group {
                    label for="email" { "Email" }
                    input type="email" name="email" id="email" required {}
                }

                button.button type="submit" { "Sign Up" }
            }
        }
    }
}

pub struct SigninForm(pub Option<String>);
impl Render for SigninForm {
    fn render(&self) -> Markup {
        html! {
            form #signin-form method="post" action="/users/signin" {
                @if let Some(err) = &self.0 {
                    div.error-message { (err) }
                }

                div.form-group {
                    label for="email" { "Email" }
                    input type="email" name="email" id="email" required {}
                }

                button.button type="submit" { "Sign In" }
            }
        }
    }
}

pub enum UserForm {
    Signup(SignupForm),
    Signin(SigninForm),
}

pub struct UserPage(UserForm);
impl UserPage {
    pub fn new(form: UserForm) -> Self {
        Self(form)
    }
}

impl UserPage {
    pub fn sign_in(error: Option<String>) -> Self {
        Self::new(UserForm::Signin(SigninForm(error)))
    }

    pub fn sign_up(error: Option<String>) -> Self {
        Self::new(UserForm::Signup(SignupForm(error)))
    }
}

impl Render for UserPage {
    fn render(&self) -> Markup {
        let markup = html! {
            #app {
                h1 { "Welcome to Todo Web" }

                // Tab Navigation with HTMX
                div.tab-nav {
                    button.tab-link
                        hx-get="/users/signup"
                        hx-target="#form-container"
                        class=(match self.0 { UserForm::Signup(_) => "active", _ => "" })
                        { "Sign Up" }
                    button.tab-link
                        hx-get="/users/signin"
                        hx-target="#form-container"
                        class=(match self.0 { UserForm::Signin(_) => "active", _ => "" })
                        { "Sign In" }
                }

                // Form Container for HTMX targeting
                #form-container {
                    @match &self.0 {
                        UserForm::Signup(form) => (form),
                        UserForm::Signin(form) => (form),
                    }
                }
            }
        };

        Page("Login", markup).render()
    }
}
