use crate::db::repository::TodoRepository;

#[derive(Clone)]
pub struct AppState {
    pub repo: TodoRepository,
}
