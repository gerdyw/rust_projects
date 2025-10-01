use crate::api::{todo::repository::TodoRepository, user::repository::UserRepository};

#[derive(Clone)]
pub struct AppState {
    pub todo_repo: TodoRepository,
    pub user_repo: UserRepository,
}

impl AppState {
    pub fn new(todo_repo: TodoRepository, user_repo: UserRepository) -> Self {
        Self {
            todo_repo,
            user_repo,
        }
    }
}
