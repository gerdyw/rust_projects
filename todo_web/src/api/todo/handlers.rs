use crate::api::todo::components::{TodoList, TodoPage};
use crate::api::todo::models::CreateTodo;
use crate::domain::appstate::AppState;
use crate::domain::components::{HtmlComponent, IntoHtmlComponent};
use crate::domain::errors::InternalServerError;
use crate::domain::models::AuthedResult;
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
) -> Result<AuthedResult<HtmlComponent<TodoPage>>, InternalServerError> {
    let user_id = extract_user_id(session).await?;
    let Some(user_id) = user_id else {
        return Ok(AuthedResult::NotAuthed);
    };

    let todos = state.todo_service.list_for_user(user_id).await?;
    let email = state.user_service.get_email_by_id(user_id).await?;
    let Some(email) = email else {
        return Ok(AuthedResult::NotAuthed);
    };

    let component = TodoPage::new(todos, email);
    Ok(AuthedResult::Authed(component.into_html_component()))
}

pub async fn list_todos(
    State(state): State<AppState>,
    session: Session,
) -> Result<AuthedResult<HtmlComponent<TodoList>>, InternalServerError> {
    let user_id = extract_user_id(session).await?;

    match user_id {
        Some(user_id) => {
            let todos = state.todo_service.list_for_user(user_id).await?;
            let component = TodoList(todos);
            Ok(AuthedResult::Authed(component.into_html_component()))
        }
        None => Ok(AuthedResult::NotAuthed),
    }
}

pub async fn create_todo(
    state: State<AppState>,
    session: Session,
    Form(payload): Form<CreateTodo>,
) -> Result<AuthedResult<HtmlComponent<TodoList>>, InternalServerError> {
    let user_id = extract_user_id(session).await?;

    match user_id {
        Some(user_id) => {
            let _ = state.todo_service.create(user_id, payload.title).await?;
            let todos = state.todo_service.list_for_user(user_id).await?;
            let component = TodoList(todos);
            Ok(AuthedResult::Authed(component.into_html_component()))
        }
        None => Ok(AuthedResult::NotAuthed),
    }
}

pub async fn mark_done(
    state: State<AppState>,
    session: Session,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let _ = state
        .todo_service
        .mark_done(id)
        .await
        .inspect_err(|e| error!("{:#?}", e));

    list_todos(state, session).await
}

pub async fn mark_undone(
    state: State<AppState>,
    session: Session,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let _ = state
        .todo_service
        .mark_undone(id)
        .await
        .inspect_err(|e| error!("{:#?}", e));

    list_todos(state, session).await
}

pub async fn delete_todo(
    state: State<AppState>,
    session: Session,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let _ = state
        .todo_service
        .delete(id)
        .await
        .inspect_err(|e| error!("{:#?}", e));
    list_todos(state, session).await
}
async fn extract_user_id(session: Session) -> Result<Option<Uuid>, InternalServerError> {
    let user_id = session.get::<Uuid>("user").await.map_err(|e| {
        error!("Failed to get user session: {}", e);
        InternalServerError {
            message: "Failed to get user session".into(),
        }
    })?;
    Ok(user_id)
}
