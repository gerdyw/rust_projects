use crate::api::todo::repository::TodoRepository;

#[derive(Clone)]
pub struct AppState {
    pub todo_repo: TodoRepository,
}

impl AppState {
    pub fn new(todo_repo: TodoRepository) -> Self {
        Self { todo_repo }
    }
}
