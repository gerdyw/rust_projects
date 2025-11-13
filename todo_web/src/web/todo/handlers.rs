use crate::data::todo::models::CreateTodo;
use crate::domain::appstate::AppState;
use crate::domain::components::IntoHtmlComponent;
use crate::domain::models::{HttpError, HttpSuccess};
use crate::web::todo::components::{TodoList, TodoPage};
use axum::{
    Form,
    extract::{Path, State},
};
use tower_sessions::Session;
use uuid::Uuid;

pub async fn get_index(
    State(state): State<AppState>,
    session: Session,
) -> Result<HttpSuccess<TodoPage>, HttpError> {
    let user_id = extract_user_id(session)
        .await?
        .ok_or(HttpError::UnauthorizedRedirect)?;

    let todos = state
        .todo_service
        .list_for_user(user_id)
        .await
        .unwrap_or_default();

    let email = state.user_service.get_email_by_id(user_id).await.ok();

    let component = TodoPage::new(todos, email);
    Ok(HttpSuccess::Html(component.into_html_component()))
}

pub async fn list_todos(
    State(state): State<AppState>,
    session: Session,
) -> Result<HttpSuccess<TodoList>, HttpError> {
    let user_id = extract_user_id(session)
        .await?
        .ok_or(HttpError::UnauthorizedRedirect)?;

    let todos = state.todo_service.list_for_user(user_id).await?;

    Ok(HttpSuccess::Html(TodoList(todos).into_html_component()))
}

pub async fn create_todo(
    state: State<AppState>,
    session: Session,
    Form(payload): Form<CreateTodo>,
) -> Result<HttpSuccess<TodoList>, HttpError> {
    let user_id = extract_user_id(session)
        .await?
        .ok_or(HttpError::UnauthorizedRedirect)?;

    let todos = state.todo_service.create(user_id, payload.title).await?;

    Ok(HttpSuccess::Html(TodoList(todos).into_html_component()))
}

pub async fn mark_done(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<Uuid>,
) -> Result<HttpSuccess<TodoList>, HttpError> {
    let user_id = extract_user_id(session)
        .await?
        .ok_or(HttpError::UnauthorizedRedirect)?;

    let todos = state.todo_service.mark_done(user_id, id).await?;

    Ok(HttpSuccess::Html(TodoList(todos).into_html_component()))
}

pub async fn mark_undone(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<Uuid>,
) -> Result<HttpSuccess<TodoList>, HttpError> {
    let user_id = extract_user_id(session)
        .await?
        .ok_or(HttpError::UnauthorizedRedirect)?;

    let todos = state.todo_service.mark_undone(user_id, id).await?;

    Ok(HttpSuccess::Html(TodoList(todos).into_html_component()))
}

pub async fn delete_todo(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<Uuid>,
) -> Result<HttpSuccess<TodoList>, HttpError> {
    let user_id = extract_user_id(session)
        .await?
        .ok_or(HttpError::UnauthorizedRedirect)?;

    let todos = state.todo_service.delete(user_id, id).await?;

    Ok(HttpSuccess::Html(TodoList(todos).into_html_component()))
}

async fn extract_user_id(session: Session) -> Result<Option<Uuid>, HttpError> {
    session.get::<Uuid>("user").await.map_err(|e| e.into())
}
