use crate::api::todo::components::{TodoList, TodoPage};
use crate::api::todo::models::CreateTodo;
use crate::domain::appstate::AppState;
use crate::domain::components::IntoHtmlComponent;
use crate::domain::errors::ServiceError;
use crate::domain::models::HttpResponse;
use axum::{
    Form,
    extract::{Path, State},
    response::IntoResponse,
};
use tower_sessions::Session;
use uuid::Uuid;

pub async fn get_index(State(state): State<AppState>, session: Session) -> HttpResponse<TodoPage> {
    let user_id = extract_user_id(session).await;

    let user_id = match user_id {
        Some(user_id) => user_id,
        None => return HttpResponse::UnauthorizedRedirect,
    };

    let todos = state
        .todo_service
        .list_for_user(user_id)
        .await
        .unwrap_or_default();

    let email = state.user_service.get_email_by_id(user_id).await.ok();

    let component = TodoPage::new(todos, email);
    HttpResponse::Html(component.into_html_component())
}

pub async fn list_todos(State(state): State<AppState>, session: Session) -> HttpResponse<TodoList> {
    let user_id = extract_user_id(session).await;

    let user_id = match user_id {
        Some(user_id) => user_id,
        None => return HttpResponse::UnauthorizedRedirect,
    };

    let todos = state.todo_service.list_for_user(user_id).await;

    match todos {
        Ok(todos) => HttpResponse::Html(TodoList(todos).into_html_component()),
        Err(ServiceError::NotFound) => HttpResponse::NotFound("Todos not found".to_string()),
        Err(ServiceError::BadRequest) => HttpResponse::BadRequest("Bad request".to_string()),
        Err(_) => HttpResponse::InternalServerError,
    }
}

pub async fn create_todo(
    state: State<AppState>,
    session: Session,
    Form(payload): Form<CreateTodo>,
) -> HttpResponse<TodoList> {
    let user_id = match extract_user_id(session).await {
        Some(user_id) => user_id,
        None => return HttpResponse::UnauthorizedRedirect,
    };

    let todos = state.todo_service.create(user_id, payload.title).await;

    match todos {
        Ok(todos) => HttpResponse::Html(TodoList(todos).into_html_component()),
        Err(ServiceError::NotFound) => HttpResponse::NotFound("Todos not found".to_string()),
        Err(ServiceError::BadRequest) => HttpResponse::BadRequest("Bad request".to_string()),
        Err(_) => HttpResponse::InternalServerError,
    }
}

pub async fn mark_done(state: State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let todos = state.todo_service.mark_done(id).await;

    match todos {
        Ok(todos) => HttpResponse::Html(TodoList(todos).into_html_component()),
        Err(ServiceError::NotFound) => HttpResponse::NotFound("Todos not found".to_string()),
        Err(ServiceError::BadRequest) => HttpResponse::BadRequest("Bad request".to_string()),
        Err(_) => HttpResponse::InternalServerError,
    }
}

pub async fn mark_undone(state: State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let todos = state.todo_service.mark_undone(id).await;

    match todos {
        Ok(todos) => HttpResponse::Html(TodoList(todos).into_html_component()),
        Err(ServiceError::NotFound) => HttpResponse::NotFound("Todos not found".to_string()),
        Err(ServiceError::BadRequest) => HttpResponse::BadRequest("Bad request".to_string()),
        Err(_) => HttpResponse::InternalServerError,
    }
}

pub async fn delete_todo(state: State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let todos = state.todo_service.delete(id).await;

    match todos {
        Ok(todos) => HttpResponse::Html(TodoList(todos).into_html_component()),
        Err(ServiceError::NotFound) => HttpResponse::NotFound("Todos not found".to_string()),
        Err(ServiceError::BadRequest) => HttpResponse::BadRequest("Bad request".to_string()),
        Err(_) => HttpResponse::InternalServerError,
    }
}

async fn extract_user_id(session: Session) -> Option<Uuid> {
    session.get::<Uuid>("user").await.unwrap_or(None)
}
