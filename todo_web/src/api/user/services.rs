use tower_sessions::Session;
use uuid::Uuid;

use crate::{
    api::user::{models::User, repository::UserRepository},
    domain::errors::InternalServerError,
};

const USER_KEY: &str = "user";

#[derive(Clone)]
pub struct UserService {
    pub repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn get_email_by_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<String>, InternalServerError> {
        let user = self.repo.find_user_by_id(user_id).await.map_err(|e| {
            tracing::error!("Error finding user by id: {}", e);
            InternalServerError::from(e)
        })?;
        Ok(user.map(|u| u.email))
    }

    pub async fn sign_in(
        &self,
        session: &mut Session,
        email: &String,
    ) -> Result<Option<User>, InternalServerError> {
        let user = self.repo.find_user_by_email(email).await.map_err(|e| {
            tracing::error!("Error finding user by email: {}", e);
            InternalServerError::from(e)
        })?;

        if let Some(user) = user {
            session.insert(USER_KEY, user.meta.id).await.map_err(|e| {
                tracing::error!("Error inserting session: {}", e);
                InternalServerError::from(e)
            })?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    pub async fn sign_up(
        &self,
        session: &mut Session,
        email: &String,
    ) -> Result<User, InternalServerError> {
        let user = self.repo.create_user(email).await.map_err(|e| {
            tracing::error!("Error creating user: {}", e);
            InternalServerError::from(e)
        })?;
        session.insert(USER_KEY, user.meta.id).await.map_err(|e| {
            tracing::error!("Error inserting session: {}", e);
            InternalServerError::from(e)
        })?;
        Ok(user)
    }
}
