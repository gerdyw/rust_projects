use axum::{
    Form,
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;
use tower_sessions::Session;
use tracing::error;

use crate::{
    api::user::{
        components::{SigninForm, SignupForm, UserPage},
        models::{ErrorQuery, SignupUser},
    },
    domain::{
        appstate::AppState,
        components::IntoHtmlComponent,
        errors::ServiceError,
        models::{HttpResponse, Never},
    },
};

#[derive(Debug, Clone, Deserialize)]
pub struct LoginQuery {
    pub user_email: Option<String>,
}

pub async fn get_index(
    state: State<AppState>,
    mut session: Session,
    Query(query): Query<LoginQuery>,
) -> HttpResponse<UserPage> {
    let Some(email) = query.user_email else {
        // No email provided: show login form
        return HttpResponse::Html(UserPage::sign_in(None).into_html_component());
    };

    match state.user_service.sign_in(&mut session, &email).await {
        Ok(_) => HttpResponse::Redirect("/todos".to_string()),
        Err(ServiceError::NotFound) => {
            HttpResponse::Html(UserPage::sign_up(None).into_html_component())
        }
        Err(_) => HttpResponse::Html(UserPage::sign_up(None).into_html_component()),
    }
}

pub async fn get_sign_in(query: Query<ErrorQuery>) -> impl IntoResponse {
    SigninForm(query.error.clone()).into_html_component()
}

pub async fn post_sign_in(
    state: State<AppState>,
    mut session: Session,
    Form(payload): Form<SignupUser>,
) -> HttpResponse<Never> {
    let result = state
        .user_service
        .sign_in(&mut session, &payload.email)
        .await
        .inspect_err(|e| error!("Error signing in user: {:#?}", e));

    match result {
        Ok(_) => HttpResponse::HxRedirect("/todos".to_string()),
        Err(ServiceError::NotFound) => {
            HttpResponse::NotFound(format!("User with email {} not found", payload.email))
        }
        Err(e) => {
            error!("Error signing in user: {:#?}", e);
            HttpResponse::InternalServerError
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
) -> HttpResponse<Never> {
    let result = state
        .user_service
        .sign_up(&mut session, &payload.email)
        .await
        .inspect_err(|e| error!("Error signing in user: {:#?}", e));

    match result {
        Ok(_) => HttpResponse::HxRedirect("/todos".into()),
        Err(ServiceError::Conflict) => HttpResponse::Conflict(format!(
            "User {} already exists, try signing in",
            payload.email
        )),
        Err(_) => HttpResponse::InternalServerError,
    }
}

pub async fn post_sign_out(session: Session) -> HttpResponse<Never> {
    let delete_result = session.delete().await;
    let save_result = session.save().await;
    match (delete_result, save_result) {
        (Ok(_), Ok(_)) => HttpResponse::HxRedirect("/users".into()),
        _ => HttpResponse::InternalServerError,
    }
}
