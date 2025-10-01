use axum::{Form, extract::State, response::IntoResponse};

use crate::{
    api::user::{
        components::{UserComponent, UserPage},
        models::SignupUser,
    },
    domain::{appstate::AppState, components::IntoHtmlComponent},
};

pub async fn get_index() -> impl IntoResponse {
    UserPage {}.into_html_component()
}

pub async fn signup_user(
    state: State<AppState>,
    Form(payload): Form<SignupUser>,
) -> impl IntoResponse {
    tracing::info!("Signup attempt for email: {}", payload.email);

    // Early return on database error
    let maybe_user = match state.user_repo.find_user_by_email(&payload.email).await {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("Database error: {}", e);
            return UserComponent(Err("Database error".into())).into_html_component();
        }
    };

    // Early return if user exists
    if let Some(user) = maybe_user {
        return UserComponent(Ok(user)).into_html_component();
    }

    // Create new user since they don't exist
    let result = state
        .user_repo
        .create_user(&payload.email)
        .await
        .map_err(|e| format!("Failed to create user: {}", e));

    UserComponent(result).into_html_component()
}
