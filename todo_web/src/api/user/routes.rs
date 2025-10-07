use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};

use crate::{
    api::user::handlers::{
        get_index, get_sign_in, get_sign_up, post_sign_in, post_sign_out, post_sign_up,
    },
    domain::{appstate::AppState, middleware::log_cookie_outbound},
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_index))
        .route("/signin", get(get_sign_in).post(post_sign_in))
        .route("/signup", get(get_sign_up).post(post_sign_up))
        .route("/signout", post(post_sign_out))
        .layer(from_fn(log_cookie_outbound))
        .with_state(state)
}
