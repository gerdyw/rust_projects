use axum::Router;

use crate::appstate::AppState;

pub mod appstate;
pub mod components;
pub mod db;
pub mod handlers;
pub mod models;
pub mod repository;
pub mod routes;
pub mod settings;

pub async fn build_app(state: AppState, assets_location: &String) -> Router {
    routes::create_router(state, assets_location)
}
