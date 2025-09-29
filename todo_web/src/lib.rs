use axum::Router;

use crate::domain::appstate::AppState;

pub mod api;
pub mod db;
pub mod domain;

pub async fn build_app(state: AppState, assets_location: &String) -> Router {
    api::routes::create_router(state, assets_location)
}
