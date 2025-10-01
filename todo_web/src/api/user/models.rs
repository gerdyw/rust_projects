use crate::db::metadata::Metadata;
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    #[sqlx(flatten)]
    pub meta: Metadata,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignupUser {
    pub email: String,
}
