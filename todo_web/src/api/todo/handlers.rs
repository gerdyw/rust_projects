use crate::api::todo::components::{TodoList, TodoPage};
use crate::api::todo::models::CreateTodo;
use crate::domain::appstate::AppState;
use crate::domain::components::IntoHtmlComponent;
use crate::domain::errors::HttpError;
use axum::response::Response;
use axum::{
    Form,
    extract::{Path, State},
    response::IntoResponse,
};
use tower_sessions::Session;
use tracing::error;
use uuid::Uuid;

pub async fn get_index(
    State(state): State<AppState>,
    session: Session,
) -> Result<Response, HttpError> {
    let user_id = extract_user_id(session).await?;
    let todos = state.todo_service.list_for_user(&user_id).await?;
    let email = state.user_service.get_email_by_id(&user_id).await?;

    let Some(email) = email else {
        return Err(HttpError::InternalServerError("Failed to get email".into()));
    };
    let component = TodoPage::new(todos, email).into_html_component();
    Ok(component.into_response())
}

pub async fn list_todos(
    State(state): State<AppState>,
    session: Session,
) -> Result<Response, HttpError> {
    let user_id = extract_user_id(session).await?;
    return_todos(&state, &user_id).await
}

pub async fn create_todo(
    state: State<AppState>,
    session: Session,
    Form(payload): Form<CreateTodo>,
) -> Result<Response, HttpError> {
    let user_id = extract_user_id(session).await?;
    let _ = state.todo_service.create(&user_id, payload.title).await?;
    return_todos(&state, &user_id).await
}

pub async fn mark_done(
    state: State<AppState>,
    session: Session,
    Path(id): Path<Uuid>,
) -> Result<Response, HttpError> {
    let user_id = extract_user_id(session).await?;
    let _ = state.todo_service.mark_done(&id).await?;
    return_todos(&state, &user_id).await
}

pub async fn mark_undone(
    state: State<AppState>,
    session: Session,
    Path(id): Path<Uuid>,
) -> Result<Response, HttpError> {
    let user_id = extract_user_id(session).await?;
    let _ = state.todo_service.mark_undone(&id).await?;
    return_todos(&state, &user_id).await
}

pub async fn delete_todo(
    state: State<AppState>,
    session: Session,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let _ = state
        .todo_service
        .delete(&id)
        .await
        .inspect_err(|e| error!("{:#?}", e))?;
    list_todos(state, session).await
}

async fn extract_user_id(session: Session) -> Result<Uuid, HttpError> {
    let user_id = session
        .get::<Uuid>("user")
        .await
        .map_err(|e| HttpError::InternalServerError(e.to_string()))?
        .ok_or_else(|| HttpError::Unauthorized)?;
    Ok(user_id)
}

async fn return_todos(state: &AppState, user_id: &Uuid) -> Result<Response, HttpError> {
    let todos = state.todo_service.list_for_user(&user_id).await?;
    let component = TodoList(todos).into_html_component();
    Ok(component.into_response())
}
