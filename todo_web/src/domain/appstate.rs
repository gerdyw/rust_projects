use sqlx::{Pool, Postgres};

use crate::api::{
    todo::{repository::TodoRepository, services::TodoService},
    user::{repository::UserRepository, services::UserService},
};

#[derive(Clone)]
pub struct AppState {
    pub todo_service: TodoService,
    pub user_service: UserService,
}

impl AppState {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        let todo_repo = TodoRepository::new(pool.clone());
        let todo_service = TodoService::new(todo_repo);
        let user_repo = UserRepository::new(pool.clone());
        let user_service = UserService::new(user_repo);
        Self {
            todo_service,
            user_service,
        }
    }
}
