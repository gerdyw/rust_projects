use maud::html;
use rocket::response::content::RawHtml;
use uuid::Uuid;

pub struct LoadingPage(pub Uuid);

impl LoadingPage {
    pub fn render(&self) -> RawHtml<String> {
        let page = html! {
            (maud::DOCTYPE)
            html lang="en" {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover";
                    meta name="format-detection" content="telephone=no";
                    meta name="theme-color" content="#000000";
                    meta name="color-scheme" content="dark";
                    link rel="stylesheet" href="/assets/styles.css";
                    link rel="stylesheet" href="/assets/loader.css";
                    script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.10/dist/htmx.min.js"
                        integrity="sha384-H5SrcfygHmAuTDZphMHqBJLc3FhssKjG7w/CeCpFReSfwBWDTKpkzPP8c+cLsK+V"
                        crossorigin="anonymous"
                    { }
                    title { "...Image stitching" }
                }
                body {
                    div
                        class="loader-shell"
                        hx-get={ (format!("/jobs/poll/{}", self.0)) }
                        hx-trigger="load, every 1s"
                        hx-swap="none"
                    {
                        div class="loader" { }
                        p class="loader-text" {
                            "loading"
                            span class="loading-dots" aria-hidden="true" {}
                        }
                    }
                }
            }
        };
        RawHtml(page.into_string())
    }
}

pub struct ErrorPage(pub String);

impl ErrorPage {
    pub fn render(&self) -> RawHtml<String> {
        let page = html! {
            (maud::DOCTYPE)
            html lang="en" {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover";
                    meta name="format-detection" content="telephone=no";
                    meta name="theme-color" content="#000000";
                    meta name="color-scheme" content="dark";
                    link rel="stylesheet" href="/assets/styles.css";
                    link rel="stylesheet" href="/assets/loader.css";
                    title { "An error occured" }
                }
                body {
                    div class="error-shell" {
                        p class="error" { (self.0) }
                    }
                }
            }
        };

        RawHtml(page.into_string())
    }
}
