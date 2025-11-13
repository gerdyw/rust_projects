use serde::Deserialize;
use sqlx::FromRow;

use crate::db::metadata::Metadata;

#[derive(Debug, FromRow)]
pub struct Todo {
    #[sqlx(flatten)]
    pub meta: Metadata,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}
