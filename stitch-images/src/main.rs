use rocket::{get, launch, routes};

use stitch_images::{api::handlers, db::{init, repo::JobRepo}};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let pool = init().await.expect("Error initialising DB");
    let repo = JobRepo::new(pool);
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", handlers::routes())
        .manage(repo)
}