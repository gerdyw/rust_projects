use maud::{Markup, html};
use uuid::Uuid;

pub struct Page(Uuid);

impl Page {
    pub fn draw() -> Markup {
        html! {
            (maud::DOCTYPE)
            html lang="en" {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover";
                    meta name="format-detection" content="telephone=no";
                    meta name="theme-color" content="#000000";
                    meta name="color-scheme" content="dark";
                    title { "...Image stitching" }
                }
                body {
                    div class="loader" { }
                }
            }
        }
    }
}