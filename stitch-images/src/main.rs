use rocket::{
    fairing::AdHoc,
    fs::{FileServer, NamedFile},
    get, launch, routes,
};
use std::io;

use stitch_images::{
    api::{handlers::api_routes, models::JobId},
    config::Config,
    db::{init, repo::JobRepo},
    persistence::ImageManager,
    telemetry::{Telemetry, TelemetryMetrics},
    view::handlers::view_handlers,
};
use tracing::{error, info, instrument};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<job_id>.jpeg")]
#[instrument(skip(image_manager), fields(otel.kind = "server", job_id = %job_id.0))]
async fn get_completed_image(
    image_manager: &rocket::State<ImageManager>,
    job_id: JobId,
) -> Result<NamedFile, io::Error> {
    let JobId(job_id) = job_id;
    let path = image_manager.stitched_image_path(job_id);
    info!(path = %path.display(), "serving completed stitched image");
    NamedFile::open(path).await
}

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let telemetry = Telemetry::init("stitch_images").expect("Error initialising telemetry");
    let telemetry_metrics: TelemetryMetrics = telemetry.metrics();
    let pool = init().await.expect("Error initialising DB");
    let repo = JobRepo::new(pool);
    let config = Config::from_env();
    let image_manager = ImageManager::new(config.clone());
    info!("starting stitch-images service");
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", api_routes())
        .mount("/jobs", view_handlers())
        .mount("/images", routes![get_completed_image])
        .mount("/images", FileServer::from(config.dest_images_dir))
        .mount("/assets", FileServer::from("./assets"))
        .manage(repo)
        .manage(image_manager)
        .manage(telemetry_metrics)
        .manage(telemetry)
        .attach(AdHoc::on_shutdown("Shutdown telemetry", |rocket| {
            Box::pin(async move {
                if let Some(telemetry) = rocket.state::<Telemetry>() {
                    telemetry.shutdown();
                } else {
                    error!("telemetry state unavailable during shutdown");
                }
            })
        }))
}