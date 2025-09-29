use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}
