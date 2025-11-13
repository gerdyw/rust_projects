use axum::{
    Form,
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;
use tower_sessions::Session;
use tracing::error;

use crate::{
    data::user::models::{ErrorQuery, SignupUser},
    domain::{
        appstate::AppState,
        components::IntoHtmlComponent,
        models::{HttpError, HttpSuccess, Never},
    },
    web::user::components::{SigninForm, SignupForm, UserPage},
};

#[derive(Debug, Clone, Deserialize)]
pub struct LoginQuery {
    pub user_email: Option<String>,
}

pub async fn get_index(
    state: State<AppState>,
    mut session: Session,
    Query(query): Query<LoginQuery>,
) -> Result<HttpSuccess<UserPage>, HttpError> {
    let Some(email) = query.user_email else {
        // No email provided: show login form
        return Ok(HttpSuccess::Html(
            UserPage::sign_in(None).into_html_component(),
        ));
    };

    match state.user_service.sign_in(&mut session, &email).await {
        Ok(_) => Ok(HttpSuccess::Redirect("/todos".to_string())),
        Err(_) => Ok(HttpSuccess::Html(
            UserPage::sign_up(None).into_html_component(),
        )),
    }
}

pub async fn get_sign_in(query: Query<ErrorQuery>) -> impl IntoResponse {
    SigninForm(query.error.clone()).into_html_component()
}

pub async fn post_sign_in(
    state: State<AppState>,
    mut session: Session,
    Form(payload): Form<SignupUser>,
) -> Result<HttpSuccess<Never>, HttpError> {
    let result = state
        .user_service
        .sign_in(&mut session, &payload.email)
        .await
        .inspect_err(|e| error!("Error signing in user: {:#?}", e));

    match result {
        Ok(_) => Ok(HttpSuccess::HxRedirect("/todos".to_string())),
        Err(e) => {
            error!("Error signing in user: {:#?}", e);
            Err(e.into())
        }
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
) -> Result<HttpSuccess<Never>, HttpError> {
    let result = state
        .user_service
        .sign_up(&mut session, &payload.email)
        .await
        .inspect_err(|e| error!("Error signing in user: {:#?}", e));

    match result {
        Ok(_) => Ok(HttpSuccess::HxRedirect("/todos".into())),
        Err(e) => Err(e.into()),
    }
}

pub async fn post_sign_out(session: Session) -> Result<HttpSuccess<Never>, HttpError> {
    session.delete().await?;
    session.save().await?;
    Ok(HttpSuccess::HxRedirect("/users".into()))
}
