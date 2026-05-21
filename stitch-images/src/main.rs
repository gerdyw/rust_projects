use rocket::{get, launch, routes};

use stitch_images::{api::handlers, config::Config, db::{init, repo::JobRepo}, persistence::ImageManager};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let pool = init().await.expect("Error initialising DB");
    let repo = JobRepo::new(pool);
    let config = Config::from_env();
    let image_manager = ImageManager::new(config.clone());
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", handlers::routes())
        .manage(repo)
        .manage(image_manager)
}