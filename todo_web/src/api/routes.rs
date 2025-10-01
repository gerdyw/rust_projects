use axum::{
    Router,
    http::{HeaderValue, header},
    routing::get,
};

use tower::ServiceBuilder;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};

use crate::{
    api::{handlers::get_index, todo, user},
    domain::appstate::AppState,
};

pub fn create_router(state: AppState, assets_location: &String) -> Router {
    let static_router = get_servedir(assets_location);
    let todo_router = todo::routes::create_router(state.clone());
    let user_router = user::routes::create_router(state.clone());
    Router::new()
        .route("/", get(get_index))
        .with_state(state.clone())
        .nest("/todos", todo_router)
        .nest("/users", user_router)
        .nest_service("/assets", static_router)
}

fn get_servedir(assets_location: &String) -> ServeDir {
    let service_builder = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("max-age=60"),
        ))
        .service(ServeDir::new(assets_location));
    service_builder.into_inner()
}
