use rocket::{
    fairing::AdHoc,
    fs::{FileServer, NamedFile},
    get,
    http::Status,
    launch, routes,
};
use uuid::Uuid;

use stitch_images::{
    api::handlers::api_routes,
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

#[get("/<file_name>")]
#[instrument(skip(image_manager), fields(otel.kind = "server", file_name = %file_name))]
async fn get_completed_image(
    image_manager: &rocket::State<ImageManager>,
    file_name: String,
) -> Result<NamedFile, Status> {
    let Some(job_id) = file_name.strip_suffix(".jpeg") else {
        return Err(Status::NotFound);
    };
    let job_id = Uuid::parse_str(job_id).map_err(|_| Status::BadRequest)?;
    let path = image_manager.stitched_image_path(job_id);
    info!(path = %path.display(), "serving completed stitched image");
    NamedFile::open(path).await.map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            Status::NotFound
        } else {
            error!(error = %err, "failed to open completed stitched image");
            Status::InternalServerError
        }
    })
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
