use axum::{
    Router,
    http::{HeaderValue, header},
    middleware::from_fn,
    routing::get,
};

use tower::ServiceBuilder;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};
use tower_sessions::SessionManagerLayer;
use tower_sessions_redis_store::{RedisStore, fred::prelude::Pool};

use crate::{
    api::{
        handlers::{get_error, get_index},
        todo, user,
    },
    domain::{appstate::AppState, middleware::log_headers},
};

pub fn create_router(
    state: AppState,
    session_layer: SessionManagerLayer<RedisStore<Pool>>,
    assets_location: &String,
) -> Router {
    let static_router = get_servedir(assets_location);
    let todo_router = todo::routes::create_router(state.clone());
    let user_router = user::routes::create_router(state.clone());

    Router::new()
        .route("/", get(get_index))
        .route("/error", get(get_error))
        .nest("/todos", todo_router)
        .nest("/users", user_router)
        .nest_service("/assets", static_router)
        .layer(session_layer.clone())
        .layer(from_fn(log_headers))
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
