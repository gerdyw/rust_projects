use axum::{
    Form,
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
};
use tower_sessions::Session;

use crate::{
    api::user::{
        components::{SigninForm, SignupForm, UserPage},
        models::{ErrorQuery, SignupUser},
    },
    domain::{appstate::AppState, components::IntoHtmlComponent, errors::HttpError},
};

pub async fn get_index() -> impl IntoResponse {
    UserPage::sign_up(None).into_html_component()
}

pub async fn get_sign_in(query: Query<ErrorQuery>) -> impl IntoResponse {
    SigninForm(query.error.clone()).into_html_component()
}

pub async fn post_sign_in(
    state: State<AppState>,
    mut session: Session,
    Form(payload): Form<SignupUser>,
) -> Result<impl IntoResponse, HttpError> {
    let result = state
        .user_service
        .sign_in(&mut session, &payload.email)
        .await?;

    match result {
        Some(_user) => Ok(Redirect::to("/").into_response()),
        None => Ok(UserPage::sign_in(Some("User not found".to_string()))
            .into_html_component()
            .into_response()),
    }
}

pub async fn get_sign_up(query: Query<ErrorQuery>) -> impl IntoResponse {
    let page = SignupForm(query.error.clone());
    page.into_html_component()
}

pub async fn post_sign_up(
    state: State<AppState>,
    mut session: Session,
    Form(payload): Form<SignupUser>,
) -> Result<impl IntoResponse, HttpError> {
    let _ = state
        .user_service
        .sign_up(&mut session, &payload.email)
        .await?;

    Ok(Redirect::to("/").into_response())
}

pub async fn post_sign_out(session: Session) -> Response {
    session.delete().await.unwrap_or_else(|e| {
        tracing::error!("Failed to destroy session: {}", e);
    });

    Redirect::to("/users").into_response()
}
