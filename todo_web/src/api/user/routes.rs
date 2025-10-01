use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    api::user::handlers::{get_index, signup_user},
    domain::appstate::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_index))
        .route("/signup", post(signup_user))
        .with_state(state)
}
