use rocket::{fs::FileServer, get, launch, routes};

use stitch_images::{api::handlers::api_routes, config::Config, db::{init, repo::JobRepo}, persistence::ImageManager, view::handlers::view_handlers};

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
        .mount("/api", api_routes())
        .mount("/jobs", view_handlers())
        .mount("/images", FileServer::from(config.dest_images_dir))
        .mount("/assets", FileServer::from("./assets"))
        .manage(repo)
        .manage(image_manager)
}