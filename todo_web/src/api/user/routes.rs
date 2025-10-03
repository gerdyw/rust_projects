use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    api::user::handlers::{
        get_index, get_sign_in, get_sign_up, post_sign_in, post_sign_out, post_sign_up,
    },
    domain::appstate::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_index))
        .route("/signin", get(get_sign_in).post(post_sign_in))
        .route("/signup", get(get_sign_up).post(post_sign_up))
        .route("/signout", post(post_sign_out))
        .with_state(state)
}
