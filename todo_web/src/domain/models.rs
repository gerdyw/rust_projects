use axum::response::{IntoResponse, Redirect};

// Add this struct to store session data
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct UserSession {
    pub user_id: uuid::Uuid,
}

pub enum AuthedResult<T> {
    Authed(T),
    NotAuthed,
}
impl<T> IntoResponse for AuthedResult<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            AuthedResult::Authed(data) => data.into_response(),
            AuthedResult::NotAuthed => Redirect::to("/login").into_response(),
        }
    }
}
